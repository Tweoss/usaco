#!/bin/bash

cd $1
a=$(cat "$2.in" | cargo run --release -q)
b=$(cat "$2.out")
if [ "$a" == "$b" ]; then
	echo "$2 is OK"
else
	prettydiff <(echo "$a") "$2.out"
fi
