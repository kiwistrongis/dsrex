#!/bin/bash

for i in 0 1 2 3; do
	shuf data/retail.dat | head -n 1024 > data/subset$i.dat;
done

for i in 4 5 6 7; do
	shuf data/retail.dat | head -n 8192 > data/subset$i.dat;
done
