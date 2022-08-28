use tree_sitter;
use colored::*;
use thiserror::Error;
use std::path::Path;
use std::io;
use std::io::Read;
use std::io::Write;
use atty;

pub enum WalkerChoice {
    GotoChild,
    GotoSibling,
    GotoParentSibling,
    Exit
}

#[derive(Error,Debug)]
pub enum LanguageError {
    #[error("Syntax error")]
    Syntax
}

pub trait Visit {
    fn visit(&mut self,curs:&tree_sitter::TreeCursor) -> WalkerChoice;
    fn walk(&mut self,tree: tree_sitter::Tree)
    {
        let mut curs = tree.walk();
        let mut choice = WalkerChoice::GotoChild;
        while ! matches!(choice,WalkerChoice::Exit)
        {
            if matches!(choice,WalkerChoice::GotoChild) && curs.goto_first_child() {
                choice = self.visit(&curs);
            } else if matches!(choice,WalkerChoice::GotoParentSibling) && curs.goto_parent() && curs.goto_next_sibling() {
                choice = self.visit(&curs);
            } else if matches!(choice,WalkerChoice::GotoSibling) && curs.goto_next_sibling() {
                choice = self.visit(&curs);
            } else if curs.goto_next_sibling() {
                choice = self.visit(&curs);
            } else if curs.goto_parent() {
                choice = WalkerChoice::GotoSibling;
            } else {
                choice = WalkerChoice::Exit;
            }
        }
    }
}

pub struct SyntaxCheckVisitor {
    pub code: String,
    pub err_count: usize
}

impl SyntaxCheckVisitor {
    fn new(prog: String) -> Self {
        Self { code: prog, err_count: 0 }
    }
}

impl Visit for SyntaxCheckVisitor {
    fn visit(&mut self,curs:&tree_sitter::TreeCursor) -> WalkerChoice
    {
        if curs.node().is_error()
        {
            self.err_count += 1;
            let mut c = curs.clone();
            let b1 = c.node().start_byte();
            let b2 = c.node().end_byte();
            let mut depth = 0;
            let mut indent = String::from("");
            while c.goto_parent() {
                depth += 1;
                indent += "  ";
            }
            let mess = match depth {
                2 => String::from("ERROR on line"),
                3 => String::from("ERROR in statement"),
                _ => String::from("ERROR within statement")
            };
            eprintln!("{}{} {} {}",indent,mess.red(),self.code.get(b1..b2).expect("none").yellow().bold(),curs.node().to_sexp());
        }
        return WalkerChoice::GotoChild;
    }
}

/// detect syntax errors in any language.  This is meant to be pipelined,
/// i.e., it will return either the source code or an error.  If an error
/// is returned the caller may choose to panic to stop the pipeline.
pub fn verify_stdin(lang: tree_sitter::Language,prompt: &str) -> Result<String,LanguageError>
{
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(lang).expect("Error loading grammar");
    let mut visitor = SyntaxCheckVisitor::new(String::new());
    let mut code = String::new();
    if atty::is(atty::Stream::Stdin)
    {
        eprintln!("Line entry interface.");
        eprintln!("This is a blind accumulation of lines.");
        eprintln!("Verify occurs when entry is terminated.");
        eprintln!("Accumulated lines can be piped.");
        eprintln!("`bye` terminates.");
        loop {
            eprint!("{} ",prompt);
            let mut line = String::new();
            io::stderr().flush().expect("could not flush stderr");
            io::stdin().read_line(&mut line).expect("could not read stdin");
            if line=="bye\n" {
                break;
            }
            code += &line;
        }
    }
    else
    {
        io::stdin().read_to_string(&mut code).expect("could not read stdin");
    }
    let mut iter = code.lines();
    while let Some(line) = iter.next()
    {
        let tree = parser.parse(line.clone(),None).expect("Error parsing file");
        visitor.code = String::from(line);
        visitor.walk(tree);
    }
    eprintln!("There were {} errors in stdin",visitor.err_count);
    if visitor.err_count==0 {
        return Ok(code);
    } else {
        return Err(LanguageError::Syntax);
    }
}