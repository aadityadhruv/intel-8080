use std::fs::File;
use std::io::Read;

pub struct IntelDebug {
    rom_buf : Vec::<u8>, 
    idx : usize,
}

impl IntelDebug {

    pub fn new() -> Self {
        IntelDebug {
            rom_buf : Vec::<u8>::new(),
            idx : 0,
        }
    }
    pub fn read_rom(&mut self, rom : &str) {
        let mut rom = File::open(rom).unwrap_or_else(|_err| panic!("Valid ROM needed!"));
        rom.read_to_end(&mut self.rom_buf).unwrap_or_else(|_err| panic!("Error reading ROM"));
    }

    fn translate(&mut self) {
        let instr = self.rom_buf[self.idx];
        println!("{:04x}    {:04x}", self.idx, instr);
        match instr {
            _ => {}
        }
    }
    pub fn dump_rom(&mut self) {
        for idx in 0..self.rom_buf.len() {
            self.idx = idx;
            self.translate();
        }
        self.idx = 0;
    }
}


