use std::mem::{size_of, size_of_val};
use std::ffi::c_void;

use windows::Win32::{
    Foundation::{HANDLE, HINSTANCE},
    System::{
        Diagnostics::{
            Debug::ReadProcessMemory,
            ToolHelp::{
                CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32,
                TH32CS_SNAPMODULE,
            },
        },
        ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExW},
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

pub fn get_pids() -> Vec<u32> {
    let mut pids: Vec<u32> = Vec::with_capacity(1024);
    let mut pids_count = 0;

    unsafe {
        K32EnumProcesses(
            pids.as_mut_ptr(),
            pids.capacity() as u32 * size_of::<u32>() as u32,
            &mut pids_count,
        );
        let pids_count = pids.capacity() * size_of::<u32>();
        pids.set_len(pids_count);
    }

    pids
}

pub fn get_handle(pid: u32) -> Option<HANDLE> {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) };

    if handle.is_invalid() {
        None
    } else {
        Some(handle)
    }
}

pub fn get_module_file_name(handle: HANDLE, instance: HINSTANCE) -> String {
    let mut name: [u16; 120] = [0; 120];

    unsafe {
        K32GetModuleFileNameExW(handle, instance, &mut name);
    }

    return String::from_utf16(&name).unwrap();
}

pub fn handle_name_contains(handle: HANDLE, title: &str) -> bool {
    let name = get_module_file_name(handle, HINSTANCE::default());

    if name.contains(title) {
        true
    } else {
        false
    }
}

pub fn get_base_address(process_id: u32, title: &str) -> Option<usize> {
    let mut module_entry = MODULEENTRY32::default();
    module_entry.dwSize = size_of::<MODULEENTRY32>() as u32;

    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process_id);

        if handle.is_invalid() {
            println!("Snapshot error!");
            return None;
        }

        if !Module32First(handle, &mut module_entry).as_bool() {
            println!("First module not found!");
            return None;
        }

        loop {
            let char_array: Vec<u8> = module_entry.szModule.iter().map(|x| x.0).collect();
            let name = String::from_utf8_lossy(&char_array);

            if name.contains(title) {
                return Some(module_entry.modBaseAddr as usize);
            }

            if !Module32Next(handle, &mut module_entry).as_bool() {
                break;
            };
        }

        return None;
    }
}

pub fn read_process(handle: HANDLE, address: usize, buff: &mut [u8]) {
    let address_ptr = address as *const i64;
    let read = 0;

    unsafe {
        ReadProcessMemory(
            handle,
            address_ptr as *const c_void,
            buff as *mut _ as *mut c_void,
            size_of_val(&buff),
            read as *mut usize
        );
    }
}

pub fn get_base_point(handle: HANDLE, address: usize) -> i64 {
    let mut buffer: [u8; 8] = [0; 8];
    read_process(
        handle,
        address,
        &mut buffer
    );

    let parsed = i64::from_le_bytes(buffer);
    parsed
}

pub fn start() {
    for pid in get_pids() {
        let handle = if let Some(handle) = get_handle(pid) {
            handle
        } else {
            continue;
        };
        if handle_name_contains(handle, "eu4.exe") {
            let base = get_base_address(pid, "eu4.exe");
            if let Some(base_address) = base {
                let base_point = get_base_point(handle, base_address + 0x2420DA8) as usize;

                let mut year_buff: [u8; 4] = [0; 4];
                read_process(handle, base_point + 0x1DD4, &mut year_buff);

                println!("{}", i32::from_le_bytes(year_buff));
            }
        }
    }
}
