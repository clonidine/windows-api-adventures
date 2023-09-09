use eyre::Result;
use std::{ffi::c_void, mem::transmute};
use windows::Win32::{
    Foundation::{GetLastError, HANDLE},
    Security::SECURITY_ATTRIBUTES,
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        Memory::{VirtualAllocEx, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE},
        Threading::{CreateRemoteThreadEx, LPPROC_THREAD_ATTRIBUTE_LIST, LPTHREAD_START_ROUTINE},
    },
};

pub fn virtual_alloc_ex(
    handle: &HANDLE,
    lp_address: Option<*const c_void>,
    dw_size: usize,
    fl_allocation_type: VIRTUAL_ALLOCATION_TYPE,
    fl_protect: PAGE_PROTECTION_FLAGS,
) -> Result<*mut c_void> {
    unsafe {
        let handle = match handle.is_invalid() {
            true => panic!("Cannot alloc virtual ex: {:?}", GetLastError()),
            false => handle,
        };

        Ok(VirtualAllocEx(
            *handle,
            lp_address,
            dw_size,
            fl_allocation_type,
            fl_protect,
        ))
    }
}

pub fn create_remote_thread(
    handle: HANDLE,
    lp_thread_attributes: Option<*const SECURITY_ATTRIBUTES>,
    dw_stack_size: usize,
    lp_start_address: LPTHREAD_START_ROUTINE,
    lp_thread_parameter: Option<*const c_void>,
    dw_creation_flags: u32,
    lp_attribute_list: LPPROC_THREAD_ATTRIBUTE_LIST,
    mut pid: u32,
) -> Result<HANDLE> {
    unsafe {
        match handle.is_invalid() {
            true => Err(std::io::Error::last_os_error().into()),
            false => {
                let remote_thread = CreateRemoteThreadEx(
                    handle,
                    lp_thread_attributes,
                    dw_stack_size,
                    lp_start_address,
                    lp_thread_parameter,
                    dw_creation_flags,
                    lp_attribute_list,
                    Some(&mut pid as *mut u32),
                );

                match remote_thread.is_err() {
                    true => Err(std::io::Error::last_os_error().into()),
                    false => Ok(remote_thread?),
                }
            }
        }
    }
}

pub fn transmute_value(value: *mut c_void) -> LPTHREAD_START_ROUTINE {
    type Src = *mut c_void;
    type Dst = unsafe extern "system" fn(*mut c_void) -> u32;

    unsafe { Some(transmute::<Src, Dst>(value)) }
}

pub fn write_process_memory(
    handle: &HANDLE,
    base_address: *mut c_void,
    buffer: *const c_void,
    size: usize,
    bytes_written: Option<*mut usize>,
) -> Result<()> {
    unsafe {
        Ok(WriteProcessMemory(
            *handle,
            base_address,
            buffer,
            size,
            bytes_written,
        )?)
    }
}
