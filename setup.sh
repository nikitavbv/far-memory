curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
apt update
DEBIAN_FRONTEND=noninteractive apt install -y build-essential pkg-config libssl-dev libfontconfig-dev
cargo build --release
