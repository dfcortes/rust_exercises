// Don't mind this for now :)
// I AM NOT DONE

#[cfg(test)]
mod tests {

    pub fn call_me() {
        println!("Hola Curso de Rust")
    }
    
    #[test]
    fn call_function() {
        call_me();
    }

}