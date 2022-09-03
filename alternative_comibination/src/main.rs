use std::{fmt, convert::TryInto};

#[derive(Debug)]
struct Combination<'a>{
    result: &'a mut [char; 16]
}

impl fmt::Display for Combination<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Elements: {:?}", self.result)
    }
}

fn swap(result: &mut [char; 16], k: i32, y: i32) {
    let temp_k: char = result[k as usize];
    result[k as usize] = result[y as usize];
    result[y as usize] = temp_k;
}

fn do_all_work(result: &mut [char; 16]) {

    let list_of_characters = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'Ñ', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut indexes = vec![0; 16];
    
    println!(
        "{:?}",
        Combination {result: result});
    
    let mut i: i32 = 0; 
    
    while i < 5 {//list_of_characters.len().try_into().unwrap() {
        if indexes[i as usize] < i { 
            if i % 2 == 0 {
                swap(result, 0, i);
            } else {
                swap(result, indexes[i as usize], i)
            }
            println!(
              "preview {:?}",
                Combination {result: result});

            indexes[i as usize] = indexes[i as usize] + 1;
            i = 0;
        } else {
            indexes[i as usize] = 0;
            i = i + 1;
        }
    }

    
}

//fn change()

fn main() {
    let mut result = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    ];
    do_all_work(&mut result);

}
