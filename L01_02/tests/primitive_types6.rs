#[cfg(test)]
mod tests {
    //use core::num;

    #[test]
    fn test_primitive_types6() {
        let numbers = (1, 2, 3);
        // Replace below ??? with the tuple indexing syntax.
        let second = numbers.1;
    
        assert_eq!(2, second,
            "This is not the 2nd number in the tuple!")
    }
}