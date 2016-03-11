#!/bin/bash

# vars
data_file="data/retail.dat"
results_dir="results/asdf"
algos="apriori pcy"
support="0.1 0.05 0.02 0.01 0.005 0.002 0.001"

# time vars
time="/usr/bin/time"
time_format="%e, %U, %S, %P"

# parse args
if [ $# -gt 0 ]; then
	algos=$1
	shift
fi
if [ $# -gt 0 ]; then
	supports=$@
fi

# for each algorithm
for algo in $algos; do
	# clear previous output
	time_output="${results_dir}/${algo}_time.csv"
	echo -n > $time_output
	
	# for each support
	for support in $supports; do
		# vars
		command=target/release/$algo
		output=${results_dir}/${algo}_${support}.dat

		# run algorithm
		echo -e "timing $algo for support $support..."
		echo -en "$algo, $support, " >> $time_output
		$time -f "$time_format" $command -s $support $data_file \
			1> $output 2>> $time_output
	done
done

echo "done"
