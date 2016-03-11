#!/bin/bash

src/support_exp.sh apriori 0.1 0.02 0.005 0.001 &
pid_a=$!

src/support_exp.sh pcy 0.05 0.01 0.002 &
pid_b=$!

wait $pid_a
wait $pid_b
