// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?


// I AM NOT DONE

fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        String::from(x)
    } else {
        String::from(y)
    }
}

#[cfg(test)]
mod tests {

    use longest;

    #[test]
    fn test_lifetime() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);
    }
}