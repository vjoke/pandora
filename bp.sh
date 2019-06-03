#!/bin/bash
if [ "$#" -ne 1 ]; then 
    echo "usage: $0 package_name"
    exit -1
fi

echo "build package $1"

cargo +nightly build -p $1