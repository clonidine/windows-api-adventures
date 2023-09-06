const ACCESS_RIGHTS: u32 = 0x1F0FFF;

use windows::{
    core::Result,
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
    },
};

pub fn get_process_handle(process_id: u32, binherinthandle: bool) -> Result<HANDLE> {
    unsafe {
        OpenProcess(
            PROCESS_ACCESS_RIGHTS(ACCESS_RIGHTS),
            binherinthandle,
            process_id,
        )
    }
}

pub fn close_handle(handle: HANDLE) -> Result<()> {
    unsafe { CloseHandle(handle) }
}
