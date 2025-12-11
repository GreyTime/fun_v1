use rand::{self, random_range};
use std::cmp::Ordering;
use std::time::Instant;
use text_io::read;

fn main() {
    println!("Welcome to the guessing game!");
    println!("How many times would you like to run the simulation: ");
    let number_of_sims: u64 = read!(); // ← jetzt u64, safe für große Zahlen

    let start = Instant::now();
    let mut try_sum: u64 = 0; // ← u64 für präzise Summen

    for _ in 0..number_of_sims {
        let target_number = random_range(1..101);
        let mut tries: u64 = 0;

        let mut low = 1;
        let mut high = 100;

        loop {
            tries += 1;

            let guess = low + (high - low) / 2; // ← präzise Mitte

            match guess.cmp(&target_number) {
                Ordering::Equal => break,
                Ordering::Greater => high = guess - 1,
                Ordering::Less => low = guess + 1,
            }
        }

        try_sum += tries;
    }

    let duration = start.elapsed();

    println!(
        "The average number of tries is {:.4}",
        try_sum as f64 / number_of_sims as f64 // ← als f64 casten für Präzision
    );
    println!("This took {:.3} seconds", duration.as_secs_f32());
}
