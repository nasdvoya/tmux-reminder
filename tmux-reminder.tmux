#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

tmux set-option -gq @tmux_reminder_interval "5"
tmux set-option -gq @tmux_reminder_file "$CURRENT_DIR/flake.nix"

tmux bind-key q run-shell "$CURRENT_DIR/target/release/tmux_reminder"
