use macroquad::prelude::*;

#[derive(Default)]
pub struct DerivedState {
    pub player_anim_finished: bool,
    pub player_moving: bool,
    pub player_touching_right: bool,
    pub player_touching_left: bool,
    pub player_touching_top: bool,
    pub player_touching_bottom: bool,
}
