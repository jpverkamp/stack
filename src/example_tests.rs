#[cfg(test)]
mod test {
    use std::process::Command;
    use std::str;

    fn test(path: &str, target: &str) {
        let vm_output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("vm")
            .arg(path)
            .output()
            .expect("failed to run vm");

        assert_eq!(vm_output.status.success(), true, "vm exit code");
        assert_eq!(
            str::from_utf8(&vm_output.stdout).unwrap(),
            target,
            "vm output"
        );

        let compile_output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("compile")
            .arg(path)
            .arg("--output")
            .arg(format!("output/test-{}.c", path.replace("/", "-")))
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
            target,
            "c compiler output"
        );
    }

    #[test]
    fn test_add2() {
        test("examples/add2.stack", "12\n");
    }

    #[test]
    fn test_basic_math() {
        test("examples/basic-math.stack", "98\n");
    }

    #[test]
    fn test_named_variables() {
        test("examples/double-named.stack", "20\n");
    }

    #[test]
    fn test_name_2() {
        test("examples/name-two.stack", "3\n");
    }

    #[test]
    fn test_arity_in_2() {
        test("examples/arity-in-2.stack", "15\n");
    }

    #[test]
    fn test_arity_out_2() {
        test("examples/dup.stack", "5\n5\n");
    }

    #[test]
    fn test_arity_2_2() {
        test(
            "examples/complex-test.stack",
            "\
multiply:
actual: 22+7i
expect: 22+7i

add:
actual: 7+3i
expect: 7+3i
",
        );
    }

    #[test]
    fn test_loop() {
        test("examples/factorial-loop.stack", "3628800\n");
    }

    #[test]
    fn test_loop_apply() {
        test("examples/factorial-loop-apply.stack", "3628800\n");
    }

    #[test]
    fn test_if() {
        test("examples/if.stack", "hello\ngoodbye\nhello\ngoodbye\n");
    }

    #[test]
    fn test_list() {
        test(
            "examples/list.stack",
            "[1, 2, 3]\n3\n[1, 2]\n[1, 2, 5]\n2\n",
        );
    }

    #[test]
    fn test_lists_of_lists() {
        test(
            "examples/lists-of-lists.stack",
            "\
[[1, 2, 3], [4, 5], [7, 8, 9, 0]]
---
[4, 5]: [4, 5]
9: 9
---
[[1, 2, 3], [4, 5], [7, 8, 9, 0], [a, b, c]]
",
        );
    }

    #[test]
    fn test_loop_list() {
        test(
            "examples/loop-list.stack",
            "[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]\n",
        );
    }

    #[test]
    fn test_recursion() {
        test("examples/factorial.stack", "3628800\n");
    }

    #[test]
    fn test_recursive_helper() {
        test("examples/fibonacci-acc.stack", "102334155\n");
    }

    #[test]
    fn test_mutual_recursion() {
        test("examples/even-odd.stack", "false\ntrue\ntrue\nfalse\n");
    }

    #[test]
    fn test_cond_recursion() {
        test(
            "examples/collatz.stack",
            "\
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
",
        );
    }
}
