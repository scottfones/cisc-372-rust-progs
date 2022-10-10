# Breakout 04: Array2d

This is a "souped up" version of an exercise from last class.

## Array2d

Work in a subdirectory named "2d" of your breakout/04 directory.
Write a C program `array2d` with all of the following:

### `create`

1. A function `create` which creates a 2-dimensional n x m array of
doubles, allocating memory on the heap:

double ** create(int n, int m);

### `init`

2. A function `init` which initializes a 2d array of doubles, setting
all boundary cells to `100.0`, and all the interior points to `0.0`. For
example, if `n=4` and `m=5`:

```bash
100   100   100   100   100
100     0     0     0   100
100     0     0     0   100
100   100   100   100   100
```

(Except, one or 2 digits after the decimal point would be better.) 
Figure out what the arguments to `init` should be.

### `print`

3. A function `print` that prints a 2d array of doubles.

### `destroy`

4. A function `destroy` that deallocates a 2d array of doubles
that was allocated by your "create" function.

### `main`

5. A `main` function which takes two command line arguments, `n` and `m`,
both positive integers.  It should generate an assertion violation if
the number of arguments is not correct, or if any argument is not a
positive integer.  It should then call `create`, `init`, `print`, and
`destroy`.

### tests
6. Write a Makefile to compile, test, and clean.
