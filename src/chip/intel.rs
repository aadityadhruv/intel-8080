use std::fs::File;
use std::io::Read;

struct Flags {
    z : u8,
    s : u8,
    p : u8,
    cy : u8,
    ac : u8,

}
pub struct Intel8080 {
    a : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    h : u8,
    l : u8,
    sp : u16,
    pc : u16,
    mem : [u8; 0x4000],
    flags : Flags
}

impl Flags {
    fn new() -> Self {
        Flags {
            z : 0,
            s : 0, 
            p : 0,
            cy : 0,
            ac : 0,
        }
    }
}

impl Intel8080 {
    pub fn new() -> Self {
        Intel8080 {
            a : 0,
            b : 0, 
            c : 0,
            d : 0,
            e : 0,
            h : 0,
            l : 0,
            sp : 0,
            pc : 0,
            mem : [0; 0x4000],
            flags : Flags::new(),
        }
    }

    pub fn load_rom(&mut self, rom : &str) {
        let mut rom_buf = Vec::new();
        let mut rom = File::open(rom).unwrap_or_else(|_err| panic!("Valid ROM needed!"));
        rom.read_to_end(&mut rom_buf).unwrap_or_else(|_err| panic!("Error reading ROM"));

        for (i, val) in rom_buf.into_iter().enumerate() {
            self.mem[i] = val;
        }
    }
}
