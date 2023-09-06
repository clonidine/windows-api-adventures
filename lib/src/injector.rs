use std::ffi::c_void;
use std::ffi::CString;

use windows::core::Result;

use crate::process::get_process_name;
use crate::{
    memory::{alloc_mem, write_process_memory},
    process::{close_handle, get_process_handle},
};

pub fn inject(content: &str, pid: u32) -> Result<()> {
    let handle = get_process_handle(pid, true)?;

    let mem_addr = alloc_mem(&handle, content)?;

    let c_str = CString::new(content);

    match c_str {
        Ok(c_str) => {
            let buffer = c_str.as_ptr() as *const c_void;

            let mem_written = write_process_memory(&handle, mem_addr, buffer, content.len(), None);

            match mem_written {
                Ok(_) => {
                    let process_name = get_process_name(pid)?;

                    println!("---------------------------");
                    println!("Process memory written!");
                    println!("Memory Address: {:p}", mem_addr);
                    println!("PID: {}", pid);
                    println!("Process Name: {}", process_name);
                    println!("---------------------------");
                }

                Err(e) => panic!("Cannot write process memory: {}", e),
            }
        }

        Err(e) => panic!("Cannot convert content to CString: {}", e),
    }

    let handle_closed = close_handle(handle);

    match handle_closed {
        Ok(_) => Ok(()),
        Err(e) => panic!("Cannot close handle: {}", e),
    }
}
