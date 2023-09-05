use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK},
};

pub unsafe fn create_message_box(title: &str, message: &str) {
    MessageBoxW(
        None,
        PCWSTR::from_raw(message.encode_utf16().collect::<Vec<_>>().as_ptr()),
        PCWSTR::from_raw(title.encode_utf16().collect::<Vec<_>>().as_ptr()),
        MB_OK | MB_ICONINFORMATION,
    );
}
