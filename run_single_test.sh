#!/bin/bash

cd $1
a=$(cat "$2.in" | cargo run -q)
b=$(cat "$2.out")
if [ "$a" == "$b" ]; then
	echo "$2 is OK"
else
	prettydiff <(echo "$a") <(echo "$b")
fi
