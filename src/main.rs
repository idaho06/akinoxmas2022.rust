extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use akinoxmas2022::display::Display;
use akinoxmas2022::point::Point;
use akinoxmas2022::starfield::Starfield;
use akinoxmas2022::vector::Vec3;
use akinoxmas2022::{render_starfield, update_starfield};

pub fn main() -> Result<(), String> {
    let mut display = Display::new();
    display.cls();
    let mut event_pump = display.event_pump();

    let mut starfield_3d = Starfield::new();
    let mut starfield_2d: Vec<Point> = vec![]; // TODO: move this to starfield.rs
    let mut direction: Vec3 = Vec3 { // TODO: move this to starfield
        x: 0.0,
        y: 0.0,
        z: -0.5,
    };
    //let v = Vec3{x:5.0, y:5.0, z:5.0};
    //starfield_3d.displace(&v);
    //let mut rng = rand::thread_rng();

    display.add_sprite("akinosoft", "./assets/akinosoft.png");

    let start = display.ticks();
    //let target_ticks_frame: u32 = 1000/60;
    let mut frame_time = start;
    let mut last_frame_delta: u32 = 0;

    let mut frames: usize = 0;

    'running: loop {
        frame_time = display.ticks();
        // check input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        display.cls();

        display.clear_color_buffer(0, 0, 0);

        update_starfield(
            last_frame_delta,
            &direction,
            &mut starfield_3d,
            &mut starfield_2d,
        ); //applies 3d animation and transform 3d points to screen 2d

        render_starfield(&mut display, &starfield_2d); // draws points to color buffer

        display.color_buffer_to_canvas();

        display.put_sprite("akinosoft", 0, 0, 0.5);

        display.present_canvas();

        // The rest of the game loop goes here...
        frames += 1;
        //if display.ticks() - start >= 1000 {
        //    break;
        //}
        last_frame_delta = display.ticks() - frame_time;
    }

    println!("Time: {}", display.ticks() - start);
    println!("frames: {}", frames);
    println!("FPS: {}", frames as f32 / (display.ticks() as f32/1000.0));
    Ok(())
}
