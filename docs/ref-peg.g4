ptn
    : ptn0
    ;

ptn0
    : ptn1 (',' ptn1)+ # ptnTuplex
    | Ident ptn1+      # ptnData
    ;

ptn1
    : Ident          # ptnBinder
    | lit            # ptnLit
    | '(' ptn ')'    # ptnParen
    ;


ptn
  : ptn1 (',' ptn1)*		# tuples

ptn1
  : '(' ptn ')'			# paren
  | lit				# lit
  | `Ident			# varmatch
  | Ident ptn+			# adt
  | Ident			# binder



example
| Nil a
	-> adt (binder)

| a, Nil (), c

