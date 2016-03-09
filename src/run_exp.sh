#!/bin/bash

# time vars
time="/usr/bin/time"
time_format="%e, %U, %S, %P"
time_output="results/time.csv"

# vars
data_file="data/retail.dat"
algos="apriori pcy"
supports="0.1 0.02 0.005 0.001"

# parse args
echo -e "$1"
if [ $# -gt 0 ]; then
	algos=$1
	time_output="results/${algo}_time.csv"
fi

# for each algorithm
for algo in $algos; do
	# clear previous output
	echo -n > $time_output
	
	# for each support
	for support in $supports; do
		# vars
		command=target/release/$algo
		output=results/${algo}_${support}.dat
		time_output="results/${algo}_time.csv"

		# run algorithm
		echo -e "timing $algo for support $support..."
		echo -en "$algo, $support, " >> $time_output
		$time -f "$time_format" $command -s $support $data_file \
			1> $output 2>> $time_output
	done
done

echo "done"
