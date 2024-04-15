mod interpreter;
mod view;

use interpreter::structs::{InterpreterError, Token};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::libc::RWF_NOWAIT;
use sdl2::rect::Rect;
use sdl2::{self, event::Event, pixels::Color, render::Canvas, video::Window, Sdl, VideoSubsystem};

use interpreter::Interpreter;

fn main() -> Result<(), String> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_system: VideoSubsystem = sdl_context.video()?;
    let window: Window = video_system.window("test", 800, 600)
        .build().unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    // let mut event_pump = sdl_context.event_pump().unwrap();
    // 'running: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} => {
    //                 break 'running;
    //             },
    //             _ => {}
    //         }
    //     }
    //     canvas.set_draw_color(Color::RGB(0, 0, 0));
    //     canvas.clear();
    //     canvas.filled_circle(50, 50, 10, Color::RGB(255, 0, 0)).unwrap();
    //     canvas.fill_rect(Rect::new(5, 1, 20, 10)).unwrap();
    //     canvas.present();
    // }

    let mut interpreter: Interpreter = Interpreter::new();
    let mut test: Vec<Token> = vec![];

    match interpreter.to_token_list("/home/tpl/projects/mwdf/test.dvi") {
        Ok(result) => {
            test = result;
        },
        Err(e) => {
            match e {
                InterpreterError::InvalidFile => print!("file issues"),
                InterpreterError::DecodingError => print!("idk"),
                InterpreterError::Syntax(row, col, msg) => print!("syntax error at row {}, col {}, message: {}", row, col, msg),
                _ => print!("i dont care"),
            }
        },
    };
    println!("");

    let mut test_ind = 0;

    match interpreter.parse_var(&mut test, &mut test_ind) {
        Ok(..) => {},
        Err(e) => {
            match e {
                InterpreterError::Syntax(row, col, msg) => {
                    println!("syntax error at {}, {}. message: {}", row, col, msg);
                }
               _ => println!("idk"), 
            }
        },
    };

    Ok(()) 
}