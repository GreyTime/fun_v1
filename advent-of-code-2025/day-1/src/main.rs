use std::fs;

fn main() {

    const DIAL_UPPER_LIMIT:i32 = 99;
    const DIAL_LOWER_LIMIT:i32 = 0;

    const DIAL_STARTING_POINT:i32 = 50;

    let mut dial_location = DIAL_STARTING_POINT;
    let mut dial_at_0_count = 0;

    let input = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    let split_input = input.lines();
    
    for element in split_input {
        let direction = element.chars().next().unwrap();
        let amount:i32 = element[1..].trim().parse::<i32>().unwrap();

        match direction {
            'R' => dial_location += amount,
            'L' => dial_location -= amount,
            _   => println!("Direction was not L or R")
        }

        if dial_location > DIAL_UPPER_LIMIT {
            dial_location = dial_location % 100
        } else if dial_location < DIAL_LOWER_LIMIT {
            dial_location = 0 - (dial_location.abs() % 100)
        }

        if dial_location == 0 {
            dial_at_0_count += 1
        }
    }

    println!("Dial has reached zero {dial_at_0_count} times!");
}