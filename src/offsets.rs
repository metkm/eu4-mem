
pub struct Tech {
    pub admin: [usize; 9]
}

impl Tech {
    pub fn new() -> Self {
        Tech {
            admin: [0xD0, 0x220, 0x40, 0x290, 0xF0, 0x118, 0x100, 0x0, 0x148]
        }
    }
}
