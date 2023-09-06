use std::ffi::c_void;
use std::ffi::CString;

use windows::core::Result;

use crate::{
    memory::{alloc_mem, write_process_memory},
    process::{close_handle, get_process_handle},
};

pub fn inject(content: &str, pid: u32) -> Result<()> {
    let handle = get_process_handle(pid, true).unwrap();

    let alloc = alloc_mem(&handle, content);

    match alloc {
        Ok(alloc) => {
            let content_buffer = CString::new(content).unwrap();

            let writed = write_process_memory(
                &handle,
                alloc,
                content_buffer.as_ptr() as *const c_void,
                content.len(),
                None,
            );

            match writed {
                Ok(_) => {
                    println!("Process memory written!");
                    println!("Memory Address: {:p}", alloc);
                }
                
                Err(e) => panic!("Cannot write process memory: {}", e),
            }
        }

        Err(e) => panic!("Cannot alloc memory: {}", e),
    }

    let handle_closed = close_handle(handle);

    match handle_closed {
        Ok(_) => Ok(()),
        Err(e) => panic!("Cannot close handle: {}", e),
    }
}
