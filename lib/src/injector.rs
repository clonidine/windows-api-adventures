use std::ffi::c_void;
use std::ffi::CString;

use windows::core::Result;

use crate::{
    memory::{alloc_mem, write_process_memory},
    process::{close_handle, get_process_handle},
};

pub fn inject(content: &str, pid: u32) -> Result<()> {
    let handle = get_process_handle(pid, true)?;

    let mem_addr = alloc_mem(&handle, content)?;

    let content_buffer = CString::new(content);

    match content_buffer {
        Ok(content_buffer) => {
            
            let mem_written = write_process_memory(
                &handle,
                mem_addr,
                content_buffer.as_ptr() as *const c_void,
                content.len(),
                None,
            );

            match mem_written {
                Ok(_) => {
                    println!("Process memory written!");
                    println!("Memory Address: {:p}", mem_addr);
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
