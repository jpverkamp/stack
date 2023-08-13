#[cfg(test)]
mod test {
    use paste::paste;
    use std::process::Command;
    use std::str;

    macro_rules! make_tests {
        ($name:ident: $path:expr => $target:expr) => {
            paste! {
                #[test]
                fn [< test_vm_ $name >]() {
                    let vm_output = Command::new("cargo")
                        .arg("run")
                        .arg("--")
                        .arg("vm")
                        .arg($path)
                        .output()
                        .expect("failed to run vm");

                    assert_eq!(vm_output.status.success(), true, "vm exit code");
                    assert_eq!(
                        str::from_utf8(&vm_output.stdout).unwrap(),
                        $target,
                        "vm output"
                    );
                }

                #[test]
                fn [< test_compile_ $name >]() {
                    let compile_output = Command::new("cargo")
                        .arg("run")
                        .arg("--")
                        .arg("compile")
                        .arg($path)
                        .arg("--output")
                        .arg(format!("output/test-{}.c", $path.replace("/", "-")))
                        .arg("--run")
                        .output()
                        .expect("failed to execute process");

                    assert_eq!(
                        compile_output.status.success(),
                        true,
                        "c compiler exit code"
                    );
                    assert_eq!(
                        str::from_utf8(&compile_output.stdout).unwrap(),
                        $target,
                        "c compiler output"
                    );
                }
            }
        };
    }

    make_tests!(add2: "examples/add2.stack" => "12\n");
    make_tests!(basic_math: "examples/basic-math.stack" => "98\n");
    make_tests!(basic_logic: "examples/boolean-ops.stack" => "\
true and true is true
true or true is true
true xor true is false
true nand true is false
not true is false

true and false is false
true or false is true
true xor false is true
true nand false is true
not true is false

false and true is false
false or true is true
false xor true is true
false nand true is true
not false is true

false and false is false
false or false is false
false xor false is false
false nand false is true
not false is true

");
    make_tests!(named_variables: "examples/double-named.stack" => "20\n");
    make_tests!(name_2: "examples/name-two.stack" => "3\n");
    make_tests!(arity_in_2: "examples/arity-in-2.stack" => "15\n");
    make_tests!(arity_out_2: "examples/dup.stack" => "5\n5\n");
    make_tests!(arity_2_2: "examples/complex-test.stack" => "\
multiply:
actual: 22+7i
expect: 22+7i

add:
actual: 7+3i
expect: 7+3i
"
    );
    make_tests!(loop: "examples/factorial-loop.stack" => "3628800\n");
    make_tests!(if: "examples/if.stack" => "hello\ngoodbye\nhello\ngoodbye\n");
    make_tests!(when: "examples/when.stack" => "\
0 is divisible by 3 is divisible by 5
1
2
3 is divisible by 3
4
5 is divisible by 5
6 is divisible by 3
7
8
9 is divisible by 3
10 is divisible by 5
11
12 is divisible by 3
13
14
15 is divisible by 3 is divisible by 5
16
17
18 is divisible by 3
19
");
    make_tests!(list: "examples/list.stack" => "[1, 2, 3]\n3\n[1, 2]\n[1, 2, 5]\n2\n");
    make_tests!(lists_of_lists: "examples/lists-of-lists.stack" => "\
[[1, 2, 3], [4, 5], [7, 8, 9, 0]]
---
[4, 5]: [4, 5]
9: 9
---
[[1, 2, 3], [4, 5], [7, 8, 9, 0], [a, b, c]]
"
    );
    make_tests!(loop_list: "examples/generate-stack.stack" => "[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]\n");
    make_tests!(recursion: "examples/factorial.stack" => "3628800\n");
    make_tests!(recursive_helper: "examples/fibonacci-acc.stack" => "102334155\n");
    make_tests!(mutual_recursion: "examples/even-odd.stack" => "false\ntrue\ntrue\nfalse\n");
    make_tests!(cond_recursion: "examples/collatz.stack" => "\
1 => 0
2 => 1
3 => 7
4 => 2
5 => 5
6 => 8
7 => 16
8 => 3
9 => 19
10 => 6
11 => 14
12 => 9
13 => 9
14 => 17
15 => 17
16 => 4
17 => 12
18 => 20
19 => 20
20 => 7
");
}
