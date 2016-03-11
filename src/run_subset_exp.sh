#!/bin/bash

# vars
subset_0="data/subset_0.dat"
subset_1="data/subset_1.dat"
subset_2="data/subset_2.dat"
subset_3="data/subset_3.dat"
subset_4="data/subset_4.dat"
subset_5="data/subset_5.dat"
subset_6="data/subset_6.dat"
subset_7="data/subset_7.dat"
subset_8="data/subset_8.dat"
subset_9="data/subset_9.dat"
subset_a="data/subset_a.dat"
subset_b="data/subset_b.dat"
subset_c="data/subset_c.dat"
subset_d="data/subset_d.dat"
subset_e="data/subset_e.dat"
subset_f="data/subset_f.dat"

## apriori 0..3
# run apriori on four 1024 line subsets
algo=apriori
echo -e "running $algo on subsets 0..3"
src/subset_exp.sh $algo $subset_0 &
pid_a=$!
src/subset_exp.sh $algo $subset_1 &
pid_b=$!
src/subset_exp.sh $algo $subset_2 &
pid_c=$!
src/subset_exp.sh $algo $subset_3 &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d

## pcy 0..3
# run pcy on four 1024 line subsets
algo=pcy
echo -e "running $algo on subsets 0..3"
src/subset_exp.sh $algo $subset_0 &
pid_a=$!
src/subset_exp.sh $algo $subset_1 &
pid_b=$!
src/subset_exp.sh $algo $subset_2 &
pid_c=$!
src/subset_exp.sh $algo $subset_3 &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d

## apriori 4..7
# run apriori on four 1024 line subsets
algo=apriori
echo -e "running $algo on subsets 4..7"
src/subset_exp.sh $algo $subset_4 &
pid_a=$!
src/subset_exp.sh $algo $subset_5 &
pid_b=$!
src/subset_exp.sh $algo $subset_6 &
pid_c=$!
src/subset_exp.sh $algo $subset_7 &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d

## pcy 4..7
# run pcy on four 1024 line subsets
algo=pcy
echo -e "running $algo on subsets 4..7"
src/subset_exp.sh $algo $subset_4 &
pid_a=$!
src/subset_exp.sh $algo $subset_5 &
pid_b=$!
src/subset_exp.sh $algo $subset_6 &
pid_c=$!
src/subset_exp.sh $algo $subset_7 &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d

## apriori 8..b
# run apriori on four 1024 line subsets
algo=apriori
echo -e "running $algo on subsets 8..b"
src/subset_exp.sh $algo $subset_8 &
pid_a=$!
src/subset_exp.sh $algo $subset_9 &
pid_b=$!
src/subset_exp.sh $algo $subset_a &
pid_c=$!
src/subset_exp.sh $algo $subset_b &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d

## pcy 8..b
# run pcy on four 1024 line subsets
algo=pcy
echo -e "running $algo on subsets 8..b"
src/subset_exp.sh $algo $subset_8 &
pid_a=$!
src/subset_exp.sh $algo $subset_9 &
pid_b=$!
src/subset_exp.sh $algo $subset_a &
pid_c=$!
src/subset_exp.sh $algo $subset_b &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d

## apriori c..f
# run apriori on four 1024 line subsets
algo=apriori
echo -e "running $algo on subsets c..f"
src/subset_exp.sh $algo $subset_c &
pid_a=$!
src/subset_exp.sh $algo $subset_d &
pid_b=$!
src/subset_exp.sh $algo $subset_e &
pid_c=$!
src/subset_exp.sh $algo $subset_f &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d

## pcy c..f
# run pcy on four 1024 line subsets
algo=pcy
echo -e "running $algo on subsets c..f"
src/subset_exp.sh $algo $subset_c &
pid_a=$!
src/subset_exp.sh $algo $subset_d &
pid_b=$!
src/subset_exp.sh $algo $subset_e &
pid_c=$!
src/subset_exp.sh $algo $subset_f &
pid_d=$!

wait $pid_a
wait $pid_b
wait $pid_c
wait $pid_d
