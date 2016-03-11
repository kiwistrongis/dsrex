# Code

## A note on Rust
The main source code for this project is written in rust[^1], a relatively new language. While the vast majority of rust code should be readable to coders from other languages, particularly C and Python, some of the new code structures may be slightly confusing. For help on such subjects, reference [The Rust Book](https://doc.rust-lang.org/book/). Additionally, the [Rust Documentation Site](https://www.rust-lang.org/documentation.html) and [Standard Library API Reference](https://doc.rust-lang.org/std/) may prove useful.

[^1]: Rust Project Website : [rust-lang.org]

## Noteworthy Files
 - src/bin/apriori.rs : Contains the main function for the `apriori` executable
 - src/bin/pcy.rs : Contains the main function for the `pcy` executable
 - src/apriori.rs : Contains implementation of the apriori algorithm
 - src/pcy.rs : Contains implementation of the pcy algorithm
 - src/util.rs : Contains the implementation of the input parser

## Dependencies
To compile this project, you'll need both the `cargo` and `rust` packages. On Archlinux, these are available in the standard repositories and can be installed with a quick `sudo pacman -Syu rust cargo`. On other platforms installation methods may vary. See [rust-lang.org] for more information.

At the time of writing, the cargo and rust package versions are as follows:

```
community/cargo 0.8.0-1
    Rust package manager

community/rust 1:1.6.0-1
    Systems programming language focused on safety, speed and concurrency
```

## Building
To build the project for debugging, a normal `cargo build` is sufficient. Before running any experiments, however, one must build an optimized 'release' (improving performance by a factor of 3). This can be done with `cargo build --release`.

## Running
The two main executables are `target/release/apriori` and `target/release/pcy` ( or alternatively `target/debug/*`). There are also two other executables `test_apriori` and `test_pcy`, which have more verbose output and slightly 'lighter' default settings, which are intended for code-testing purposes.

Each executable should be invoked in the following format:

```
$executable [ -b buffer_size ] [ -s support_threshold ] [ data_file ]
```

For `apriori` and `pcy` the default values of these args are `buffer_size=16`, `support_threshold=0.01`, and `datafile="data/retail.dat"`.

## Experiment Scripts
There are two main experiments scripts - `src/support_exp.sh` and `src/subset_exp.sh`. These can be invoked directly, or with args specifying the algorithm, support thresholds, or data files to run the experiment on. For details, see the scripts themselves.

Additionally, there are two more scripts - `src/run_subset_exp.sh` and `src/run_support_exp.sh`, which were written for running batches of experiments on a four-core machine.

Finally, if on has the netflix dataset located at `data/netflix.dat`, one can use the `src/run_netflix.sh` script for running the `pcy` executable against it.

# Experiments
## Support Threshold
To test how varying the support threshold affected run-time performance, both algorithms were run against the retail dataset with thresholds of 0.1, 0.05, 0.02, 0.01, 0.005, 0.002, and 0.001. The results are as follows:

| Algorithm | Support threshold | Time (seconds) |
|:-:|:-:|-:|
| apriori | 0.1 | 0.30 |
| apriori | 0.05 | 1.40 |
| apriori | 0.02 | 0.46 |
| apriori | 0.01 | 22.10 |
| apriori | 0.005 | 223.57 |
| apriori | 0.002 | 3591.07 |
| apriori | 0.001 | 9875.59 |
| pcy | 0.1 | 3.90 |
| pcy | 0.05 | 4.18 |
| pcy | 0.02 | 4.76 |
| pcy | 0.01 | 5.64 |
| pcy | 0.005 | 10.14 |
| pcy | 0.002 | 300.68 |
| pcy | 0.001 | 7574.55 |

As evident from the above, pcy outperforms apriori only as the support theshold shrinks, increasing the number candidate pairs after the first pass. This makes sense, as the effects of pcy's extra step in the first pass only pay off when it eliminates a significant number of candidate pairs.

A side result of this experiment was finding the (approximate) support threshold required to find the top 100 frequent pairs in the dataset:

| Algorithm | Support threshold | Number of results (max 100) |
|:-:|:-:|-:|
| apriori | 0.1 | 4 |
| apriori | 0.05 | 7 |
| apriori | 0.02 | 22 |
| apriori | 0.01 | 58 |
| apriori | 0.005 | 100 |
| apriori | 0.002 | 100 |
| apriori | 0.001 | 100 |
| pcy | 0.1 | 4 |
| pcy | 0.05 | 7 |
| pcy | 0.02 | 22 |
| pcy | 0.01 | 58 |
| pcy | 0.005 | 100 |
| pcy | 0.002 | 100 |
| pcy | 0.001 | 100 |


## Subsets
To test how dataset size affected run-time performance, both algorithms were run against subsets of the retail dataset with a support threshold of 0.005 (the minimum to find 100 frequent pairs). The subsets were generated with the `src/gen_subsets.sh` script, which created 16 random subsets of varying sizes. The results are as follows[^2]:

[^2]: The result for the retail dataset at a support threshold of 0.005 is included for reference.

| Algorithm | Dataset | Dataset size (lines) | Time (seconds) |
|:-:|:-:|:-:|-:|
| apriori | data/subset_0.dat | 1024 | 3.02 |
| apriori | data/subset_1.dat | 1024 | 2.78 |
| apriori | data/subset_2.dat | 1024 | 3.19 |
| apriori | data/subset_3.dat | 1024 | 2.78 |
| apriori | data/subset_4.dat | 4096 | 9.53 |
| apriori | data/subset_5.dat | 4096 | 10.37 |
| apriori | data/subset_6.dat | 4096 | 11.35 |
| apriori | data/subset_7.dat | 4096 | 11.47 |
| apriori | data/subset_8.dat | 8192 | 23.14 |
| apriori | data/subset_9.dat | 8192 | 22.72 |
| apriori | data/subset_a.dat | 8192 | 22.89 |
| apriori | data/subset_b.dat | 8192 | 23.09 |
| apriori | data/subset_c.dat | 32768 | 80.87 |
| apriori | data/subset_d.dat | 32768 | 82.60 |
| apriori | data/subset_e.dat | 32768 | 78.98 |
| apriori | data/subset_f.dat | 32768 | 82.29 |
| apriori | data/retail.dat | 88162 | 223.57 |
| pcy | data/subset_0.dat | 1024 | 0.15 |
| pcy | data/subset_1.dat | 1024 | 0.16 |
| pcy | data/subset_2.dat | 1024 | 0.17 |
| pcy | data/subset_3.dat | 1024 | 0.15 |
| pcy | data/subset_4.dat | 4096 | 0.45 |
| pcy | data/subset_5.dat | 4096 | 0.44 |
| pcy | data/subset_6.dat | 4096 | 0.48 |
| pcy | data/subset_7.dat | 4096 | 0.48 |
| pcy | data/subset_8.dat | 8192 | 0.94 |
| pcy | data/subset_9.dat | 8192 | 0.90 |
| pcy | data/subset_a.dat | 8192 | 0.89 |
| pcy | data/subset_b.dat | 8192 | 0.89 |
| pcy | data/subset_c.dat | 32768 | 3.52 |
| pcy | data/subset_d.dat | 32768 | 3.47 |
| pcy | data/subset_e.dat | 32768 | 3.35 |
| pcy | data/subset_f.dat | 32768 | 3.46 |
| pcy | data/retail.dat | 88162 | 10.14 |

These results demonstrate how run-time increases linearly with dataset size.

# Netflix Dataset
I currently have two machines analyzing the netflix dataset with a support threshold of 0.01 and 0.005. When any of the processes finish, I will post the results to [kalev.io/netflix_results.dat](http://kalev.io/netflix_results.dat). ( The 0.005 process has been running for 50 hours, and the 0.01 for 5 hours  to date )

<!-- links -->
[rust-lang.org]: https://www.rust-lang.org/