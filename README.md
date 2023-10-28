# SAT_CDCL

A simple implementation of a sat solver using the CDCL (Conflict Driven Clause Learning) heuristic.

To count the number of tests : `ls -Rf1 tests/ | grep '.cnf' | wc -l`


## Install

## Dependencies
All the dependencies of the rust code are handled by Cargo. When one build the project all the dependencies are automatically updated.

If you want to run the bash script that tests for a limited amount of time your sat solver on real life examples, you need to have installed the sat solver picosat and gnu parallel (in order to test in parallel). 
You can run :
```bash
sudo apt-get install picosat && sudo apt-get install parallel
```

## Use
This progam imlements different SAT solvers. To use it, you may write `sat_solver [args] file(s)`. The order does not matter and you may ask to solve multiple problem providong multiple files.

### Args
-h --help     Show help message
--cdcl        Using the CDCL solver
--khorn       Using the Horn solver
--2sat        Using the 2sat solver
--dummy       Using the naive solver
-v --verbose  Print the time spent and different informations
-t --time     Limit the maximum time (in seconds) to spend searching

### Example
To solve the files `tests/sat/horn1.cnf` and `tests/unsat/tseitin5.cnf` you can type the following command :
```bash
$ ./sat_solver --cdcl tests/sat/horn1.cnf tests/unsat/tseitin5.cnf
```
It will returns
```
SAT
UNSAT
```