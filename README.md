# Proxy Playground

A place to play around with ink! proxies!

You can deploy and send a few calls through the `Proxy` contract using the
`./proxy-flip.sh` script.

Make sure you have logging enabled on your `substrate-contracts-node`
(`-linfo,runtime::contracts=debug 2>&1`) so you can a bit of insight into what's being
read/written from storage.
