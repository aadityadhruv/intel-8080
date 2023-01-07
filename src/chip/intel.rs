use std::fs::File;
use std::io::Read;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::WIDTH;
use crate::DISPLAY_LENGTH;
use crate::SCALE;

struct Flags {
    z : u8,
    s : u8,
    p : u8,
    cy : u8,
    ac : u8,

}
pub struct Intel8080 {
    //Registers
    a : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    h : u8,
    l : u8,
    pub instr : u8, //Current instr
    sp : u16, //stack pointer
    pc : u16, //program counter
    mem : [u8; 0x4000], //Memory buffer, which includes video buffer starting at 0x2400
    flags : Flags //Flags for math
}

impl Flags {
    //New Flags struct, used only once
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
    //New Intel 8080 Struct
    pub fn new() -> Self {
        Intel8080 {
            a : 0,
            b : 0, 
            c : 0,
            d : 0,
            e : 0,
            h : 0,
            l : 0,
            instr : 0,
            sp : 0,
            pc : 0,
            mem : [0; 0x4000],
            flags : Flags::new(),
        }
    }

    //Read rom into memory starting at address 0x0
    pub fn load_rom(&mut self, rom : &str) {
        let mut rom_buf = Vec::new();
        let mut rom = File::open(rom).unwrap_or_else(|_err| panic!("Valid ROM needed!"));
        rom.read_to_end(&mut rom_buf).unwrap_or_else(|_err| panic!("Error reading ROM"));

        for (i, val) in rom_buf.into_iter().enumerate() {
            self.mem[i] = val;
        }
    }
    //Clear input key
    pub fn clear_input(&mut self) {
        todo!();
    }
    //Read input key
    pub fn feed_input(&mut self, key : Keycode) {
        todo!();
    }
    //Fetch next instructions
    pub fn fetch(&mut self) {
        self.instr = self.mem[(self.pc as usize)];
        println!("Current instr is 0x{:02X}", self.instr);
        self.pc += 1;
    }
    pub fn execute(&mut self) {
        let mut next = 0;
        let arg1 = self.mem[self.pc as usize + 1];
        let arg2 = self.mem[self.pc as usize + 2];

        match self.instr {

            0x00 => { print!("NOP") }
            0x01 => { print!("LXI B D16"); next = 2;}
            0x02 => { print!("STAX B") }
            0x03 => { print!("INX B") }
            0x04 => { print!("INR B") }
            0x05 => { print!("DCR B") }
            0x06 => { print!("MVI B D8"); next = 1; }
            0x07 => { print!("RLC") }
            0x08 => { print!("-") }
            0x09 => { print!("DAD B") }
            0x0a => { print!("LDAX B") }
            0x0b => { print!("DCX B") }
            0x0c => { print!("INR C") }
            0x0d => { print!("DCR C") }
            0x0e => { print!("MVI C D8"); next = 1; }
            0x0f => { print!("RRC") }

            0x10 => { print!("-") }
            0x11 => { print!("LXI D D16"); next = 2; }
            0x12 => { print!("STAX D") }
            0x13 => { print!("INX D") }
            0x14 => { print!("INR D") }
            0x15 => { print!("DCR D") }
            0x16 => { print!("MVI D D8"); next = 1;}
            0x17 => { print!("RAL") }
            0x18 => { print!("-") }
            0x19 => { print!("DAD D") }
            0x1a => { print!("LDAX D") }
            0x1b => { print!("DCX D") }
            0x1c => { print!("INR E") }
            0x1d => { print!("DCR E") }
            0x1e => { print!("MVI E D8"); next = 1; }
            0x1f => { print!("RAR"); }

            0x20 => { print!("RIM") }
            0x21 => { print!("LXI H D16"); next = 2; }
            0x22 => { print!("SHLD adr"); next = 2; }
            0x23 => { print!("INX H") }
            0x24 => { print!("INR H") }
            0x25 => { print!("DCR H") }
            0x26 => { print!("MVI H D8"); next = 1; }
            0x27 => { print!("DAA") }
            0x28 => { print!("-") }
            0x29 => { print!("DAD H") }
            0x2a => { print!("LHLD adr"); next = 2; }
            0x2b => { print!("DCX H") }
            0x2c => { print!("INR L") }
            0x2d => { print!("DCR L") }
            0x2e => { print!("MVI L D8"); next = 1; }
            0x2f => { print!("CMA") }

            0x30 => { print!("SIM") }
            0x31 => { print!("LXI SP D16"); next = 2; }
            0x32 => { print!("STA adr"); next = 2; }
            0x33 => { print!("INX SP") }
            0x34 => { print!("INR M") }
            0x35 => { print!("DCR M") }
            0x36 => { print!("MVI M D8"); next = 1; }
            0x37 => { print!("STC") }
            0x38 => { print!("-") }
            0x39 => { print!("DAD SP") }
            0x3a => { print!("LDA adr"); next = 2; }
            0x3b => { print!("DCX SP") }
            0x3c => { print!("INR A") }
            0x3d => { print!("DCR A") }
            0x3e => { print!("MVI A, D8"); next = 1; }
            0x3f => { print!("CMC") }

            0x40 => { print!("MOV B B") }
            0x41 => { print!("MOV B C") }
            0x42 => { print!("MOV B D") }
            0x43 => { print!("MOV B E") }
            0x44 => { print!("MOV B H") }
            0x45 => { print!("MOV B L") }
            0x46 => { print!("MOV B M") }
            0x47 => { print!("MOV B A") }

            0x48 => { print!("MOV C B") }
            0x49 => { print!("MOV C C") }
            0x4a => { print!("MOV C D") }
            0x4b => { print!("MOV C E") }
            0x4c => { print!("MOV C H") }
            0x4d => { print!("MOV C L") }
            0x4e => { print!("MOV C M") }
            0x4f => { print!("MOV C A") }

            0x50 => { print!("MOV D B") }
            0x51 => { print!("MOV D C") }
            0x52 => { print!("MOV D D") }
            0x53 => { print!("MOV D E") }
            0x54 => { print!("MOV D H") }
            0x55 => { print!("MOV D L") }
            0x56 => { print!("MOV D M") }
            0x57 => { print!("MOV D A") }

            0x58 => { print!("MOV E B") }
            0x59 => { print!("MOV E C") }
            0x5a => { print!("MOV E D") }
            0x5b => { print!("MOV E E") }
            0x5c => { print!("MOV E H") }
            0x5d => { print!("MOV E L") }
            0x5e => { print!("MOV E M") }
            0x5f => { print!("MOV E A") }

            0x60 => { print!("MOV H B") }
            0x61 => { print!("MOV H C") }
            0x62 => { print!("MOV H D") }
            0x63 => { print!("MOV H E") }
            0x64 => { print!("MOV H H") }
            0x65 => { print!("MOV H L") }
            0x66 => { print!("MOV H M") }
            0x67 => { print!("MOV H A") }

            0x68 => { print!("MOV L B") }
            0x69 => { print!("MOV L C") }
            0x6a => { print!("MOV L D") }
            0x6b => { print!("MOV L E") }
            0x6c => { print!("MOV L H") }
            0x6d => { print!("MOV L L") }
            0x6e => { print!("MOV L M") }
            0x6f => { print!("MOV L A") }

            0x70 => { print!("MOV M B") }
            0x71 => { print!("MOV M C") }
            0x72 => { print!("MOV M D") }
            0x73 => { print!("MOV M E") }
            0x74 => { print!("MOV M H") }
            0x75 => { print!("MOV M L") }

            0x76 => { print!("HLT") }

            0x77 => { print!("MOV M A") }

            0x78 => { print!("MOV A B") }
            0x79 => { print!("MOV A C") }
            0x7a => { print!("MOV A D") }
            0x7b => { print!("MOV A E") }
            0x7c => { print!("MOV A H") }
            0x7d => { print!("MOV A L") }
            0x7e => { print!("MOV A M") }
            0x7f => { print!("MOV A A") }

            0x80 => { print!("ADD B") }
            0x81 => { print!("ADD C") }
            0x82 => { print!("ADD D") }
            0x83 => { print!("ADD E") }
            0x84 => { print!("ADD H") }
            0x85 => { print!("ADD L") }
            0x86 => { print!("ADD M") }
            0x87 => { print!("ADD A") }

            0x88 => { print!("ADC B") }
            0x89 => { print!("ADC C") }
            0x8a => { print!("ADC D") }
            0x8b => { print!("ADC E") }
            0x8c => { print!("ADC H") }
            0x8d => { print!("ADC L") }
            0x8e => { print!("ADC M") }
            0x8f => { print!("ADC A") }

            0x90 => { print!("SUB B") }
            0x91 => { print!("SUB C") }
            0x92 => { print!("SUB D") }
            0x93 => { print!("SUB E") }
            0x94 => { print!("SUB H") }
            0x95 => { print!("SUB L") }
            0x96 => { print!("SUB M") }
            0x97 => { print!("SUB A") }

            0x98 => { print!("SBB B") }
            0x99 => { print!("SBB C") }
            0x9a => { print!("SBB D") }
            0x9b => { print!("SBB E") }
            0x9c => { print!("SBB H") }
            0x9d => { print!("SBB L") }
            0x9e => { print!("SBB M") }
            0x9f => { print!("SBB A") }

            0xa0 => { print!("ANA B") }
            0xa1 => { print!("ANA C") }
            0xa2 => { print!("ANA D") }
            0xa3 => { print!("ANA E") }
            0xa4 => { print!("ANA H") }
            0xa5 => { print!("ANA L") }
            0xa6 => { print!("ANA M") }
            0xa7 => { print!("ANA A") }

            0xa8 => { print!("XRA B") }
            0xa9 => { print!("XRA C") }
            0xaa => { print!("XRA D") }
            0xab => { print!("XRA E") }
            0xac => { print!("XRA H") }
            0xad => { print!("XRA L") }
            0xae => { print!("XRA M") }
            0xaf => { print!("XRA A") }


            0xb0 => { print!("ORA B") }
            0xb1 => { print!("ORA C") }
            0xb2 => { print!("ORA D") }
            0xb3 => { print!("ORA E") }
            0xb4 => { print!("ORA H") }
            0xb5 => { print!("ORA L") }
            0xb6 => { print!("ORA M") }
            0xb7 => { print!("ORA A") }

            0xb8 => { print!("CMP B") }
            0xb9 => { print!("CMP C") }
            0xba => { print!("CMP D") }
            0xbb => { print!("CMP E") }
            0xbc => { print!("CMP H") }
            0xbd => { print!("CMP L") }
            0xbe => { print!("CMP M") }
            0xbf => { print!("CMP A") }

            0xc0 => { print!("RNZ") }
            0xc1 => { print!("POP B") }
            0xc2 => { print!("JNZ adr"); next = 2; }
            0xc3 => { print!("JMP adr"); next = 2; }
            0xc4 => { print!("CNZ adr"); next = 2; }
            0xc5 => { print!("PUSH B") }
            0xc6 => { print!("ADI D8"); next = 1; }
            0xc7 => { print!("RST 0") }

            0xc8 => { print!("RZ") }
            0xc9 => { print!("RET") }
            0xca => { print!("JZ adr") }
            0xcb => { print!("-") }
            0xcc => { print!("CZ adr"); next = 2; }
            0xcd => { print!("CALL adr"); next = 2; }
            0xce => { print!("ACI D8"); next = 1; }
            0xcf => { print!("RST 1") }

            0xd0 => { print!("RNC") }
            0xd1 => { print!("POP D") }
            0xd2 => { print!("JNC adr"); next = 2; }
            0xd3 => { print!("OUT D8"); next = 1; }
            0xd4 => { print!("CNC adr"); next = 2; }
            0xd5 => { print!("PUSH D") }
            0xd6 => { print!("SUI D8"); next = 1; }
            0xd7 => { print!("RST 2") }

            0xd8 => { print!("RC") }
            0xd9 => { print!("-") }
            0xda => { print!("JC adr"); next = 2; }
            0xdb => { print!("IN D8"); next = 1; }
            0xdc => { print!("CC adr"); next = 2; }
            0xdd => { print!("-");  }
            0xde => { print!("SBI D8"); next = 1; }
            0xdf => { print!("RST 3") }

            0xe0 => { print!("RPO") }
            0xe1 => { print!("POP H") }
            0xe2 => { print!("JPO adr"); next = 2; }
            0xe3 => { print!("XTHL") }
            0xe4 => { print!("CPO adr"); next = 2; }
            0xe5 => { print!("PUSH H") }
            0xe6 => { print!("ANI D8"); next = 1; }
            0xe7 => { print!("RST 4") }

            0xe8 => { print!("RPE") }
            0xe9 => { print!("PCHL") }
            0xea => { print!("JPE adr"); next = 2; }
            0xeb => { print!("XCHG") }
            0xec => { print!("CPE adr"); next = 2; }
            0xed => { print!("-") }
            0xee => { print!("XRI D8"); next = 1; }
            0xef => { print!("RST 5") }

            0xf0 => { print!("RP") }
            0xf1 => { print!("POP PSW") }
            0xf2 => { print!("JP adr"); next = 2; }
            0xf3 => { print!("DI") }
            0xf4 => { print!("CP adr"); next = 2; }
            0xf5 => { print!("PUSH PSW") }
            0xf6 => { print!("ORI D8"); next = 1; }
            0xf7 => { print!("RST 6") }

            0xf8 => { print!("RM") }
            0xf9 => { print!("SPHL") }
            0xfa => { print!("JM adr"); next = 2; }
            0xfb => { print!("EI") }
            0xfc => { print!("CM adr"); next = 2; }
            0xfd => { print!("-") }
            0xfe => { print!("CPI D8"); next = 1; }
            0xff => { print!("RST 7") }
        }

        match next {
            1 => { println!(" ${:02x}", arg1) }
            2 => { println!(" ${:02x}{:02x}", arg2, arg1) }
            _ => { println!("") }
        }
        self.pc += next;
    }
    pub fn render(&mut self, canvas : &mut Canvas<Window>) {
        for idx in 0..DISPLAY_LENGTH {
            let (mut x_coord, mut y_coord) : (i32, i32) =((idx as i32 % WIDTH as i32), (idx as i32 / WIDTH as i32)); //get x and y coord
            //Change scale to specified one
            x_coord *= SCALE as i32;
            y_coord *= SCALE as i32;
            //Draw rectangle as pixel, scale - 1 so border are seen
            let rect = Rect::new(x_coord, y_coord, SCALE, SCALE);
            //Choose color of bit
            let color = match self.mem[0x2400 + (idx / 8) as usize] {
                    0 => Color::RGB(0, 0, 0),
                    _ => Color::RGB(255, 255, 255),
            };
            //Draw into buffer
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();

        }
    }
}
