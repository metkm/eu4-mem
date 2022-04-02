mod offsets;
mod process;

use offsets::TechOffsets;
use process::{add_offsets, get_game, get_value, Game};

use std::{
    thread::sleep,
    time::{Duration},
};

fn delay() {
    sleep(Duration::from_millis(500));
}

fn main() {
    let tech_offsets = TechOffsets::default();

    'process: loop {
        if let Some(game) = get_game() {
            let Game { handle, address } = game;
            
            let tech_base = address + 0x02420FC8;
            let admin_tech_address = add_offsets(&handle, &tech_base, &tech_offsets.admin);
            let diplo_tech_address = add_offsets(&handle, &tech_base, &tech_offsets.diplo);
            let military_tech_address = add_offsets(&handle, &tech_base, &tech_offsets.military);
            
            'game: loop {
                if handle.is_invalid() {
                    println!("Game process is lost!");
                    break 'process;
                }

                let admin = get_value(&handle, &admin_tech_address);
                if admin < 2 { // game is not loaded yet. Just skip it.
                    continue;
                }

                println!("Admin Tech = {}", get_value(&handle, &admin_tech_address));
                println!("Diplo Tech = {}", get_value(&handle, &diplo_tech_address));
                println!("Military Tech = {}", get_value(&handle, &military_tech_address));

                delay();
            }
        } else {
            println!("Can't find eu4.exe process! Retrying..");
        }

        delay();
    }
}
