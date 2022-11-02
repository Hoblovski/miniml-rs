# MiniML-rs
A rewrite of MiniML in Rust.
Originally [MiniML](https://github.com/Hoblovski/MiniML) was in Python.

# TODO
* parse comments
* typer, (complete) codegen
* translate CFG to PEG (now the PEG is unchecked rewrite of CFG)
* use more &str than String

# Run
```
$ cargo build
$ vim testcases/closure.ml
$ py-miniml -s secd testcases/closure.ml > t.secd	# use the original miniml
$ py-secdi t.secd					# use the original interpreter
$ ./target/debug/secdi t.secd				# rust interpreter
```

# TESTCASES
| testcase                       | status          |
| ---                            | ---             |
| gcd.ml                         | failed          |
| tuple.ml                       | RSMINIML failed |
| tup_list.ml                    | MINIML failed   |
| evenodd.ml                     | ok              |
| fact.ml                        | ok              |
| typer.ml                       | failed          |
| curry.ml                       | failed          |
| higherorder_badtype.ml         | RSMINIML failed |
| let_rec_variad_poly.ml         | RSMINIML failed |
| adt_qsort.ml                   | RSMINIML failed |
| match.ml                       | RSMINIML failed |
| let_poly_constr.ml             | MINIML failed   |
| adt_qsort_badtype.ml           | MINIML failed   |
| closure.ml                     | failed          |
| let_rec_poly_constrfv.ml       | RSMINIML failed |
| let_rec_variad_poly_badtype.ml | MINIML failed   |
| namer.ml                       | ok              |
| let_poly.ml                    | RSMINIML failed |
| summod.ml                      | failed          |
| adt_list_rand.ml               | MINIML failed   |
| patmat_tup.ml                  | MINIML failed   |
| relu.ml                        | MINIML failed   |
| higherorder.ml                 | RSMINIML failed |
| adt_triv.ml                    | RSMINIML failed |
| helloworld.ml                  | ok              |
| let_rec_poly.ml                | RSMINIML failed |
| typerbad.ml                    | MINIML failed   |
