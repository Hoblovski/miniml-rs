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
```
adt_list_rand.ml                                  MINIML failed
adt_qsort_badtype.ml                              MINIML failed
adt_qsort.ml                                      RSMINIML failed
adt_triv.ml                                       RSMINIML failed
closure.ml                                        exec result differ
curry.ml                                          exec result differ
evenodd.ml                                        ok
fact.ml                                           ok
gcd.ml                                            ok
helloworld.ml                                     ok
higherorder_badtype.ml                            RSMINIML failed
higherorder.ml                                    RSMINIML failed
let_poly_constr.ml                                MINIML failed
let_poly.ml                                       RSMINIML failed
let_rec_poly_constrfv.ml                          RSMINIML failed
let_rec_poly.ml                                   RSMINIML failed
let_rec_variad_poly_badtype.ml                    MINIML failed
let_rec_variad_poly.ml                            RSMINIML failed
match.ml                                          RSMINIML failed
namer.ml                                          ok
patmat_tup.ml                                     MINIML failed
relu.ml                                           MINIML failed
summod.ml                                         exec result differ
tuple.ml                                          RSMINIML failed
tup_list.ml                                       MINIML failed
typerbad.ml                                       MINIML failed
typer.ml                                          exec result differ
```
