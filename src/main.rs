use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use clap::Parser;


use intel_8080::debug::dassm;
use intel_8080::chip::intel;

#[derive(Parser, Debug)]
#[command(author="Aaditya Dhruv", version = "0.1.0", about="", long_about = None)]
struct Args {
    /// Set this flag to run the ROM under debug mode. 1 is to dump the rom data, 2 is to enter
    /// step mode.
    #[arg(short, long)]
    debug : Option<u8>,
    /// The path to the ROM, e.g. /path/to/rom
    rom : String,
}


fn main() {
    let args = Args::parse();
    let rom = args.rom;
    let debug = match args.debug { Some(v) => { v } None => {0} };
    println!("Debug mode is {}, loading ROM {}", if debug > 0 {"ON"} else {"OFF"}, rom);


    //SDL initalizationa and window creation
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_name = String::from("Intel-8080") + &rom;
    let window = video_subsystem.window(
        window_name.as_str(),
        intel_8080::WIDTH * intel_8080::SCALE,
        intel_8080::HEIGHT * intel_8080::SCALE
    )
        .position_centered()
        .build()
        .unwrap();

    //Canvas to interact with
    let mut canvas = window.into_canvas().build().unwrap();

    //Keyboard input handler
    let mut event_pump = sdl_context.event_pump().unwrap();



    if debug > 0 {
        let mut dassm = dassm::IntelDebug::new();
        dassm.load_rom(&rom);
        if debug == 1 {
            dassm.dump_rom();
        }
        else {
            'running: loop {
                //chip.clear_input();
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                break 'running
                            },
                            Event::KeyDown { keycode : Some(key), .. } => {
                                //chip.feed_input(key);
                            }
                        _ => {}
                    }
                }
                // The rest of the game loop goes here...
                //Draw black bg
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                //clear the screen with black bg
                canvas.clear();
                //choose white color 
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                //render all the rectangles as white pixels on the canvas
                //display canvas
                canvas.present();

                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

            }
        }
    }
    else {
        let mut chip = intel::Intel8080::new();
        chip.load_rom(&rom);

        'running: loop {
            //chip.clear_input();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        Event::KeyDown { keycode : Some(key), .. } => {
                            //chip.feed_input(key);
                        }
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            //Draw black bg
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            //clear the screen with black bg
            canvas.clear();
            //choose white color 
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            //render all the rectangles as white pixels on the canvas
            chip.fetch();
            chip.execute();
            chip.render(&mut canvas);
            //display canvas
            canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        }
    }
}
