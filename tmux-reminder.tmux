#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if [ ! -f "$CURRENT_DIR/target/release/tmux_reminder" ]; then
    (cd "$CURRENT_DIR" && cargo build --release)
fi

# PLugin options
tmux set-option -gq @tmux_reminder_interval "5"
tmux set-option -gq @tmux_reminder_file "$CURRENT_DIR/Cargo.toml"

# Run the Rust program periodically to update the status
tmux set-option -g status-interval 1
tmux set-option -g status-right "#($CURRENT_DIR/target/release/tmux_reminder)"
