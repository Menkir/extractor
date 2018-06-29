extern crate extractor;
use untitled::*;

#[test]
fn test_without_error() {
    assert_eq!(
        get_error_text_without_error_code(String::from(
            "error[E0382]: use of moved value: `v`\n
 --> examples/fail.rs:4:29\n
  |\n
3 |     let v2 = v;\n
  |         -- value moved here\n
4 |     println!(\"v[0] is: {}\", v[0]);\n
  |                             ^ value used here after move\n
  |\n
  = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait",
        )),
        vec![String::from("use of moved value")]
    )
}

#[test]
fn test_with_error() {
    assert_eq!(
        get_error_text_with_error_code(String::from(
            "error[E0382]: use of moved value: `v`\n
 --> examples/fail.rs:4:29\n
  |\n
3 |     let v2 = v;\n
  |         -- value moved here\n
4 |     println!(\"v[0] is: {}\", v[0]);\n
  |                             ^ value used here after move\n
  |\n
  = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait",
        )),
        vec![String::from("error[E0382]: use of moved value")]
    )
}

#[test]
fn test_warning() {
    let empty: Vec<String> = vec![];
    assert_eq!(
        get_error_text_with_error_code(String::from(
            "warning: unused variable: `s`\n
 --> src/lib.rs:9:23\n
  |\n
9 | pub fn get_error_code(s: String) -> String{\n
  |                       ^ help: consider using `_s` instead\n
  |\n
  = note: #[warn(unused_variables)] on by default",
        )),
        empty
    )
}

#[test]
fn test_multiple_error_messages() {
    assert_eq!(
        get_error_text_without_error_code(String::from(
            "error[E0369]: binary operation + cannot be applied to type <T as std::ops::Mul>::Output\n
    error[E0599]: no associated item named `Lifetime`\n",
        )),
        vec![
            "binary operation cannot be applied to type",
            "no associated item named",
        ]
    )
}
