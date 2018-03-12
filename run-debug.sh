#!/bin/bash

set -xue

root_dir=$PWD

./build-debug.sh

cd build

./lc-debug

cd $root_dir
