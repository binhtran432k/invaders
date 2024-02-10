use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    curr_frame.iter().enumerate().for_each(|(x, col)| {
        col.iter()
            .enumerate()
            .filter(|&(y, &s)| force || s != last_frame[x][y])
            .for_each(|(y, &s)| {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", s);
            })
    });

    stdout.flush().unwrap();
}
