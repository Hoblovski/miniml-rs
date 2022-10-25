# MiniML-rs
A rewrite of MiniML in Rust.
Originally [MiniML](https://github.com/Hoblovski/MiniML) was in Python.

# TODO
* parse comments
* typer, codegen
* translate CFG to PEG (now the PEG is unchecked rewrite of CFG)

# Run
```
$ cargo build
$ vim testcases/closure.ml
$ py-miniml -s secd testcases/closure.ml > t.secd	# use the original miniml
$ py-secdi t.secd					# use the original interpreter
$ ./target/debug/secdi t.secd				# rust interpreter
```
