use sdl2::{self, video::Window, Sdl, VideoSubsystem, event::Event, pixels::Color};

fn main() -> Result<(), String> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_system: VideoSubsystem = sdl_context.video()?;
    let window: Window = video_system.window("test", 800, 600)
        .build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }

    Ok(()) 
}