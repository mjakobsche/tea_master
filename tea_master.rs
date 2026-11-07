use std::time::{Duration, Instant};
use std::thread;
use std::io::Write;
use std::io;

fn main() {

    // Set default times for a brewing session
    // by filling 'defaults' vector with time (in seconds);
    // each brewing will use next value from the vector;
    //
    // for example:
    //
    // println!("DARK OOLONG BREWING");
    // let mut defaults: Vec<u64> = vec![70, 50, 80, 100, 130, 170, 225, 270];
    //
    // or:
    //
    // println!("HIGH MOUNTAIN OOLONG BREWING");
    // let mut defaults: Vec<u64> = vec![80, 50, 80, 120, 150, 210];
    // or just leave it empty - without defaults:
    // 
    let mut defaults: Vec<u64> = vec![];

    defaults.reverse();
    let mut time = get_time(&get_or_zero(defaults.pop()));
    let mut counter = 1;
    while time > 0 {
        wait(&time);
        let prompt = format!("Review (#{}): ", counter);
        read(&prompt);
        time = get_time(&get_or_zero(defaults.pop()));
        counter = counter + 1;
    }
    print!("bye");
}

fn get_time(default: &u64) -> u64 {
    let prompt = format!("Enter time to wait (0 to quit) [{}]: ", default);
    let raw_input = read(&prompt);
    let input = raw_input.trim().to_string();
    if input.is_empty() {
        *default 
    } else {
        handle_input(&input, &default)
    }
}

fn handle_input(input: &String, default: &u64) -> u64 {
    match input.parse() {
        Ok(time) => {
            if time > 600 {
                println!("Please input a smaller number");
                get_time(&default)
            } else {
                time
            }
        },
        Err(_) => {
            println!("Please input a number");
            get_time(&default)
        }
    }
}

fn read(message: &str) -> String {
    gray(message);
    flush();
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Error getting value");
    return (&* val).to_string()
}

fn wait(time: &u64) {
    let now = Instant::now();
    let mut counter = now.elapsed().as_secs();
    while counter <= *time {
        print!("\r{elapsed}/{total} ", elapsed=counter, total=*time);
        flush();
        sleep(&1);
        counter = now.elapsed().as_secs();
    }
    green("ready!");
    println!();
}

fn get_or_zero(number: Option<u64>) -> u64 {
    match number {
        Some(number) => number,
        None => 0
    }
}

fn green(message: &str) {
    print!("\x1b[92m{}\x1b[0m", message);
}

fn gray(message: &str) {
    print!("\x1b[90m{}\x1b[0m", message);
}

fn sleep(time: &u64) {
    thread::sleep(Duration::new(*time, 0));
}

fn flush() {
    io::stdout().flush().unwrap();
}
