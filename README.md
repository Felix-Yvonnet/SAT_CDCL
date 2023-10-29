# SAT_CDCL

A simple implementation of a sat solver using the CDCL (Conflict Driven Clause Learning) heuristic.

To count the number of tests : `ls -R tests/sat tests/unsat | grep '.cnf' | wc -l`

## Dependencies
All the dependencies of the rust code are handled by Cargo. When one build the project all the dependencies are automatically updated.

To install Cargo (and a rust compiler), please refer to [the documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

To run the bash script that runs the solver on the tests, one needs to install the sat solver picosat and gnu parallel. Those will help verify that we did not fail our code and do the tests in parallel. 
You can run :
```bash
sudo apt-get install picosat && sudo apt-get install parallel
```

## Use
One can build the project using `cargo build --release` and using the executable here: `./target/release/sat_solver`. But it is also possible to run it directly using `cargo run --release ./path/to/your/test.cnf` without building the project. 
I will write `sat_solver` from now on independantly of the way it is run.

This progam implements different SAT solvers. To use it, one may write `sat_solver [args] file(s)`. The order does not matter and it is possible to solve multiple problem providing multiple files. The arguments will apply to all of them.

### Args
-h --help     Show help message
--cdcl        Using the CDCL solver
--khorn       Using the Horn solver
--2sat        Using the 2sat solver
--dummy       Using the naive solver
--proof       Test whether the returned assigments are correct (the ouput model indeed satisfies the problem)
-v --verbose  Print the model and different informations. The problem may take a bit more time doing more verifications.

### Example
To solve the files `tests/sat/horn1.cnf` and `tests/unsat/tseitin5.cnf` one can type the following command :
```bash
$ ./sat_solver --cdcl tests/sat/horn1.cnf tests/unsat/tseitin5.cnf
```
It will returns
```
s SATISFIABLE
s UNSATISFIABLE
```

The proof will be presented in the following format:
```bash
s SATISFIABLE
v -1 2 -3 4 -5 6
```
if the assignments: $x_1:=false, x_2:=true, x_3:=false, x_4:=true, x_5:=false, x_6:=true$ is a model satisfying the problem.

## Improvments

In addition to the basic CDCL algorithm, we implemented different other solvers to compare and optimize the results. 