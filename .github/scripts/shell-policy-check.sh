#!/bin/sh
# Run src/shell_policy.rs tests without compiling the full client.
# The stub `base` crate only supplies the option-key strings. It is not the
# workspace `base` crate, and a passing run does not build the desktop app.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

mkdir -p "$work/src" "$work/base/src"
cat > "$work/Cargo.toml" << EOF
[package]
name = "shell_policy_test"
version = "0.1.0"
edition = "2021"

[dependencies]
base = { path = "base" }
serde_json = "1.0"
EOF
cat > "$work/base/Cargo.toml" << EOF
[package]
name = "base"
version = "0.1.0"
edition = "2021"
EOF
cat > "$work/base/src/lib.rs" << 'EOF'
pub mod config {
    pub mod keys {
        pub const OPTION_CUSTOM_RENDEZVOUS_SERVER: &str = "custom-rendezvous-server";
        pub const OPTION_RELAY_SERVER: &str = "relay-server";
        pub const OPTION_API_SERVER: &str = "api-server";
        pub const OPTION_KEY: &str = "key";
        pub const OPTION_HIDE_SERVER_SETTINGS: &str = "hide-server-settings";
        pub const OPTION_HIDE_HELP_CARDS: &str = "hide-help-cards";
        pub const OPTION_DISABLE_DISCOVERY_PANEL: &str = "disable-discovery-panel";
        pub const OPTION_DISABLE_GROUP_PANEL: &str = "disable-group-panel";
        pub const OPTION_CODEC_PREFERENCE: &str = "codec-preference";
    }
}
EOF
cat > "$work/src/lib.rs" << EOF
#[path = "$root/src/shell_policy.rs"]
mod shell_policy;
EOF
cd "$work"
cargo test --manifest-path "$work/Cargo.toml"
