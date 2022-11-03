# MiniML-rs
A rewrite of MiniML in Rust.
Originally [MiniML](https://github.com/Hoblovski/MiniML) was in Python.

# Features
> adopted from miniml/README
* Function as first-class citizen, higher order functions
* Compile to SECD IR
* Associated IR interpreter.
* [TODO] Polymorphic type checking.
* [TODO] Pattern matching (as powerful as in ML).
* [TODO] garbage collection
* [TODO] polymorphic types
* [TODO] optimizations
  - [Tail recursive SECD](https://www.cs.utexas.edu/users/boyer/ftp/nqthm/trsecd/trsecd.html)
  - CPS
  - Even G machine maybe?

# Run
```bash
$ cargo build
$ ./target/debug/miniml testcases/closure.ml -o t.secd
$ ./target/debug/secdi t.secd
```

You can cross check miniml-rs with miniml by
```bash
$ ./scripts/xchk.sh
```

# TESTCASES
Results from xchk.sh:

```
adt_list_rand.ml                                  MINIML failed
adt_qsort_badtype.ml                              MINIML failed
adt_qsort.ml                                      RSMINIML failed
adt_triv.ml                                       RSMINIML failed
ambig.ml                                          RSMINIML failed
closure.ml                                        ok
curry.ml                                          ok
evenodd.ml                                        ok
fact.ml                                           ok
gcd.ml                                            ok
helloworld.ml                                     ok
higherorder_badtype.ml                            ok
higherorder.ml                                    ok
let_poly_constr.ml                                MINIML failed
let_poly.ml                                       RSMINIML failed
let_rec_poly_constrfv.ml                          RSMINIML failed
let_rec_poly.ml                                   RSMINIML failed
let_rec_variad_poly_badtype.ml                    MINIML failed
let_rec_variad_poly.ml                            RSMINIML failed
match.ml                                          RSMINIML failed
namer.ml                                          ok
patmat_tup.ml                                     RSMINIML failed
relu.ml                                           ok
summod.ml                                         ok
tuple.ml                                          RSMINIML failed
tup_list.ml                                       MINIML failed
typerbad.ml                                       MINIML failed
typer.ml                                          ok
```

spurious ok:
* typer
* anything containing bad

# TODO
* typer, (complete) codegen
* validate translation from CFG to PEG (now the PEG is unchecked rewrite of CFG)
* use more &str than String (at the expense of littering <'a>?)
* xchk for negative input
