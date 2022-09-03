use std::fmt;

#[derive(Debug)]
struct Combination<'a> {
    elements: &'a Vec<char>
}

impl fmt::Display for Combination<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Elements: {:?}", self.elements)
    }
}

fn do_all_work(result: &mut [char; 16]) {

    let list_of_characters = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'Ñ', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut i: i32 = 0; 
    while i < 16 {
        let combination = Combination {elements: &list_of_characters};
        println!("{:?}", combination);
        i = i + 1;
    }
}

//fn change()

fn main() {
    let mut result = [
        'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A',
    ];
    do_all_work(&mut result);

}
