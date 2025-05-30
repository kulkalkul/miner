use std::vec;

use macroquad::prelude::*;

#[derive(Default)]
pub struct ActionState {
    pub mine: Action<MineDownState>,
    pub lay_ladder: Action<LayLadderState>,
}

impl ActionState {
    pub fn reset(&mut self) {
        self.mine.reset();
        self.lay_ladder.reset();
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum MineDownState {
    #[default]
    Empty,
    Mine(IVec2, IVec2),
}

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum LayLadderState {
    #[default]
    Empty,
    LayLadder(IVec2, i32),
}

pub mod actions {
    use super::*;

    pub use MineDownState::Mine;
    pub use LayLadderState::LayLadder;    
}

use actions::*;

#[derive(Default)]
pub struct Actions<T> {
    actions: Vec<T>,
}

#[derive(Default)]
pub struct Action<T> {
    action: T,
}

impl<T> Actions<T> {
    pub fn act(&mut self, action: T) {
        self.actions.push(action);
    }
    pub fn reset(&mut self) {
        self.actions.clear();
    }
}

impl<T: Eq + Default + Copy> Action<T> {
    pub fn act(&mut self, action: T) {
        if self.action == Default::default() {
            self.action = action;
        }
    }
    pub fn act_panic(&mut self, action: T) {
        if self.action == Default::default() {
            panic!("This is singular action, can't add another one!");
        }
        self.action = action;
    }
    pub fn consume(&mut self) -> T {
        let action = self.action;
        self.action = Default::default();
        action
    }
    pub fn reset(&mut self) {
        self.action = Default::default();
    }
}

impl<'a, T> IntoIterator for &'a mut Actions<T> {
    type Item = T;
    type IntoIter = vec::Drain<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.drain(..)
    }
}
