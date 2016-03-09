#!/bin/bash

echo -e "analyzing netflix data set..."
target/release/pcy -b 32 -s 0.001 data/netflix.dat
echo -e "done"
