curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
$HOME/.cargo/bin/cargo doc --document-private-items
cp ./.cloudflare/index.html ./target/doc/