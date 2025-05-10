#!/bin/bash
# Simple bash utility to shuffle uniqstrings.txt

echo "computing hashes..."
cat $1 | while read l; do
    echo $l | sha256sum >> shuffled_$1.txt;
done