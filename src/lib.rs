//! # Extractor
//!
//! This is a experimental crate for extracting possible inline titles for error searching with
//! [resa](https://github.com/Menkir/resa).
//!
//! The following example will demonstrate the purpose:
//! ```text
//! error[E0382]: use of moved value: 'v'
//! --> examples/fail.rs:4:29
//! |
//! 3 |     let v2 = v;
//! |         -- value moved here
//! 4 |     println!("v[0] is: {}\", v[0]);
//!   |                             ^ value used here after move
//! |
//! = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait"
//! ```
//!
//! A compiler error. The extractor crate will search for the inline title `use of moved value`. To get this sentence
//! the crate need to filter, split and match regular expressions.
//!
//! Based on this title we're able to search e.g on StackOverflow for similar issues.
extern crate regex;
use regex::Regex;


const WHITESPACE: char = ' ';
const NEWLINE: char = '\n';
const FIRST_LINE: usize = 0;

/// Retuns a Vector of error sentences with error code
///
/// # Arguments
/// * `s` - A error message as String
///
/// # Example
/// ```rust
/// assert_eq!(extractor::get_error_text_without_error_code(String::from("error[E0369]: binary operation + cannot be applied to type <T as std::ops::Mul>::Output")),
/// vec!["binary operation cannot be applied to type"]);
/// ```
pub fn get_error_text_with_error_code(s: String) -> Vec<String> {
    get_error_text(s, true)
}

/// Retuns a Vector of error sentences without error code
///
/// # Arguments
/// * `s` - A error message as String
///
/// # Example
/// ```rust
/// assert_eq!(extractor::get_error_text_with_error_code(String::from("error[E0369]: binary operation + cannot be applied to type <T as std::ops::Mul>::Output")),
/// vec!["error[E0369]: binary operation cannot be applied to type"]);
/// ```
pub fn get_error_text_without_error_code(s: String) -> Vec<String> {
    get_error_text(s, false)
}

fn get_error_text(s: String, with_error: bool) -> Vec<String> {
    let mut descriptions: Vec<String> = Vec::new();
    let errors = structure_compiler_output(s);
    for entry in errors.into_iter() {
        let temp: Vec<&str> = entry.split(NEWLINE).collect();
        descriptions.push(cut_out(String::from(temp[FIRST_LINE]), with_error));
    }
    descriptions
}

/// Returns a Vector of compiler error entries
/// 
/// # Arguments
/// * `s` - The whole Compilererror output as String

pub fn structure_compiler_output(output: String) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    let re = Regex::new(r"(?m)error\[E\d{4}\]:").unwrap();
    for entry in output.split("\n\n") {
        let stringfied_entry = String::from(entry);
        if re.is_match(stringfied_entry.as_str()) {
            //check if certain error code is found
            errors.push(stringfied_entry);
        }
    }
    if errors.is_empty() && re.is_match(output.as_str()) {
        return vec![output];
    } else {
        errors
    }
}

fn cut_out(i: String, with_error: bool) -> String {
    let mut erg = String::new();
    let re = Regex::new(r"(?m)<(?m).*>").unwrap();
    let p = re.replace_all(i.as_str(), ""); //remove typedefs like <std::ops::...>
    let mut words = p.split_whitespace().collect::<Vec<&str>>();
    let mut error: String = String::from("");
    words.pop();
    error.push_str(words.remove(0)); //remove error code
    error.push(WHITESPACE);


    for s in words.into_iter() {
        let mut word = s.chars()
            .filter(|c| (*c).is_alphabetic())
            .collect::<String>();
        if !word.is_empty() {
            erg.push_str(String::from(word).trim());
            erg.push(WHITESPACE);
        }
    }

    if with_error {
        return {
            let mut x = error;
            x.push_str(erg.as_str());
            String::from(x.trim())
        };
    } else {
        String::from(erg.trim())
    }
}
