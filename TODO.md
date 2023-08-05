* Test harness
  * Run and store in/out/err for all examples; move to test cases?
* Structs
  * Parsing for dotted identifiers
  * Virtual tables for storing associated data and function pointers
* Type checking:
  * Automatically determine specific types of expressions (including blocks)
  * Automatically determine the arity of blocks when possible
* Numeric tower:
  * Implement rationals/complex numbers at the parser level + in any interpreter / compiler I have at that point
* Interpreters:
  * A bytecode interpreter/compiler, evaluating at a lower level (I’m not sure how much this would gain, the AST is already fairly low level)
* Compilers:
  * Compile to WASM; since it’s also stack based, this should be interesting
  * Compile to x86/ARM assembly