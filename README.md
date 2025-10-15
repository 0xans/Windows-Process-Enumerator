# Windows Process Enumerator

A simple Rust program to enumerate running processes on a Windows system, retrieving their process IDs (PIDs) and executable names using the Windows API.

## Prerequisites
- Add the following dependency to your `Cargo.toml`:
  ```toml
  [dependencies]
  winapi = { version = "0.3", features = ["minwindef", "ntdef", "psapi", "processthreadsapi", "winnt", "handleapi"] }
  ```

## Usage
1. Clone the repository or copy the source code into a Rust project.
2. Run the program using:
   ```bash
   cargo run
   ```
3. The program will output a list of processes in the format:
   ```
   (index) PID: <pid> | Name: <executable_name>
   ```

## Example Output
```
(  0) PID: 4     | Name: System
(  1) PID: 392   | Name: svchost.exe
(  2) PID: 648   | Name: explorer.exe
...
```

## Code Structure
- `Process`: A struct to store a process's PID and name.
- `ProcessHandle`: A struct that manages a Windows process handle, with methods to open a process and retrieve its name. Implements `Drop` for automatic handle cleanup.
- `get_all_procs`: Enumerates all process IDs on the system.
- `get_procs_names`: Retrieves the names of all processes and returns a `Vec<Process>`.
- `main`: Orchestrates the process enumeration and prints the results.

## Limitations
- Windows-specific due to reliance on `winapi`.
- Hardcoded buffer sizes (1024 for processes/modules, 256 for names) may not suffice for systems with many processes or long module names.
- Skips processes that cannot be accessed due to permissions.
- List a very simple informatin about the process.
