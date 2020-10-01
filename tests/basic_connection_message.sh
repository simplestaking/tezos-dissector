#!/usr/bin/env bash

function ok {
    echo OK
}

function fail {
    echo FAILED
    exit 1
}

tshark \
    -o tezos.identity_json_file:data/identity.json \
    -Vr data/cap-09.pcap | grep 'Connection message' \
    >/dev/null 2>/dev/null && ok || fail
