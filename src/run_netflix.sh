#!/bin/bash

# vars
algo=pcy
support=0.01

echo -e "analyzing netflix dataset with $algo and support threshold of $support..."
target/release/$algo -b 32 -s $support data/netflix.dat
echo -e "done"
