extern crate winapi;

use winapi::shared::minwindef::HMODULE;
use winapi::shared::ntdef::HANDLE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameA};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

use std::io;
use std::mem::size_of;
use std::ptr::null_mut as null;

/* Represents a process with its PID and name */
#[derive(Debug)]
struct Process {
    pid: u32,
    name: String,
}

struct ProcessHandle {
    _pid: u32,
    handle: HANDLE,
}

impl ProcessHandle {
    /* Opens a handle to the process with the given PID */
    pub fn open(pid: u32) -> Self {
        Self {
            _pid: pid,
            handle: unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) },
        }
    }

    /* This function enumerates the process modules and gets the name of the first (base) module */
    pub fn name(&self) -> io::Result<String> {
        let mut lphmodule: Vec<HMODULE> = vec![null() as HMODULE; 1024];
        let mut lpcbneeded: u32 = 0;

        /* Enumerate the modules loaded in the process */
        let result = unsafe {
            EnumProcessModules(
                self.handle,
                lphmodule.as_mut_ptr(),
                (lphmodule.len() * size_of::<HMODULE>()) as u32,
                &mut lpcbneeded,
            )
        };
        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        /* Calculate the number of modules returned */
        let count = (lpcbneeded as usize) / size_of::<HMODULE>();
        if count == 0 {
            return Err(io::Error::last_os_error());
        }

        /* Get the base module handle (first one) */
        let hmodule = lphmodule[0];

        let mut lpbasename = vec![0u8; 256];
        let buffer = unsafe {
            GetModuleBaseNameA(
                self.handle,
                hmodule,
                lpbasename.as_mut_ptr() as *mut i8,
                lpbasename.len() as u32,
            )
        };
        if buffer == 0 {
            return Err(io::Error::last_os_error());
        }

        /* Set the length of the name buffer to the actual length returned */
        unsafe {
            lpbasename.set_len(buffer as usize);
        }

        /* Return the process name as a String */
        Ok(String::from_utf8_lossy(&lpbasename).to_string())
    }
}

/* Automatically closes the process handle when the ProcessHandle goes out of scope */
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}

/* Opens each process, retrieves its name, and collects them into a Vec<Process> */
fn get_procs_names() -> io::Result<Vec<Process>> {
    let mut processes: Vec<Process> = Vec::new();
    if let Ok(pids) = get_all_procs() {
        for pid in pids {
            let hproc = ProcessHandle::open(pid);
            if hproc.handle.is_null() {
                continue; /* Skip if unable to open handle (e.g., access denied) */
            }

            if let Ok(name) = hproc.name() {
                processes.push(Process { pid, name });
            }
        }
    }
    Ok(processes)
}

/* Enumerates all process IDs on the system and return a Vec<u32> if success */
fn get_all_procs() -> io::Result<Vec<u32>> {
    let mut lpidprocess: Vec<u32> = vec![0; 1024];
    let mut lpcbneeded: u32 = 0;

    /* Enumerate all processes */
    let result = unsafe {
        EnumProcesses(
            lpidprocess.as_mut_ptr(),
            (lpidprocess.len() * size_of::<u32>()) as u32,
            &mut lpcbneeded,
        )
    };

    if result == 0 {
        return Err(io::Error::last_os_error());
    }

    /* Calculate the number of processes and truncate the vector */
    let buffer = (lpcbneeded as usize) / size_of::<u32>();
    lpidprocess.truncate(buffer);
    Ok(lpidprocess)
}

fn main() {
    /* Get the list of processes and handle any errors */
    match get_procs_names() {
        Ok(processes) => {
            /* Print the processes ID and name in a clean format */
            for (index, proc) in processes.iter().enumerate() {
                println!("({:^3}) PID: {:<5} | Name: {}", index, proc.pid, proc.name);
            }
        }
        Err(e) => {
            eprintln!("Error enumerating processes: {}", e);
        }
    }
}
