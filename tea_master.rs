use std::time::{Duration, Instant};
use std::thread;
use std::io::Write;
use std::io;

fn main() {
    let mut numbers: Vec<u64> = vec![];
    numbers.reverse();
    if numbers.is_empty() {
        let mut time = get_time();
        while time > 0 {
            wait(&time);
            read("Review: ");
            time = get_time();
        }
    } else {
        while let Some(number) = numbers.pop() {
            gray("Press enter to wait ");
            print!("{}s ", number);
            flush();
            let mut val = String::new();
            io::stdin().read_line(&mut val).expect("Error getting guess");
            wait(&number);
        }
    }
    print!("bye");
}

fn get_time() -> u64 {
    match read("Enter time to wait (0 to quit): ").trim().parse() {
        Ok(time) => {
            if time > 600 {
                println!("Please input a smaller number");
                get_time()
            } else {
                time
            }
        },
        Err(_) => {
            println!("Please input a number");
            get_time()
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
