mod process;
mod offsets;

use std::{
    time::{Duration, Instant},
    thread::sleep
};
use process::{get_game, add_offsets, Game, get_value};
use offsets::{Tech};

fn delay() {
    sleep(Duration::from_millis(500));
}

fn main() {
    let tech_offsets = Tech::new();

    loop {
        if let Some(game) = get_game() {
            let Game { handle, address } = game;
            let adm_tech_address = add_offsets(handle, &(address + 0x02420FC8), &tech_offsets.admin);
            let dip_tech_address = add_offsets(handle, &(address + 0x02420FC8), &tech_offsets.diplo);

            'read: loop {
                let now = Instant::now();
                println!("Admin Tech is: {}, Diplo Tech is: {}", get_value(handle, &adm_tech_address), get_value(handle, &dip_tech_address));
                println!("Delay: {:.2?}", now.elapsed());
                delay();
            }
        } else {
            println!("Couldn' get the game process! Retrying..");
        }

        delay();
    }
}
