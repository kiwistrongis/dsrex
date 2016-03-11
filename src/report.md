# Asdf
## Dependencies
This code is written in rust, a relatively new language. To compile, you'll need both the `cargo` and `rust` packages. On Archlinux, these are available in the standard repositories and can be installed with a quick `sudo pacman -Syu rust cargo`. On other platforms installation methods may vary.

At the time of writing, the cargo and rust package versions are as follows:

```
community/cargo 0.8.0-1
    Rust package manager

community/rust 1:1.6.0-1
    Systems programming language focused on safety, speed and concurrency
```

## Building
To build the project for debugging, a normal `cargo build` is sufficient. Before running any experiments, however, one must build an optimized 'release'. This can be done with `cargo build --release`

## Running
The two main executables are `target/release/apriori` and `target/release/pcy` ( or alternatively `target/debug/*`). There are also two other executables `test_apriori` and `test_pcy`, which have more verbose output and slightly 'lighter' default settings, which are intended for code-testing purposes.

Each executable should be invoked in the following format:

```
$executable [ -b buffer_size ] [ -s support_threshold ] [ data_file ]
```

For `apriori` and `pcy` the default values of these args are `buffer_size=16`, `support_threshold=0.01`, and `datafile="data/retail.dat"`.

## Experiments
There are two main experiments - `src/support_exp.sh` and `src/subset_exp.sh`. These can be invoked directly, or with args specifying the algorithm, support thresholds, or data files to run the experiment on. For details, see the scripts themselves.

Additionally, there are two more scripts - `src/run_subset_exp.sh` and `src/run_support_exp.sh`, which were written for running batches of experiments on a four-core machine.

Finally, if on has the netflix dataset located at `data/netflix.dat`, one can use the `src/run_netflix.sh` script for running the `pcy` executable against it.
