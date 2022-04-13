use windows::Win32::{
    Foundation::{HANDLE},
};

use crate::process::get_value;

pub fn read_string(handle: &HANDLE, mut address: usize) -> String {
    let mut curr = get_value(handle, &address) as u8;
    let mut name = String::with_capacity(3);

    while curr != 0 {
        name.push(curr as char);
        
        address += 0x1;
        curr = get_value(handle, &address) as u8;
    }

    name
}
