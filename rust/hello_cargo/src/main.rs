use text_io::read;
fn main() {
    let mut list: Vec<String> = vec![];
    println!("Welcome to the Todo-List!");
    loop {
        print!("\n");
        println!(
            "What would you like to do? 
                \n 1: show list, 2: add item, 3: check off item, 4: exit"
        );
        let user_option: i32 = read!();
        println!("{user_option}");
        match user_option {
            1 => render_list(&list),
            2 => add_entry(&mut list),
            3 => print!("check off"),
            4 => std::process::exit(0),
            _ => println!("That is not a valid input!"),
        }
    }
}

fn render_list(list: &Vec<String>) {
    print!("\x1B[2J");
    let mut line_counter = 1;
    for entry in list {
        print!("{line_counter}. {entry} \n");
        line_counter += 1;
    }
    print!("\n");
}

fn add_entry(list: &mut Vec<String>) {
    print!("\x1B[2J");
    println!("What would you like to add: ");
    let user_entry: String = read!();
    list.push(user_entry);
    print!("\n");
}
