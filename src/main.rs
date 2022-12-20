#![windows_subsystem = "windows"]
extern crate sdl2;

use std::sync::mpsc;
use std::thread;

use akinoxmas2022::display::Display;
use akinoxmas2022::logo::Logo;
use akinoxmas2022::platonics::Platonics;
use akinoxmas2022::scene::{music_thread, sequencer_thread, Scene, Sequence};
use akinoxmas2022::scroller::Scroller;
use akinoxmas2022::starfield::Starfield;
use akinoxmas2022::torus::Torus;
//use akinoxmas2022::vector::Vec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String> {
    let mut display = Display::new();
    display.cls();
    let mut event_pump = display.event_pump();

    let starfield = Starfield::new(&mut display);

    let scroller = Scroller::new(&mut display);

    let platonics = Platonics::new(&mut display);

    //display.add_sprite("akinosoft", "./assets/akinosoft.png");
    let logo = Logo::new(&mut display);

    let torus = Torus::new(&mut display);

    let mut scenes: Vec<Box<dyn Scene>> = vec![
        Box::new(starfield),
        Box::new(logo),
        Box::new(scroller),
        Box::new(platonics),
        Box::new(torus),
    ];

    let mut frame_time: u32;
    let mut last_frame_delta: u32 = 0;

    let mut frames: usize = 0;

    //display.clear_color_buffer(0, 0, 0);

    let _scene_music_thread_handle = thread::spawn(music_thread);

    let mut change_current_scene: Option<Sequence>;
    let (tx, rx) = mpsc::channel::<Option<Sequence>>();
    let _scene_changer_thread_handle = thread::spawn(move || sequencer_thread(tx));

    let start = display.ticks();
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

        //check current scene
        change_current_scene = rx.try_recv().unwrap_or(None);

        display.cls();

        // update all
        scenes
            .iter_mut()
            .for_each(|scene| scene.update(last_frame_delta, &display, &change_current_scene));

        // render all
        scenes.iter().for_each(|scene| scene.render(&mut display));

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
    println!("FPS: {}", frames as f32 / (display.ticks() as f32 / 1000.0));
    Ok(())
}
