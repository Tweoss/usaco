#!/bin/bash

dir=${1%/}

echo "Running tests in $dir"

ls "$dir/"*.in | rargs -p '(\d*)\.in' ./run_single_test.sh "$dir" {1} 