#!/bin/bash

# set -eux

LOGIC_ADDRESS=""
PROXY_ADDRESS=""

cargo contract build --manifest-path logic/Cargo.toml
cargo contract build --manifest-path proxy/Cargo.toml


LOGIC_ADDRESS=$(cargo contract instantiate --constructor new \
    --suri //Alice --salt $(date +%s) \
    --manifest-path logic/Cargo.toml \
    --output-json --skip-confirm | jq .contract -r)

echo "Logic Address: $LOGIC_ADDRESS"

PROXY_ADDRESS=$(cargo contract instantiate --constructor new \
    --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
    --args $LOGIC_ADDRESS \
    --suri //Alice --salt $(date +%s) --manifest-path proxy/Cargo.toml \
    --output-json --skip-confirm | jq .contract -r)

echo "Proxy Address: $PROXY_ADDRESS"

cargo contract call --contract $PROXY_ADDRESS \
    --message get -s //Alice \
    target/ink/proxy/metadata.json --dry-run --skip-confirm

sleep 1

cargo contract call --contract $LOGIC_ADDRESS \
    --message flip -s //Bob \
    target/ink/logic/metadata.json --skip-confirm

sleep 1

cargo contract call --contract $PROXY_ADDRESS \
    --message get -s //Alice \
    target/ink/proxy/metadata.json --dry-run --skip-confirm
