StackLang is a simple, stack-based programming language designed for expressive and concise code. 

# Features

* Stack-based execution model
* Minimalistic syntax
* Numeric tower with automatic coercion

## Upcoming features

* Rational/complex number support (full numeric tower)
* Compiler (probably to C as an intermediate?)

# Syntax

StackLang uses a minimalistic, postfix syntax:

* Literals: `42`, `3.14`, `"hello world"`
* Identifiers: `+`, `*`, `writeln`
* Stack variable naming: `42 @x`
* Block definition: `{ @n 2 + } @add2`
* Conditionals: `"hello" "world" 2 3 > if`

## Naming

All variables are stored on a global stack. Named variables assign a specific name to stack indices and lookups are done from the current top of the stack down. 

Naming a variable (or a list of variables all at once) does not remove it from the stack. 

## Blocks

Blocks are the building block of functions in this language. Each block will have an `arity_in` and `arity_out`, the number of values it will pop off the stack and the number it will push back after done. Any other values will be dropped automatically when the block returns.

If possible, arity will be automatically calculated, but if not, you can specify it at the beginning of the function in a few different ways:

* `arity_in` defaults to 0, but can be set to a number with `@2` (for example), a single named value with `@n` or a list of named values with `@[a b c]`
* `arity_out` defaults to 1, but can be set to a number with `!2` (for example)

So to write a simple block that takes 4 values and returns the sum and average:

```
{
    @4 !2  # take 4, return 2
    + + +  # sum them
    dup    # duplicate the sum
    / 4.0  # take the second and divide by 4.0
} @sum_and_avg4

8 6 7 5 sum_and_avg4 writeln writeln
```

Output would be: 

```
26
6.5
```

Dup isn't actually a built in function currently, but can easily be defined with the same structure (+ named variables) as:

```
{ @v !2 v } @dup
```

The first `v` is already on the stack and named with `@v`, the second is added with the latter `v`, then both are returned. 

# Examples

## Factorial as a loop

```
{
    @n
    1 { @2 1 + * } n loop
} @fact

10 fact writeln
```

## Recursive factorial

```
{
  @[n fact]
  1
  { @0 n 1 - $fact fact n * }
  n 1 < if
} @fact

5 $fact fact writeln
```

## Fibonacci using an inner function and accumulator

```
{
    @n 

    {
        @[n a b fibacc]
        b
        {
            @0 !1
            n 1 - 
            a b + 
            a
            $fibacc
            fibacc
        } 
        n 1 <= if
    } @fibacc

    n 1 1 $fibacc fibacc
} @fib

50 fib writeln
```

## Complex multiplication and addition (taking 4 and returning 2 values)

```
{
  @[ar ai br bi] !2
  ar br * ai bi * - 
  ar bi * ai br * +
} @cmul

{
  @[ar ai br bi] !2
  ar br +
  ai bi +
} @cadd

{
  @[r i] !0
  r write 
  "+" write
  i write
  "i" write
} @cwrite

"multiply:" writeln
3 -2 4 5 cmul cwrite newline
22 7 cwrite newline
newline

"add:" writeln
3 -2 4 5 cadd cwrite newline
7 3 cwrite newline
```

## Generating a Mandelbrot set reading the width/height/iterations from stdin

```
# Set image dimensions and maximum number of iterations
read int @width
read int @height
read int @max_iterations

# Set the range of complex numbers to visualize
-2.0 @min_real
1.0 @max_real
-1.0 @min_imag
1.0 @max_imag

# Calculate the step sizes for the real and imaginary parts
max_real min_real - width / @real_step
max_imag min_imag - height / @imag_step

{
  @[ar ai br bi] !2
  ar br * ai bi * - 
  ar bi * ai br * +
} @cmul

{
  @[ar ai br bi] !2
  ar br +
  ai bi +
} @cadd

{
  @[r i]
  r i * r i * +
} @cmag2

{ 
  @[px py max_iter]
  
  {
    @[zx zy i iter] 
    
    0
    {
      @0 !1
      i
      { 
        @0 !1
        zx zy zx zy cmul px py cadd
        i 1 +
        $iter iter
      }
      zx zy cmag2 4.0 > if
    } 
    i max_iter == if

  } @iter

  px py 1 $iter iter
} @mandelbrot

# Write the PPM header
"P3" writeln
width writeln
height writeln
"255" writeln

# Loop through image rows (y) and columns (x)
{
    @y
    {
        @x

        # Calculate the current complex number (real + imag * i)
        x real_step * min_real + @real
        y imag_step * min_imag + @imag

        # Calculate the number of iterations for the current complex number
        real imag max_iterations mandelbrot @iterations

        # Scale the number of iterations to a color value (assuming grayscale)
        1.0 iterations * max_iterations / 255 * int @color

        # Write the color value to the PPM file (red, green, blue)
        color write " " write
        color write " " write
        color write " " write
    } width loop
    newline
} height loop
```

To run this:

```
echo '128\n128\n16' | cargo run -- --file examples/mandelbrot.stack > output/mandelbrot-128x128-16.ppm
```

# Usage

To run a StackLang script:

```
cargo run --file fact.stack
```

Compiler is a work in progress, but will use:

```
cargo run --file fact.stack --compile
```

## Justfile

To run an example:

```
just example-run fibonacci-acc
```

## Logging

Logging is available with `RUST_LOG={level}`. 

* `INFO` will log execution context at each step (lex, parse, and each eval step)
* `DEBUG` will log much more verbose data if needed

### Example logging of fibonacci-acc with n=5

```
┌ ^_^ jp@Mercury {git main} ~/Projects/stacklang
└ RUST_LOG=info just example-run fibonacci-acc

cargo run -- --file examples/fibonacci-acc.stack
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/stack --file examples/fibonacci-acc.stack`
 INFO  stack > Tokens: { @ n { @ [ n a b fibacc ] b { @ 0 ! 1 n 1 - a b + a $ fibacc fibacc } n 1 <= if } @ fibacc n 1 1 $ fibacc fibacc } @ fib 5 fib writeln
 INFO  stack > AST:
Group(
    [
        Block(
            [
                At(
                    Identifier(
                        "n",
                    ),
                ),
                Block(
                    [
                        At(
                            List(
                                [
                                    Identifier(
                                        "n",
                                    ),
                                    Identifier(
                                        "a",
                                    ),
                                    Identifier(
                                        "b",
                                    ),
                                    Identifier(
                                        "fibacc",
                                    ),
                                ],
                            ),
                        ),
                        Identifier(
                            "b",
                        ),
                        Block(
                            [
                                At(
                                    Literal(
                                        Number(
                                            Integer(
                                                0,
                                            ),
                                        ),
                                    ),
                                ),
                                Bang(
                                    Literal(
                                        Number(
                                            Integer(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                                Identifier(
                                    "n",
                                ),
                                Literal(
                                    Number(
                                        Integer(
                                            1,
                                        ),
                                    ),
                                ),
                                Identifier(
                                    "-",
                                ),
                                Identifier(
                                    "a",
                                ),
                                Identifier(
                                    "b",
                                ),
                                Identifier(
                                    "+",
                                ),
                                Identifier(
                                    "a",
                                ),
                                Dollar(
                                    Identifier(
                                        "fibacc",
                                    ),
                                ),
                                Identifier(
                                    "fibacc",
                                ),
                            ],
                        ),
                        Identifier(
                            "n",
                        ),
                        Literal(
                            Number(
                                Integer(
                                    1,
                                ),
                            ),
                        ),
                        Identifier(
                            "<=",
                        ),
                        Identifier(
                            "if",
                        ),
                    ],
                ),
                At(
                    Identifier(
                        "fibacc",
                    ),
                ),
                Identifier(
                    "n",
                ),
                Literal(
                    Number(
                        Integer(
                            1,
                        ),
                    ),
                ),
                Literal(
                    Number(
                        Integer(
                            1,
                        ),
                    ),
                ),
                Dollar(
                    Identifier(
                        "fibacc",
                    ),
                ),
                Identifier(
                    "fibacc",
                ),
            ],
        ),
        At(
            Identifier(
                "fib",
            ),
        ),
        Literal(
            Number(
                Integer(
                    5,
                ),
            ),
        ),
        Identifier(
            "fib",
        ),
        Identifier(
            "writeln",
        ),
    ],
)
 INFO  stack::vm > eval(({@n {@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if} @fibacc n 1 1 $fibacc fibacc} @fib 5 fib writeln), [])
 INFO  stack::vm > eval({@n {@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if} @fibacc n 1 1 $fibacc fibacc}, [])
 INFO  stack::vm > eval(@fib, [{1->1}])
 INFO  stack::vm > eval(5, [{1->1}@fib])
 INFO  stack::vm > eval(fib, [{1->1}@fib 5])
 INFO  stack::vm > eval((@n {@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if} @fibacc n 1 1 $fibacc fibacc), [{1->1}@fib] : [5])
 INFO  stack::vm > eval(@n, [{1->1}@fib] : [5])
 INFO  stack::vm > eval({@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if}, [{1->1}@fib] : [5@n])
 INFO  stack::vm > eval(@fibacc, [{1->1}@fib] : [5@n {4->1}])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc 5])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc 5 1])
 INFO  stack::vm > eval($fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc 5 1 1])
 INFO  stack::vm > eval(fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc 5 1 1 {4->1}])
 INFO  stack::vm > eval((@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5 1 1 {4->1}])
 INFO  stack::vm > eval(@[n a b fibacc], [{1->1}@fib] : [5@n {4->1}@fibacc] : [5 1 1 {4->1}])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc])
 INFO  stack::vm > eval({@0 !1 n 1 - a b + a $fibacc fibacc}, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc 1])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc 1 {0->1}])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc 1 {0->1} 5])
 INFO  stack::vm > eval(<=, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc 1 {0->1} 5 1])
 INFO  stack::vm > eval(if, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc 1 {0->1} false])
 INFO  stack::vm > eval((@0 !1 n 1 - a b + a $fibacc fibacc), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(@0, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(!1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [5])
 INFO  stack::vm > eval(-, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [5 1])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [4])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [4 1])
 INFO  stack::vm > eval(+, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [4 1 1])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [4 2])
 INFO  stack::vm > eval($fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [4 2 1])
 INFO  stack::vm > eval(fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [4 2 1 {4->1}])
 INFO  stack::vm > eval((@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4 2 1 {4->1}])
 INFO  stack::vm > eval(@[n a b fibacc], [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4 2 1 {4->1}])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc])
 INFO  stack::vm > eval({@0 !1 n 1 - a b + a $fibacc fibacc}, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc 1])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc 1 {0->1}])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc 1 {0->1} 4])
 INFO  stack::vm > eval(<=, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc 1 {0->1} 4 1])
 INFO  stack::vm > eval(if, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc 1 {0->1} false])
 INFO  stack::vm > eval((@0 !1 n 1 - a b + a $fibacc fibacc), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(@0, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(!1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [4])
 INFO  stack::vm > eval(-, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [4 1])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [3])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [3 2])
 INFO  stack::vm > eval(+, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [3 2 1])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [3 3])
 INFO  stack::vm > eval($fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [3 3 2])
 INFO  stack::vm > eval(fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [3 3 2 {4->1}])
 INFO  stack::vm > eval((@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3 3 2 {4->1}])
 INFO  stack::vm > eval(@[n a b fibacc], [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3 3 2 {4->1}])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc])
 INFO  stack::vm > eval({@0 !1 n 1 - a b + a $fibacc fibacc}, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc 2])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc 2 {0->1}])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc 2 {0->1} 3])
 INFO  stack::vm > eval(<=, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc 2 {0->1} 3 1])
 INFO  stack::vm > eval(if, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc 2 {0->1} false])
 INFO  stack::vm > eval((@0 !1 n 1 - a b + a $fibacc fibacc), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(@0, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(!1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [3])
 INFO  stack::vm > eval(-, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [3 1])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [2])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [2 3])
 INFO  stack::vm > eval(+, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [2 3 2])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [2 5])
 INFO  stack::vm > eval($fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [2 5 3])
 INFO  stack::vm > eval(fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [2 5 3 {4->1}])
 INFO  stack::vm > eval((@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2 5 3 {4->1}])
 INFO  stack::vm > eval(@[n a b fibacc], [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2 5 3 {4->1}])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc])
 INFO  stack::vm > eval({@0 !1 n 1 - a b + a $fibacc fibacc}, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc 3])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc 3 {0->1}])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc 3 {0->1} 2])
 INFO  stack::vm > eval(<=, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc 3 {0->1} 2 1])
 INFO  stack::vm > eval(if, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc 3 {0->1} false])
 INFO  stack::vm > eval((@0 !1 n 1 - a b + a $fibacc fibacc), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(@0, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(!1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [2])
 INFO  stack::vm > eval(-, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [2 1])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [1])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [1 5])
 INFO  stack::vm > eval(+, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [1 5 3])
 INFO  stack::vm > eval(a, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [1 8])
 INFO  stack::vm > eval($fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [1 8 5])
 INFO  stack::vm > eval(fibacc, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [1 8 5 {4->1}])
 INFO  stack::vm > eval((@[n a b fibacc] b {@0 !1 n 1 - a b + a $fibacc fibacc} n 1 <= if), [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1 8 5 {4->1}])
 INFO  stack::vm > eval(@[n a b fibacc], [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1 8 5 {4->1}])
 INFO  stack::vm > eval(b, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1@n 8@a 5@b {4->1}@fibacc])
 INFO  stack::vm > eval({@0 !1 n 1 - a b + a $fibacc fibacc}, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1@n 8@a 5@b {4->1}@fibacc 5])
 INFO  stack::vm > eval(n, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1@n 8@a 5@b {4->1}@fibacc 5 {0->1}])
 INFO  stack::vm > eval(1, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1@n 8@a 5@b {4->1}@fibacc 5 {0->1} 1])
 INFO  stack::vm > eval(<=, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1@n 8@a 5@b {4->1}@fibacc 5 {0->1} 1 1])
 INFO  stack::vm > eval(if, [{1->1}@fib] : [5@n {4->1}@fibacc] : [5@n 1@a 1@b {4->1}@fibacc] : [] : [4@n 2@a 1@b {4->1}@fibacc] : [] : [3@n 3@a 2@b {4->1}@fibacc] : [] : [2@n 5@a 3@b {4->1}@fibacc] : [] : [1@n 8@a 5@b {4->1}@fibacc 5 {0->1} true])
 INFO  stack::vm > eval(writeln, [{1->1}@fib 5])
5
```

# Tests

There are currently test cases written for the lexer and parser:

```
┌ ^_^ jp@Mercury {git main} ~/Projects/stacklang
└ cargo test

    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (target/debug/deps/stack-8bce093d04ffb758)

running 22 tests
test lexer::test::test_float_scientific ... ok
test lexer::test::test_floats ... ok
test lexer::test::test_identifiers ... ok
test lexer::test::test_brackets ... ok
test lexer::test::test_integers ... ok
test lexer::test::test_hex ... ok
test lexer::test::test_binary ... ok
test lexer::test::test_negative_integers ... ok
test lexer::test::test_strings ... ok
test lexer::test::test_rationals ... ok
test lexer::test::test_prefixed ... ok
test lexer::test::test_symbolic ... ok
test parser::test::test_boolean_literal ... ok
test parser::test::test_factorial ... ok
test parser::test::test_naming ... ok
test parser::test::test_assignment_bang ... ok
test parser::test::test_integer ... ok
test parser::test::test_list_naming ... ok
test parser::test::test_float ... ok
test parser::test::test_simple_block ... ok
test parser::test::test_simple_addition ... ok
test parser::test::test_string_literal ... ok

test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

Thank you Github Copilot. 

More will be forthcoming. 