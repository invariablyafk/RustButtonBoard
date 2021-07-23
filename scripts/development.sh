#!/bin/bash
# Must run with sudo, ie as root user

apt install -y libasound2-dev libudev-dev samba
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo run
