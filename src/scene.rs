use rusty_audio::Audio;

use crate::display::Display;
use core::time;
use std::{sync::mpsc::Sender, thread};

pub trait Scene {
    fn update(&mut self, t: u32, display: &Display, scene: &Option<Sequence>);
    fn render(&self, display: &mut Display);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sequence {
    Quit,
    LogoFallingIn,
    LogoFallingOut,
    PlatonicsTetraIn,
    PlatonicsTetraOut,
    PlatonicsOctaIn,
    PlatonicsOctaOut,
    PlatonicsCubeIn,
    PlatonicsCubeOut,
    PlatonicsIcosIn,
    PlatonicsIcosOut,
    PlatonicsDodecIn,
    PlatonicsDodecOut,
    Torus01In,
    Torus01ColorRotate,
    Torus01RotateY,
    Torus01Out,
    Torus02In,
    Torus02RotateZ,
    Torus02Out,
    Torus03In,
    Torus03CrazyRotate,
    Torus03Out,
    Torus04In,
    Torus04Rotate,
    Torus04Out,
}

pub fn music_thread() {
    let mut audio = Audio::new();
    audio.add("music", "./assets/placeholder01.ogg");
    loop {
        audio.play("music");
        audio.wait();
    }
}

pub fn sequencer_thread(tx: Sender<Option<Sequence>>) {
    fn change_sequence_delayed(to: Sequence, after: f32) -> Sequence {
        //println!("Sleeping before change to {:?}", to);
        thread::sleep(time::Duration::from_secs_f32(after));
        to
    }

    //let mut current_sequence = Sequence::LogoFallingIn;
    //let mut current_sequence = Sequence::LogoFallingIn; // first scene
    let mut current_sequence = Sequence::LogoFallingIn; // first scene
    loop {
        let sequence_totx = current_sequence;
        tx.send(Some(sequence_totx)).unwrap();
        match current_sequence {
            Sequence::LogoFallingIn => {
                current_sequence = change_sequence_delayed(Sequence::LogoFallingOut, 10_f32)
            }
            Sequence::LogoFallingOut => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsTetraIn, 3_f32)
            }
            Sequence::PlatonicsTetraIn => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsTetraOut, 8_f32)
            }
            Sequence::PlatonicsTetraOut => {
                current_sequence = change_sequence_delayed(Sequence::Torus01In, 4_f32)
            }
            Sequence::PlatonicsOctaIn => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsOctaOut, 8_f32)
            }
            Sequence::PlatonicsOctaOut => {
                current_sequence = change_sequence_delayed(Sequence::Torus02In, 4_f32)
            }
            Sequence::PlatonicsCubeIn => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsCubeOut, 8_f32)
            }
            Sequence::PlatonicsCubeOut => {
                current_sequence = change_sequence_delayed(Sequence::Torus03In, 4_f32)
            }
            Sequence::PlatonicsIcosIn => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsIcosOut, 8_f32)
            }
            Sequence::PlatonicsIcosOut => {
                current_sequence = change_sequence_delayed(Sequence::Torus04In, 4_f32)
            }
            Sequence::PlatonicsDodecIn => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsDodecOut, 8_f32)
            }
            Sequence::PlatonicsDodecOut => {
                current_sequence = change_sequence_delayed(Sequence::LogoFallingIn, 4_f32)
            }
            Sequence::Torus01In => {
                current_sequence = change_sequence_delayed(Sequence::Torus01ColorRotate, 3.1_f32)
            }
            Sequence::Torus01ColorRotate => {
                current_sequence = change_sequence_delayed(Sequence::Torus01RotateY, 5_f32)
            }
            Sequence::Torus01RotateY => {
                current_sequence = change_sequence_delayed(Sequence::Torus01Out, 15_f32)
            }
            Sequence::Torus01Out => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsOctaIn, 3_f32)
            }
            Sequence::Torus02In => {
                current_sequence = change_sequence_delayed(Sequence::Torus02RotateZ, 3_f32)
            }
            Sequence::Torus02RotateZ => {
                current_sequence = change_sequence_delayed(Sequence::Torus02Out, 15_f32)
            }
            Sequence::Torus02Out => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsCubeIn, 2_f32)
            }
            Sequence::Torus03In => {
                current_sequence = change_sequence_delayed(Sequence::Torus03CrazyRotate, 2_f32)
            }
            Sequence::Torus03CrazyRotate => {
                current_sequence = change_sequence_delayed(Sequence::Torus03Out, 20_f32)
            }
            Sequence::Torus03Out => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsIcosIn, 2_f32)
            }
            Sequence::Torus04In => {
                current_sequence = change_sequence_delayed(Sequence::Torus04Rotate, 4_f32)
            }
            Sequence::Torus04Rotate => {
                current_sequence = change_sequence_delayed(Sequence::Torus04Out, 20_f32)
            }
            Sequence::Torus04Out => {
                current_sequence = change_sequence_delayed(Sequence::PlatonicsDodecIn, 4_f32)
            }

            Sequence::Quit => break,
            _ => current_sequence = change_sequence_delayed(Sequence::LogoFallingIn, 0_f32),
        }
    }
}
