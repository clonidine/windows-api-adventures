use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK},
};

pub unsafe fn create_message_box(title: &str, message: &str) {
    MessageBoxW(
        None,
        PCWSTR::from_raw(message.as_ptr() as *const u16),
        PCWSTR::from_raw(title.as_ptr() as *const u16),
        MB_OK | MB_ICONINFORMATION,
    );
}
