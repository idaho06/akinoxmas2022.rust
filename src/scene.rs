use crate::display::Display;
use core::time;
use std::{sync::mpsc::Sender, thread};

pub trait Scene {
    fn update(&mut self, t: u32, display: &Display, scene: &Option<Sequence>);
    fn render(&self, display: &mut Display);
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    PlatonicsIcoIn,
    PlatonicsIcoOut,
    PlatonicsDodecIn,
    PlatonicsDodecOut,
}

pub fn sequencer_thread(tx: Sender<Option<Sequence>>) {
    fn change_sequence_delayed(to: Sequence, after: f32) -> Sequence {
        println!("Sleeping before change to {:?}", to);
        thread::sleep(time::Duration::from_secs_f32(after));
        to
    }

    //let mut current_sequence = Sequence::LogoFallingIn;
    let mut current_sequence = Sequence::PlatonicsTetraIn;
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
                current_sequence = change_sequence_delayed(Sequence::PlatonicsOctaIn, 4_f32)
            }

            Sequence::Quit => break,
            _ => current_sequence = change_sequence_delayed(Sequence::LogoFallingIn, 0_f32),
        }
    }
}
