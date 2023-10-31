# SAT_CDCL

A simple implementation of a sat solver using the CDCL (Conflict Driven Clause Learning) heuristic.

To count the number of tests, use `ls -R tests/sat tests/unsat | grep '.cnf' | wc -l`

## Dependencies
All the dependencies of the rust code are handled by Cargo. When one builds the project, all the dependencies are automatically updated.

To install Cargo (and a rust compiler), please refer to [the documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

To run the bash script that runs the solver on the tests, one needs to install the sat solver picosat and gnu parallel. Picosat helps verify that we did not fail our code. Gnu parallel does these two tests in parallel. 
You can run :
```bash
sudo apt-get install picosat && sudo apt-get install parallel
```

## Use
To build the project, you can run `cargo build --release` and use the executable here: `./target/release/sat_solver`. You can also run it directly using `cargo run --release ./path/to/your/test.cnf` without building the project. 
I will write `sat_solver` from now on independently of the way it is run.

Next, run `sat_solver [args] file(s)` to execute the program on your file(s). It is possible to provide multiple files, and the arguments will apply to all of them.

### Args
```
-h --help     Show help message
--cdcl        Using the CDCL solver
--khorn       Using the Horn solver
--2sat        Using the 2sat solver
--dummy       Using the naive solver
--proof       Show the obtained model if the problem is satisfied
-v --verbose  Display precise information. It may takes a bit more time doing more verifications.
```

If no solver is specified in the arguments, the program will determine which solver would optimize the run and execute it on your file(s).

The program will tests wether the input file is Horn (or 2SAR) when the specific solver is asked (with `--khorn` or `--2sat`) if and only if the flag `--verbose` is present.</br>
If both flags `--proof` and `--verbose` are present, the program will test that the ouputed model indeed satisfies the problem.

### Example
To solve the files `tests/sat/horn1.cnf` and `tests/unsat/tseitin5.cnf` using the cdcl solver, run the following command
```bash
$ ./sat_solver --cdcl tests/sat/horn1.cnf tests/unsat/tseitin5.cnf
```
It will returns
```
s SATISFIABLE
s UNSATISFIABLE
```

If the argument `--proof` is used, the proof will be returned in the following format
```bash
s SATISFIABLE
v -1 2 -3 4 -5 6
```
if the assignments: $x_1:=false, x_2:=true, x_3:=false, x_4:=true, x_5:=false, x_6:=true$ is a model satisfying the problem.

## Improvments

In addition to the basic CDCL algorithm, we implemented different other solvers to compare and optimize the results. 

## testing
Before running anything, make sure to have your binary available on the root of the project by calling `cp target/release/sat_solver` . I recommand using `cargo build --release` before that as explained in section [Use](#Use) 
To test the program, run the following line:
```bash
./tests/run.sh <timeout> <jobs>
```
replacing <timeout> by the maximum time spent searching (default 30) and jobs by the number of parallel research done (default 4, requires <timeout> to be changed).

For example, calling `./tests/run.sh 10 8` will run each test in `tests/sat` and `tests/unsat` for at most 10 seconds on 8 differernt jobs.