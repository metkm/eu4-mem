
pub struct Tech {
    pub Admin: Vec<usize>
}

impl Tech {
    pub fn new() -> Self {
        Tech {
            Admin: vec![0xD0, 0x220, 0x40, 0x290, 0xF0, 0x118, 0x100, 0x0, 0x148]
        }
    }
}
