// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

// I AM NOT DONE

#[cfg(test)]
mod tests {
    use std::{num::ParseIntError, fmt};

    type Result<T> = std::result::Result<T, ParseStringError>;

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct ParseStringError;
    
    impl fmt::Display for ParseStringError {
        fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
            write!(f, "invalid digit found in string")
        }
    }
    

    pub fn total_cost(item_quantity: &str) -> Result<i32> {
        let processing_fee = 1;
        let cost_per_item = 5;
        match item_quantity.parse::<i32>() {
            Ok(qty) => Ok(qty * cost_per_item + processing_fee),
            Err(_err) => Ok(0)
        }
    }    

    #[test]
    fn test_error() {
        let mut tokens = 100;
        let pretend_user_input = "8";
    
        let cost = total_cost(pretend_user_input);
        let my_cost = cost.unwrap_or_default();

        if  my_cost > tokens {
            println!("You can't afford that many!");
        } else {
            tokens -= my_cost;
            println!("You now have {} tokens.", tokens);
        }
    }

}
