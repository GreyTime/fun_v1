use std::{thread, time::Duration};

use text_io::read;

fn main() {
    println!("Please input the number of minutes you'd like to focus: ");
    let input: u64 = read!();
    let mut duration = Duration::from_mins(input);
    let duration_mins = duration.as_secs() / 60;
    println!("{duration_mins} minute timer is set");
    
    while duration >= Duration::from_secs(1) {
        print_time(duration);
        thread::sleep(Duration::from_secs(1));
        duration -= Duration::from_secs(1);
    }
    println!("You are done!!");
} 


fn print_time(duration: Duration) {
    let mins = duration.as_secs() / 60;
    let secs = duration.as_secs() % 60;
    println!("{mins}:{secs}");
}