example-run name:
    cargo run -- --file examples/{{name}}.stack

example-compile-and-run name:
    cargo run -- --file examples/{{name}}.stack --compile > output/{{name}}.c
    clang output/{{name}}.c -o output/{{name}}
    output/{{name}}