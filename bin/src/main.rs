mod injector;
mod util;

use eyre::Result;
use injector::inject;

fn main() -> Result<()> {
    let pid = get_pid();

    inject(pid)
}

fn get_pid() -> u32 {
    let arguments = std::env::args().collect::<Vec<String>>();

    if arguments.len() != 2 {
        println!("Usage: {} <pid>", arguments[0]);
        std::process::exit(1);
    }

    let pid = arguments[1].parse::<u32>().unwrap();

    println!("[+] PID: {}", pid);

    pid
}
