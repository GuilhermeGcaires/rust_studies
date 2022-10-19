use std::io;

fn main() {
    let mut input = String::new();
    let mut final_string = String::new();

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    io::stdin().read_line(&mut input).expect("Invalid input");
    input = input.to_lowercase().trim().to_string();

    let first_letter = input.chars().next().unwrap();

    if vowels.contains(&first_letter) {
        final_string = format!("{}-hay", &input); 
    } else {
        for(i, char) in input.char_indices() {
            if i == 0 {
                continue;
            } else {
                final_string.push(char);
            }
        }
        final_string = format!("{}-{}ay", final_string, first_letter);
    }
    println!("{}", final_string);
}
