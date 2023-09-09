use windows::{
    core::{Result, PWSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Threading::{
            OpenProcess, QueryFullProcessImageNameW, PROCESS_ACCESS_RIGHTS, PROCESS_NAME_FORMAT,
        },
    },
};

pub fn get_process_handle(
    pid: u32,
    b_inherint_handle: bool,
    dw_desired_access: PROCESS_ACCESS_RIGHTS,
) -> Result<HANDLE> {
    unsafe { OpenProcess(dw_desired_access, b_inherint_handle, pid) }
}

pub fn get_process_name(
    pid: u32,
    b_inherint_handle: bool,
    dw_desired_access: PROCESS_ACCESS_RIGHTS,
) -> Result<String> {
    let handle = get_process_handle(pid, b_inherint_handle, dw_desired_access)?;

    let mut buffer = [0u16; 260];

    unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(buffer.as_mut_ptr()),
            &mut (buffer.len() as u32),
        )
    }?;

    let process_name = String::from_utf16_lossy(&buffer);
    let process_name = process_name.split("\\").last().unwrap().to_string();

    Ok(process_name)
}

pub fn close_handle(handle: HANDLE) -> Result<()> {
    unsafe { CloseHandle(handle) }
}
