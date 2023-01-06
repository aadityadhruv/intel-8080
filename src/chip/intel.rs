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
use rand::Rng;

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
    instr : u16, //Current instr
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
        self.instr = (self.mem[(self.pc as usize)] as u16) << 8  | self.mem[self.pc as usize + 1] as u16;
        self.pc += 2;
    }
    pub fn execute(&mut self) {
        match self.instr {
            _ => {}
        }
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
