// HANDLE CreateRemoteThread(
//   [in]  HANDLE                 hProcess,
//   [in]  LPSECURITY_ATTRIBUTES  lpThreadAttributes,
//   [in]  SIZE_T                 dwStackSize,
//   [in]  LPTHREAD_START_ROUTINE lpStartAddress,
//   [in]  LPVOID                 lpParameter,
//   [in]  DWORD                  dwCreationFlags,
//   [out] LPDWORD                lpThreadId
// );

use std::ffi::c_void;
use windows::{
    core::Result,
    Win32::{
        Foundation::HANDLE,
        Security::SECURITY_ATTRIBUTES,
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            Memory::{VirtualAllocEx, MEM_COMMIT, PAGE_EXECUTE_READWRITE},
            Threading::{CreateRemoteThread, LPTHREAD_START_ROUTINE},
        },
    },
};

pub fn alloc_mem(handle: &HANDLE, content: &str) -> Result<*mut c_void> {
    unsafe {
        Ok(VirtualAllocEx(
            *handle,
            None,
            content.len(),
            MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        ))
    }
}

pub fn create_remote_thread(
    handle: &HANDLE,
    lpthreadattributes: Option<*const SECURITY_ATTRIBUTES>,
    dwstacksize: usize,
    lpstartaddress: LPTHREAD_START_ROUTINE,
    lpthreadparameter: Option<*const c_void>,
    dwcreationflags: u32,
    lpthreadid: Option<*mut u32>,
) -> Result<HANDLE> {
    unsafe {
        assert!(!handle.is_invalid());

        CreateRemoteThread(
            *handle,
            lpthreadattributes,
            dwstacksize,
            lpstartaddress,
            lpthreadparameter,
            dwcreationflags,
            lpthreadid,
        )
    }
}

pub fn write_process_memory(
    handle: &HANDLE,
    baseaddress: *mut c_void,
    buffer: *const c_void,
    size: usize,
    numberofbyteswritten: Option<*mut usize>,
) -> Result<()> {
    unsafe { WriteProcessMemory(*handle, baseaddress, buffer, size, numberofbyteswritten) }
}
