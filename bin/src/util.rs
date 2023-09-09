use std::ffi::c_void;

pub fn info(process_name: String, mem_address: *mut c_void) {
    println!();
    println!("---------- PROCESS INFORMATION ----------");
    println!("Process Name: {}", process_name);
    println!("Base Address: {:p}", mem_address);
    println!("---------------------------");
    println!();
}
