#!/bin/bash

# Paths to the completion directories
ZSH_DIR="${ZDOTDIR:-$HOME}/.zsh/completions"
BASH_DIR="/etc/bash_completion.d"
ELVISH_DIR="$HOME/.config/elvish/lib"
FISH_DIR="$HOME/.config/fish/completions"

install_completions() {
    mkdir -pv "$ZSH_DIR" "$ELVISH_DIR" "$FISH_DIR"

    # Copy completions
    cp ./completions/_a2kit "$ZSH_DIR/"
    cp ./completions/a2kit.bash "$BASH_DIR/"
    cp ./completions/a2kit.elv "$ELVISH_DIR/"
    cp ./completions/a2kit.fish "$FISH_DIR/"

    echo "Completions installed"
}

remove_completions() {
    rm -f "$ZSH_DIR/_a2kit" \
          "$BASH_DIR/a2kit.bash" \
          "$ELVISH_DIR/a2kit.elv" \
          "$FISH_DIR/a2kit.fish" \
          || true
}

case "$1" in
    install) install_completions ;;
    remove) remove_completions ;;
    *) echo "Usage: $0 {install|remove}" ;;
esac
