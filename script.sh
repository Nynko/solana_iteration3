#!/bin/bash


# Exit if any command fails
set -e


# Remove current idl
rm /Users/nicolasbeaudouin/Documents/Project1/asset_based/target/deploy/asset_based-keypair.json

# Build with anchor
anchor build

# Generate new pubkey
new_id=$(solana-keygen pubkey /Users/nicolasbeaudouin/Documents/Project1/asset_based/target/deploy/asset_based-keypair.json)

# Replace in lib.rs
sed -i '' "s/declare_id!(\".*\")/declare_id!(\"$new_id\")/g" /Users/nicolasbeaudouin/Documents/Project1/asset_based/programs/asset_based/src/lib.rs


# Define the path to your config file
config_file="/Users/nicolasbeaudouin/Documents/Project1/asset_based/Anchor.toml"

# Replace in config.toml for localnet
awk -v id="$new_id" '
    /^\[programs\.localnet\]$/ { f = 1 }
    f && /asset_based =/ { sub(/=.*/, "= \"" id "\""); f = 0 }
    1' "$config_file" > tmp && mv tmp "$config_file"

# Replace in config.toml for devnet
awk -v id="$new_id" '
    /^\[programs\.devnet\]$/ { f = 1 }
    f && /asset_based =/ { sub(/=.*/, "= \"" id "\""); f = 0 }
    1' "$config_file" > tmp && mv tmp "$config_file"