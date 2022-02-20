#!/bin/bash

dir=${1%/}

echo "Running tests in $dir"

ls "$dir/"*.in | rargs -p '(\d*)\.in' time ./run_single_test.sh "$dir" {1} 