//! Module for handling random access text

use std::fmt;
use std::str::FromStr;
use std::collections::HashMap;
use super::{FileImage,Records,TextConversion,Error};
use crate::{STDRESULT,DYNERR};

impl Records {
    pub fn new(record_len: usize) -> Self {
        Self {
            record_len,
            map: HashMap::new()
        }
    }
    /// add a string as record number `num`, fields should be separated by LF or CRLF.
    pub fn add_record(&mut self,num: usize,fields: &str) {
        self.map.insert(num,fields.to_string());
    }
    /// Derive records from file image, this should find any real record, but may also find spurious ones.
    /// This is due to fundamental non-invertibility of the A2 file system's random access storage pattern.
    /// This routine assumes ASCII null terminates any record.
    pub fn from_fimg(fimg: &FileImage,record_length: usize,converter: impl TextConversion) -> Result<Records,DYNERR> {
        if record_length==0 {
            return Err(Box::new(Error::FileFormat));
        }
        let mut ans = Records::new(record_length);
        let mut list: Vec<usize> = Vec::new();
        // add record index for each starting record boundary that falls within a chunk
        let chunk_len = fimg.chunk_len;
        for c in fimg.chunks.keys() {
            let start_rec = c*chunk_len/record_length + match c*chunk_len%record_length { x if x>0 => 1, _ => 0 };
            let end_rec = (c+1)*chunk_len/record_length + match (c+1)*chunk_len%record_length { x if x>0 => 1, _ => 0 };
            for r in start_rec..end_rec {
                list.push(r);
            }
        }
        // add only records with complete data
        for r in list {
            let start_chunk = r*record_length/chunk_len;
            let end_chunk = 1 + (r+1)*record_length/chunk_len;
            let start_offset = r*record_length%chunk_len;
            let mut bytes: Vec<u8> = Vec::new();
            let mut complete = true;
            for chunk_num in start_chunk..end_chunk {
                match fimg.chunks.get(&chunk_num) {
                    Some(chunk) => {
                       for i in chunk {
                            bytes.push(*i);
                        }
                    },
                    _ => complete = false
                }
            }
            if complete && start_offset < bytes.len() {
                let actual_end = usize::min(start_offset+record_length,bytes.len());
                if let Some(long_str) = converter.to_utf8(&bytes[start_offset..actual_end].to_vec()) {
                    if let Some(partial) = long_str.split("\u{0000}").next() {
                        if partial.len()>0 {
                            ans.map.insert(r,partial.to_string());
                        }
                    } else {
                        if long_str.len()>0 {
                            ans.map.insert(r,long_str);
                        }
                    }
                }
            }
        }
        return Ok(ans);
    }
    /// Update a file image's data using the records, this is usually done before writing to a disk image.
    /// This will set the file image's eof, but no other metadata.
    pub fn update_fimg(&self,ans: &mut FileImage,require_first: bool,converter: impl TextConversion,clear: bool) -> STDRESULT {
        if clear {
            ans.chunks = HashMap::new();
        }
        let chunk_len = ans.chunk_len;
        let mut eof: usize = 0;
        // always need to have the first chunk referenced on ProDOS
        if require_first {
            ans.chunks.insert(0,vec![0;chunk_len]);
        }
        // now insert the actual records, first chunk can always be overwritten
        for (rec_num,fields) in &self.map {
            match converter.from_utf8(fields) {
                Some(data_bytes) => {
                    let logical_chunk = self.record_len * rec_num / chunk_len;
                    let end_logical_chunk = 1 + (self.record_len * (rec_num+1) - 1) / chunk_len;
                    let fwd_offset = self.record_len * rec_num % chunk_len;
                    for lb in logical_chunk..end_logical_chunk {
                        let start_byte = match lb {
                            l if l==logical_chunk => fwd_offset,
                            _ => 0
                        };
                        let end_byte = match lb {
                            l if l==end_logical_chunk-1 => fwd_offset + data_bytes.len() - chunk_len*(end_logical_chunk-logical_chunk-1),
                            _ => chunk_len
                        };
                        let mut buf = match ans.chunks.contains_key(&lb) {
                            true => ans.chunks.get(&lb).unwrap().clone(),
                            false => Vec::new()
                        };
                        // extend only to the end of data
                        for _i in buf.len()..end_byte as usize {
                            buf.push(0);
                        }
                        // load the part of the chunk with the data
                        for i in start_byte..end_byte {
                            buf[i as usize] = data_bytes[chunk_len*(lb-logical_chunk) + i - fwd_offset];
                        }
                        eof = usize::max(lb*512 + buf.len(),eof);
                        ans.chunks.insert(lb as usize,buf);
                    }
                },
                None => return Err(Box::new(std::fmt::Error))
            }
        }
        ans.set_eof(eof);
        return Ok(());
    }
    /// Get records from the JSON string representation
    pub fn from_json(json_str: &str) -> Result<Records,DYNERR> {
        match json::parse(json_str) {
            Ok(parsed) => {
                let maybe_type = parsed["fimg_type"].as_str();
                let maybe_len = parsed["record_length"].as_usize();
                if let (Some(typ),Some(len)) = (maybe_type,maybe_len) {
                    if typ=="rec" {
                        let mut records: HashMap<usize,String> = HashMap::new();
                        let map_obj = &parsed["records"];
                        if map_obj.entries().len()==0 {
                            log::error!("no object entries in json records");
                            return Err(Box::new(Error::FileImageFormat));
                        }
                        for (key,lines) in map_obj.entries() {
                            if let Ok(num) = usize::from_str(key) {
                                let mut fields = String::new();
                                for maybe_field in lines.members() {
                                    if let Some(line) = maybe_field.as_str() {
                                        fields = fields + line + "\n";
                                    } else {
                                        log::error!("record is not a string");
                                        return Err(Box::new(Error::FileImageFormat));
                                    }
                                }
                                records.insert(num,fields);
                            } else {
                                log::error!("key is not a number");
                                return Err(Box::new(Error::FileImageFormat));
                            }
                        }
                        return Ok(Self {
                            record_len: len,
                            map: records
                        });    
                    } else {
                        log::error!("json metadata type mismatch");
                        return Err(Box::new(Error::FileImageFormat));
                    }
                }
                log::error!("json records missing metadata");
                Err(Box::new(Error::FileImageFormat))
            },
            Err(_e) => Err(Box::new(Error::FileImageFormat))
        } 
    }
    /// Put records into the JSON string representation, if indent=0 use unpretty form
    pub fn to_json(&self,indent: Option<u16>) -> String {
        let mut json_map = json::JsonValue::new_object();
        for (r,l) in &self.map {
            let mut json_array = json::JsonValue::new_array();
            for line in l.lines() {
                json_array.push(line).expect("error while building JSON array");
            }
            json_map[r.to_string()] = json_array;
        }
        let ans = json::object! {
            fimg_type: "rec",
            record_length: self.record_len,
            records: json_map
        };
        if let Some(spaces) = indent {
            return json::stringify_pretty(ans, spaces);
        } else {
            return json::stringify(ans);
        }
    }
}

/// Allows the records to be displayed to the console using `println!`.  This also
/// derives `to_string`, so the structure can be converted to `String`.
impl fmt::Display for Records {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx,fields) in &self.map {
            write!(f,"Record {}",idx).expect("format error");
            for field in fields.lines() {
                write!(f,"    {}",field).expect("format error");
            }
        }
        write!(f,"Record Count = {}",self.map.len())
    }
}
