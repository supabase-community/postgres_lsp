# Tests macros

Macros to help auto-generate tests based on files.

## Usage

Pass a glob pattern that'll identify your files and a test-function that'll run for each file. The glob pattern has to start at the root of your crate.

You can add a `.expected.` file next to your test file. Its path will be passed to your test function so you can outcome-based assertions. (Alternatively, write snapshot tests.)

Given the following file structure:

```txt
crate/
|-- src/
|-- tests/
    |-- queries/
        |-- test.sql
        |-- test.expected.sql
    |-- querytest.rs
```

You can generate tests like so:

```rust
  // crate/tests/querytest.rs

  tests_macros::gen_tests!{
    "tests/queries/*.sql",
    crate::run_test  // use `crate::` if the linter complains.
  }

  fn run_test(
    test_path: &str,  // absolute path on the machine
    expected_path: &str, // absolute path of .expected file
    test_dir: &str // absolute path of the test file's parent
  ) {
      // your logic
  }
```

Test name is the "snake case" version of the file name.
this will generate the following for each file:

```rust
#[test]
pub fn somefilename()
{
    let test_file = "<crate's cargo.toml full path>/tests/sometest.txt";
    let test_expected_file = "<crate's cargo.toml full path>/tests/sometest.expected.txt";
    run_test(test_file, test_expected_file);
}
```

## Pitfalls

- If you use a Rust-keyword as a file name, this'll result in invalid syntax for the generated tests.
- All files of the glob-pattern must (currently) be `.sql` files.
- The macro will wrap your tests in a `mod tests { .. }` module. If you need multiple generations, wrap them in modules like so: ```mod some_test { tests_macros::gen_tests! { .. } }`.

## How to run

Simply run your `cargo test` commands as usual.
