use lib::injector::inject;
use windows::core::Result;

const PID: u32 = 0;

fn main() -> Result<()> {
    let injected = inject("Lorde best friend", PID); // bytes: 17

    match injected {
        Ok(_) => {
            println!("PID: {}", PID);
        }
        Err(e) => panic!("{}", e),
    }

    Ok(())
}
