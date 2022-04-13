mod offsets;
mod process;
mod read;

use offsets::{CountryOffsets, TechOffsets};
use process::{add_offsets, get_game, get_value, Game};
use read::read_string;

use std::{thread::sleep, time::Duration};

fn delay() {
    sleep(Duration::from_millis(500));
}

fn main() {
    loop {
        if let Some(game) = get_game() {
            let Game { handle, address } = game;
            let tech_base = address + 0x02420FC8;
            let country_base = address + 0x0243D018;

            'handle: loop {
                // let admin_tech_address = add_offsets(&handle, &tech_base, &TechOffsets::ADMIN);

                // if get_value(&handle, &admin_tech_address) < 1 {
                //     println!("Probably game is not loaded yet. If it is try to open tech tab once.");
                //     delay();
                //     continue;
                // }

                let admin_tech_address = add_offsets(&handle, &tech_base, &TechOffsets::ADMIN);
                let diplo_tech_address = add_offsets(&handle, &tech_base, &TechOffsets::DIPLO);
                let military_tech_address =
                    add_offsets(&handle, &tech_base, &TechOffsets::MILITARY);

                let country_name_address =
                    add_offsets(&handle, &country_base, &CountryOffsets::NAME);

                loop {
                    // if get_value(&handle, &admin_tech_address) < 1 {
                    //     println!("Lost process!");
                    //     break 'handle;
                    // }

                    let name = read_string(&handle, country_name_address);
                    println!(
                        "Country: {name} --------- \n Admin: {} \n Diplo: {} \n Mil: {}",
                        get_value(&handle, &admin_tech_address),
                        get_value(&handle, &diplo_tech_address),
                        get_value(&handle, &military_tech_address)
                    );

                    delay();
                }
            }
        } else {
            println!("Can't find eu4.exe process! Retrying..");
        }

        delay();
    }
}
