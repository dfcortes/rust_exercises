// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!


// I AM NOT DONE

use std::fmt::{self, write, Error};

type Result<T> = std::result::Result<T, EmptyStringError>;

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyStringError;

impl fmt::Display for EmptyStringError {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f, "`name` was empty; it must be nonempty.")
    }
}

impl From<&str> for EmptyStringError {
    fn from(_: &str) -> Self {
        EmptyStringError
    }
}

pub fn generate_nametag_text(name: String) -> Result<String> {

    if name.is_empty() {
        Err(EmptyStringError)
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
