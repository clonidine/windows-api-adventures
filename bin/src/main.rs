use lib::open_process;
use windows::core::Result;

fn main() -> Result<()> {
    open_process(12532)
}
