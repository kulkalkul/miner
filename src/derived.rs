use macroquad::prelude::*;

#[derive(Default)]
pub struct DerivedState {
    pub time_sine_1: [f32; 16],
    pub time_sine_2: [f32; 16],
    pub time_sine_3: [f32; 16],
    pub time_sine_4: [f32; 16],
    pub player_anim_finished: bool,
    pub player_moving: bool,
    pub player_touching_right: bool,
    pub player_touching_left: bool,
    pub player_touching_top: bool,
    pub player_touching_bottom: bool,

    pub player_mining_speed: f32,
    pub player_ladder_speed: f32,
    pub player_bag_carry_capacity: usize,
    pub player_hit_str: f32,

    pub ui_show_statue_key: bool,
}

#[derive(Default)]
pub struct LateDerivedState {
    pub ui_is_active: bool,
}
