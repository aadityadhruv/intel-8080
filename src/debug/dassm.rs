use std::fs::File;
use std::io::Read;

//A rudimentary struct for debugging 
pub struct IntelDebug {
    rom_buf : Vec::<u8>, 
    idx : usize,
}

impl IntelDebug {
    //Create a new struct for debugging
    pub fn new() -> Self {
        IntelDebug {
            //The entire rom as a buffer
            rom_buf : Vec::<u8>::new(),
            //idx to index the memory
            idx : 0,
        }
    }
    //Read the provided ROM into memory
    pub fn read_rom(&mut self, rom : &str) {
        let mut rom = File::open(rom).unwrap_or_else(|_err| panic!("Valid ROM needed!"));
        rom.read_to_end(&mut self.rom_buf).unwrap_or_else(|_err| panic!("Error reading ROM"));
    }

    //DEBUG: Translates the instruction at self.idx into human-readable assembly
    fn translate(&mut self) {
        //The instruction itself
        let instr = self.rom_buf[self.idx];
        //Possible arguments for the current instruction
        let arg1 = self.rom_buf.get(self.idx + 1);
        let arg2 = self.rom_buf.get(self.idx + 2);
        let mut next = 1;

        //Print the current instruction number and the instruction
        print!("|{:04x}  |  {:04x}  |  ", self.idx, instr);
        //Match the Instruction to all possible opcodes and print the corresponding instruction
        match instr {

            0x00 => { println!("NOP") }
            0x01 => { println!("LXI B D16 | {:?}, {:?}", arg2, arg1); next = 3;}
            0x02 => { println!("STAX B") }
            0x03 => { println!("INX B") }
            0x04 => { println!("INR B") }
            0x05 => { println!("DCR B") }
            0x06 => { println!("MVI B D8 | {:?}", arg2); next = 2; }
            0x07 => { println!("RLC") }
            0x08 => { println!("-") }
            0x09 => { println!("DAD B") }
            0x0a => { println!("LDAX B") }
            0x0b => { println!("DCX B") }
            0x0c => { println!("INR C") }
            0x0d => { println!("DCR C") }
            0x0e => { println!("MVI C D8 | {:?}", arg1); next = 2; }
            0x0f => { println!("RRC") }

            0x10 => { println!("-") }
            0x11 => { println!("LXI D D16 | {:?}, {:?}", arg2, arg1); next = 3; }
            0x12 => { println!("STAX D") }
            0x13 => { println!("INX D") }
            0x14 => { println!("INR D") }
            0x15 => { println!("DCR D") }
            0x16 => { println!("MVI D D8 | {:?}", arg1); next = 2;}
            0x17 => { println!("RAL") }
            0x18 => { println!("-") }
            0x19 => { println!("DAD D") }
            0x1a => { println!("LDAX D") }
            0x1b => { println!("DCX D") }
            0x1c => { println!("INR E") }
            0x1d => { println!("DCR E") }
            0x1e => { println!("MVI E D8 | {:?}", arg1); next = 2; }
            0x1f => { println!("RAR"); }

            0x20 => { println!(" ") }
            0x21 => { println!(" ") }
            0x22 => { println!(" ") }
            0x23 => { println!(" ") }
            0x24 => { println!(" ") }
            0x25 => { println!(" ") }
            0x26 => { println!(" ") }
            0x27 => { println!(" ") }
            0x28 => { println!(" ") }
            0x29 => { println!(" ") }
            0x2a => { println!(" ") }
            0x2b => { println!(" ") }
            0x2c => { println!(" ") }
            0x2d => { println!(" ") }
            0x2e => { println!(" ") }
            0x2f => { println!(" ") }
            0x30 => { println!(" ") }
            0x31 => { println!(" ") }
            0x32 => { println!(" ") }
            0x33 => { println!(" ") }
            0x34 => { println!(" ") }
            0x35 => { println!(" ") }
            0x36 => { println!(" ") }
            0x37 => { println!(" ") }
            0x38 => { println!(" ") }
            0x39 => { println!(" ") }
            0x3a => { println!(" ") }
            0x3b => { println!(" ") }
            0x3c => { println!(" ") }
            0x3d => { println!(" ") }
            0x3e => { println!(" ") }
            0x3f => { println!(" ") }
            0x40 => { println!(" ") }
            0x41 => { println!(" ") }
            0x42 => { println!(" ") }
            0x43 => { println!(" ") }
            0x44 => { println!(" ") }
            0x45 => { println!(" ") }
            0x46 => { println!(" ") }
            0x47 => { println!(" ") }
            0x48 => { println!(" ") }
            0x49 => { println!(" ") }
            0x4a => { println!(" ") }
            0x4b => { println!(" ") }
            0x4c => { println!(" ") }
            0x4d => { println!(" ") }
            0x4e => { println!(" ") }
            0x4f => { println!(" ") }
            0x50 => { println!(" ") }
            0x51 => { println!(" ") }
            0x52 => { println!(" ") }
            0x53 => { println!(" ") }
            0x54 => { println!(" ") }
            0x55 => { println!(" ") }
            0x56 => { println!(" ") }
            0x57 => { println!(" ") }
            0x58 => { println!(" ") }
            0x59 => { println!(" ") }
            0x5a => { println!(" ") }
            0x5b => { println!(" ") }
            0x5c => { println!(" ") }
            0x5d => { println!(" ") }
            0x5e => { println!(" ") }
            0x5f => { println!(" ") }
            0x60 => { println!(" ") }
            0x61 => { println!(" ") }
            0x62 => { println!(" ") }
            0x63 => { println!(" ") }
            0x64 => { println!(" ") }
            0x65 => { println!(" ") }
            0x66 => { println!(" ") }
            0x67 => { println!(" ") }
            0x68 => { println!(" ") }
            0x69 => { println!(" ") }
            0x6a => { println!(" ") }
            0x6b => { println!(" ") }
            0x6c => { println!(" ") }
            0x6d => { println!(" ") }
            0x6e => { println!(" ") }
            0x6f => { println!(" ") }
            0x70 => { println!(" ") }
            0x71 => { println!(" ") }
            0x72 => { println!(" ") }
            0x73 => { println!(" ") }
            0x74 => { println!(" ") }
            0x75 => { println!(" ") }
            0x76 => { println!(" ") }
            0x77 => { println!(" ") }
            0x78 => { println!(" ") }
            0x79 => { println!(" ") }
            0x7a => { println!(" ") }
            0x7b => { println!(" ") }
            0x7c => { println!(" ") }
            0x7d => { println!(" ") }
            0x7e => { println!(" ") }
            0x7f => { println!(" ") }
            0x80 => { println!(" ") }
            0x81 => { println!(" ") }
            0x82 => { println!(" ") }
            0x83 => { println!(" ") }
            0x84 => { println!(" ") }
            0x85 => { println!(" ") }
            0x86 => { println!(" ") }
            0x87 => { println!(" ") }
            0x88 => { println!(" ") }
            0x89 => { println!(" ") }
            0x8a => { println!(" ") }
            0x8b => { println!(" ") }
            0x8c => { println!(" ") }
            0x8d => { println!(" ") }
            0x8e => { println!(" ") }
            0x8f => { println!(" ") }
            0x90 => { println!(" ") }
            0x91 => { println!(" ") }
            0x92 => { println!(" ") }
            0x93 => { println!(" ") }
            0x94 => { println!(" ") }
            0x95 => { println!(" ") }
            0x96 => { println!(" ") }
            0x97 => { println!(" ") }
            0x98 => { println!(" ") }
            0x99 => { println!(" ") }
            0x9a => { println!(" ") }
            0x9b => { println!(" ") }
            0x9c => { println!(" ") }
            0x9d => { println!(" ") }
            0x9e => { println!(" ") }
            0x9f => { println!(" ") }
            0xa0 => { println!(" ") }
            0xa1 => { println!(" ") }
            0xa2 => { println!(" ") }
            0xa3 => { println!(" ") }
            0xa4 => { println!(" ") }
            0xa5 => { println!(" ") }
            0xa6 => { println!(" ") }
            0xa7 => { println!(" ") }
            0xa8 => { println!(" ") }
            0xa9 => { println!(" ") }
            0xaa => { println!(" ") }
            0xab => { println!(" ") }
            0xac => { println!(" ") }
            0xad => { println!(" ") }
            0xae => { println!(" ") }
            0xaf => { println!(" ") }
            0xb0 => { println!(" ") }
            0xb1 => { println!(" ") }
            0xb2 => { println!(" ") }
            0xb3 => { println!(" ") }
            0xb4 => { println!(" ") }
            0xb5 => { println!(" ") }
            0xb6 => { println!(" ") }
            0xb7 => { println!(" ") }
            0xb8 => { println!(" ") }
            0xb9 => { println!(" ") }
            0xba => { println!(" ") }
            0xbb => { println!(" ") }
            0xbc => { println!(" ") }
            0xbd => { println!(" ") }
            0xbe => { println!(" ") }
            0xbf => { println!(" ") }
            0xc0 => { println!(" ") }
            0xc1 => { println!(" ") }
            0xc2 => { println!(" ") }
            0xc3 => { println!(" ") }
            0xc4 => { println!(" ") }
            0xc5 => { println!(" ") }
            0xc6 => { println!(" ") }
            0xc7 => { println!(" ") }
            0xc8 => { println!(" ") }
            0xc9 => { println!(" ") }
            0xca => { println!(" ") }
            0xcb => { println!(" ") }
            0xcc => { println!(" ") }
            0xcd => { println!(" ") }
            0xce => { println!(" ") }
            0xcf => { println!(" ") }
            0xd0 => { println!(" ") }
            0xd1 => { println!(" ") }
            0xd2 => { println!(" ") }
            0xd3 => { println!(" ") }
            0xd4 => { println!(" ") }
            0xd5 => { println!(" ") }
            0xd6 => { println!(" ") }
            0xd7 => { println!(" ") }
            0xd8 => { println!(" ") }
            0xd9 => { println!(" ") }
            0xda => { println!(" ") }
            0xdb => { println!(" ") }
            0xdc => { println!(" ") }
            0xdd => { println!(" ") }
            0xde => { println!(" ") }
            0xdf => { println!(" ") }
            0xe0 => { println!(" ") }
            0xe1 => { println!(" ") }
            0xe2 => { println!(" ") }
            0xe3 => { println!(" ") }
            0xe4 => { println!(" ") }
            0xe5 => { println!(" ") }
            0xe6 => { println!(" ") }
            0xe7 => { println!(" ") }
            0xe8 => { println!(" ") }
            0xe9 => { println!(" ") }
            0xea => { println!(" ") }
            0xeb => { println!(" ") }
            0xec => { println!(" ") }
            0xed => { println!(" ") }
            0xee => { println!(" ") }
            0xef => { println!(" ") }
            0xf0 => { println!(" ") }
            0xf1 => { println!(" ") }
            0xf2 => { println!(" ") }
            0xf3 => { println!(" ") }
            0xf4 => { println!(" ") }
            0xf5 => { println!(" ") }
            0xf6 => { println!(" ") }
            0xf7 => { println!(" ") }
            0xf8 => { println!(" ") }
            0xf9 => { println!(" ") }
            0xfa => { println!(" ") }
            0xfb => { println!(" ") }
            0xfc => { println!(" ") }
            0xfd => { println!(" ") }
            0xfe => { println!(" ") }
            0xff => { println!(" ") }
        }

        self.idx += next;
    }
    //A very basic debugging option to dump the whole rom and translate it
    pub fn dump_rom(&mut self) {
        //temporarily store the value of idx for later restoration
        let temp = self.idx;
        //Loop until we reach the end of the buffer
        println!("Index  | Opcode |  Instruction");
        println!("--------------------------------");
        while self.idx < self.rom_buf.len() {
            self.translate();
        }
        //reset the idx back to 0 incase future operations are needed
        self.idx = temp;
    }
}


