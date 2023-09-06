mod injector;

use injector::inject;
use windows::core::Result;

fn main() -> Result<()> {
    let pid = get_pid();
    let content = get_content();

    inject(&content, pid)
}

fn get_pid() -> u32 {
    let mut pid = String::new();

    println!("Enter PID: ");
    std::io::stdin()
        .read_line(&mut pid)
        .expect("Failed to read line");

    let pid: u32 = pid.trim().parse().expect("Please type a number!");

    pid
}

fn get_content() -> String {
    let mut content = String::new();

    println!("Enter content: ");
    std::io::stdin()
        .read_line(&mut content)
        .expect("Failed to read line");

    content
}
