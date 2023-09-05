use windows::{
    core::Result,
    Win32::System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
};

const ACCESS_RIGHTS: u32 = 0x1F0FFF;

pub fn open_process(process_id: u32) -> Result<()> {
    let handle = unsafe { OpenProcess(PROCESS_ACCESS_RIGHTS(ACCESS_RIGHTS), false, process_id) };

    match handle {
        Ok(process) => {
            println!("Process handle: {:?}", process);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    Ok(())
}
