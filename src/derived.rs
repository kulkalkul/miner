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
    pub player_mining: bool,

    pub player_at_overworld: bool,
    pub player_mining_speed: f32,
    pub player_ladder_speed: f32,
    pub player_bag_carry_capacity: usize,
    pub player_climb_momentum_max: f32,
    pub player_hit_str: f32,
    pub player_has_jetpack: bool,
    pub player_can_place_ladder: bool,
    pub player_can_use_jetpack: bool,
    pub player_jetpack_fuel_capacity: f32,
    pub player_jetpack_speed: f32,
    
    pub bought_demolisher: bool,

    pub ui_main_menu: bool,
    pub ui_show_statue_key: bool,
    pub ui_show_demolisher_key: bool,
    pub ui_show_minecart_key: bool,
}

#[derive(Default)]
pub struct LateDerivedState {
    pub ui_is_active: bool,
    pub travelling_in_elevator: bool,
}
