use windows::{
    core::Result,
    Win32::System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
};

pub fn open_process(process_id: u32) -> Result<()> {
    let process = unsafe { OpenProcess(PROCESS_ACCESS_RIGHTS(0x1F0FFF), false, process_id) };

    match process {
        Ok(process) => {
            println!("Process handle: {:?}", process);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    Ok(())
}
