extern crate sdl2;

use akinoxmas2022::platonics::Platonics;
use akinoxmas2022::scroller::Scroller;
use akinoxmas2022::display::Display;
use akinoxmas2022::starfield::Starfield;
use akinoxmas2022::logo::Logo;
use akinoxmas2022::vector::Vec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String> {
    let mut display = Display::new();
    display.cls();
    let mut event_pump = display.event_pump();

    let mut starfield = Starfield::new(&mut display);

    let mut scroller = Scroller::new(&mut display);

    let mut platonics = Platonics::new(&mut display);

    //display.add_sprite("akinosoft", "./assets/akinosoft.png");
    let mut logo = Logo::new(&mut display);

    let start = display.ticks();
    //let target_ticks_frame: u32 = 1000/60;
    let mut frame_time: u32;
    let mut last_frame_delta: u32 = 0;

    let mut frames: usize = 0;

    //display.clear_color_buffer(0, 0, 0);

    display.add_streaming_buffer("test", 320, 180);

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

        starfield.update(last_frame_delta, &display);

        scroller.update(last_frame_delta, &display);

        platonics.update(last_frame_delta, &display);

        logo.update(last_frame_delta, &display);

        //display.clear_color_buffer(0, 0, 0);
        //display.clear_streaming_buffer_named("starfield", 0, 0, 0);

        starfield.render(&mut display);

        //display.color_buffer_to_canvas();
        //display.streaming_buffer_to_canvas_named("starfield");

        logo.render(&mut display);
        //display.put_sprite_centered("akinosoft", display.w_width() as i32 / 2_i32, 150, 0.33);

        scroller.render(&mut display);

        platonics.render(&mut display);

        display.clear_streaming_buffer("test", 0, 0, 0);

        display.line("test", 320, 200, 0, 0, 255, 255, 255);
        display.filled_triangle(
            "test", 
            &Vec2{x: 30.0, y: -10.0}, 
            &Vec2{x: 50.0, y: 100.0}, 
            &Vec2{x: -20.0, y: 50.0}, 
            255, 0, 0);

        display.streaming_buffer_to_canvas("test");

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
