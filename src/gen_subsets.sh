#!/bin/bash

for i in 0 1 2 3; do
	shuf data/retail.dat | head -n 1024 > data/subset_$i.dat;
done

for i in 4 5 6 7; do
	shuf data/retail.dat | head -n 4096 > data/subset_$i.dat;
done

for i in 8 9 a b; do
	shuf data/retail.dat | head -n 8192 > data/subset_$i.dat;
done

for i in c d e f; do
	shuf data/retail.dat | head -n 32768 > data/subset_$i.dat;
done
