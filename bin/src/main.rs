use lib::create_message_box;
use windows::core::Result;

fn main() -> Result<()> {
    create_message_box("Bitch", "Message")
}
