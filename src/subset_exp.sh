#!/bin/bash

# vars
data_files="data/subset*"
results_dir="results/subset"
algo="pcy"
support="0.005"

# time vars
time="/usr/bin/time"
time_format="%e, %U, %S, %P"

# parse args
if [ $# -gt 0 ]; then
	algo=$1
	shift
fi
if [ $# -gt 0 ]; then
	data_files=$@
fi

for data_file in $data_files; do

	# vars
	command=target/release/$algo
	subset=${data_file##*/}
	subset=${subset%.*}

	time_output="${results_dir}/${algo}_${subset}.time"
	cmd_output=${results_dir}/${algo}_${subset}.dat

	# run algorithm
	echo -e "timing $algo for file $data_file..."
	echo -en "$algo, $data_file, " >> $time_output
	$time -f "$time_format" $command -s $support $data_file \
		1> $cmd_output 2>> $time_output
done

echo "done"
