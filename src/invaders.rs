use std::time::Duration;

use rusty_time::Timer;

use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        Self {
            army: (2..NUM_COLS - 2)
                .filter(|&x| x % 2 == 0)
                .flat_map(|x| {
                    (1..9)
                        .filter(|&y| (y % 2 == 0))
                        .map(|y| Invader { x, y })
                        .collect::<Vec<_>>()
                })
                .collect(),
            move_timer: Timer::new(Duration::from_millis(200)),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.tick(delta);
        if self.move_timer.finished() {
            self.move_timer.reset();
            let downwards = if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    true
                } else {
                    false
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    true
                } else {
                    false
                }
            };
            if downwards {
                let new_duration = self
                    .move_timer
                    .duration()
                    .as_millis()
                    .saturating_sub(250)
                    .max(250);
                self.move_timer = Timer::new(Duration::from_millis(new_duration as u64));
                self.army.iter_mut().for_each(|invader| invader.y += 1);
            } else {
                self.army.iter_mut().for_each(|invader| {
                    invader.x = invader.x.saturating_add_signed(self.direction as isize);
                });
            }
            true
        } else {
            false
        }
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        self.army.iter().for_each(|invader| {
            frame[invader.x][invader.y] = if self.move_timer.percent_left() > 0.5 {
                "x"
            } else {
                "+"
            };
        });
    }
}
