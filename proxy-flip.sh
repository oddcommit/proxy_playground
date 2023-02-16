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

sleep 1

# These next couple of messages should be executed by the `Proxy` contract since the
# admin, `Alice`, is sending the messages.
cargo contract call --contract $PROXY_ADDRESS \
    --message set --args 12 -s //Alice \
    target/ink/proxy/proxy.json --skip-confirm

sleep 1

cargo contract call --contract $PROXY_ADDRESS \
    --message get -s //Alice \
    target/ink/proxy/proxy.json --skip-confirm

sleep 1

# These next couple of messages should be executed by the `Logic` contract since a
# non-admin, `Bob`, is sending the messages.
cargo contract call --contract $PROXY_ADDRESS \
    --message set --args 34 -s //Bob \
    target/ink/logic/logic.json --skip-confirm

sleep 1

cargo contract call --contract $PROXY_ADDRESS \
    --message get -s //Bob \
    target/ink/logic/logic.json --skip-confirm

# Note: If you want to play around with the `admin` overwriting stuff, uncomment this.
#
# cargo contract call --contract $PROXY_ADDRESS \
#     --message set_admin --args 5E7kXs2CJxDEKaqZ9ckB9KHDwUY8LMjPFjhu7VDzpw7ND5iS -s //Bob \
#     target/ink/logic/logic.json --skip-confirm
# 
# sleep 1
# 
# cargo contract call --contract $PROXY_ADDRESS \
#     --message get_admin -s //Bob \
#     target/ink/logic/logic.json --skip-confirm
