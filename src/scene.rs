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
    IntroScene01,
    MainScene01,
    OutroScene01,
    Scene02,
}

pub fn sequencer_thread(tx: Sender<Option<Sequence>>){
    fn change_sequence_delayed(to: Sequence, after: f32) -> Sequence{
        println!("Sleeping before change to {:?}", to);
        thread::sleep(time::Duration::from_secs_f32(after));
        to
    }

    let mut current_sequence = Sequence::IntroScene01;
    loop {
        let sequence_totx = current_sequence;
        tx.send(Some(sequence_totx)).unwrap();
        match current_sequence {
            Sequence::IntroScene01 => current_sequence = change_sequence_delayed(Sequence::MainScene01, 2_f32),
            Sequence::MainScene01 => current_sequence = change_sequence_delayed(Sequence::OutroScene01, 2_f32),
            Sequence::OutroScene01 => current_sequence = change_sequence_delayed(Sequence::Scene02, 2_f32),
            Sequence::Quit => break,
            _ => current_sequence = change_sequence_delayed(Sequence::IntroScene01, 2_f32),
        }
    }
}