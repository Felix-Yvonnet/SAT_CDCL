# SAT_CDCL

A simple implementation of a sat solver using the CDCL (Conflict Driven Clause Learning) heuristic.

To count the number of tests : `ls -Rf1 tests/ | grep '.cnf' | wc -l`


## Use
This progam imlements different SAT solvers. To use it, you may write `sat_solver [args] file(s)`. The order does not matter and you may ask to solve multiple problem providong multiple files.

### Args
-h --help     Show help message
--cdcl        Using the CDCL solver
--khorn       Using the Horn solver
--dummy       Using the dummy solver
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