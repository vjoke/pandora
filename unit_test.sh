#!/bin/bash
if [ "$#" -ne 2 ]; then 
    echo "usage: $0 package_name test_fn"
    exit -1
fi

echo "run unit test for $1::tests::$2"

RUST_LOG=log=info cargo +nightly test -p $1 -- --nocapture $2 