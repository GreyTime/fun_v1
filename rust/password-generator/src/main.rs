use rand;
use text_io::read;

fn main() {
    println!("Welcome to the password  generator!");
    println!("Please input your desired length: ");
    let input_length: i32 = read!();
    println!("Would you like to include special characters? (y/n)");
    let special_characters_input: char = read!();
    let special_character: bool;

    if special_characters_input == 'y' {
        special_character = true
    } else {
        special_character = false
    }

    


}
