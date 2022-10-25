let f = \x -> (
        (\y -> let g = \z -> z + y in g)
        (x+1)
) in
        f 1 2
