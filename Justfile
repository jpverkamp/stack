both name:
    just example {{ name }}
    just example {{ name }} compile=true

example name compile="false" debug="false":
    just example{{ if compile != "false" { "-compile" } else { "-run" } }}{{ if debug != "false" { "-debug" } else { "" } }} {{name}}

example-run name:
    time cargo run --release -- --file examples/{{name}}.stack

example-compile name:
    cargo run --release -- --file examples/{{name}}.stack --compile > output/{{name}}.c
    clang -Ofast output/{{name}}.c -o output/{{name}}
    time output/{{name}}

example-run-debug name:
    cargo run -- --debug --file examples/{{name}}.stack

example-compile-debug name:
    cargo run -- --debug --file examples/{{name}}.stack --compile > output/{{name}}.c
    clang output/{{name}}.c -o output/{{name}}
    output/{{name}}