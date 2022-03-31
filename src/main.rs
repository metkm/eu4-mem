mod process;
mod offsets;

use std::{
    time::Duration,
    thread::sleep
};
use process::{get_game, add_offsets, Game, get_value};
use offsets::{Tech};

fn delay() {
    sleep(Duration::from_millis(500));
}

fn main() {
    let tech_offsets = Tech::new();

    'proc: loop {
        if let Some(game) = get_game() {
            let Game { handle, address } = game;
            let adm_tech_address = add_offsets(handle, &(address + 0x02420FC8), &tech_offsets.Admin);
            println!("address: {:X}", adm_tech_address);

            'read: loop {
                println!("Admin Tech is: {}", get_value(handle, &adm_tech_address));
                delay();
            }
        } else {
            println!("Couldn' get the game process! Retrying..");
        }

        delay();
    }
}
