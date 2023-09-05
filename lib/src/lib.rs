use windows::{
    core::h,
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
};

pub fn create_message_box() {
    unsafe {
        MessageBoxW(None, h!("Hello!"), h!("Hi!"), MB_OK);
    }
}
