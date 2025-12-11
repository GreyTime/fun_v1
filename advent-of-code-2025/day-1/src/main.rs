use std::fs;

fn main() {

    let mut dial_location: i32 = 50;
    let mut dial_0_count = 0;

    let input = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    for element in input.lines() {
        let direction = element.chars().next().unwrap();
        let amount: i32 = element[1..].trim().parse::<i32>().unwrap();

        for _ in 0..amount {
            if direction == 'R' {
                dial_location = (dial_location + 1) % 100;
            } else {
                dial_location = (dial_location + 99) % 100;
            }

            if dial_location == 0 {
                dial_0_count += 1;
            }
        }

        println!("- the dial is rotated {element} to point at {dial_location}.");
    }

    println!("Dial has pointed at zero {dial_0_count} times!");
}
