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
            3 => check_off(&mut list),
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

fn check_off(list: &mut Vec<String>) {
    print!("\x1B[2J");
    render_list(&list);
    print!("\n");
    println!("Enter the number you'd like to check off:");
    let user_option: i32 = read!();
    list.remove((user_option - 1).try_into().unwrap());
}
