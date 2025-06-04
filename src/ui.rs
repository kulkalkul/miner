use std::hash::{DefaultHasher, Hash, Hasher};

use macroquad::prelude::*;

use crate::sprite::{draw_ui_three_patch, SpriteAsset};

pub struct UIState {
    pub mouse_div: Vec2,
    pub last_clicked_button_hash: Option<u64>,
}

pub fn ui_button(
    state: &mut UIState,
    name: &str,
    position: Vec2,
    width: f32,
    disabled: bool,
    out_pressing: Option<&mut bool>,
    sprites: &[[SpriteAsset; 3]; 3],
) -> bool {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let hash = hasher.finish();

    // HACK: again...
    let height = sprites[0][0].frames[0].h;
    let rect = Rect { x: position.x, y: position.y, w: width, h: height };
    
    #[allow(unused_assignments)]
    let mut pressed = false;
    let mut pressing = false;
    
    if !disabled && rect.contains(Vec2::from(mouse_position()) / state.mouse_div) {
        if is_mouse_button_pressed(MouseButton::Left) {
            if state.last_clicked_button_hash.is_none() {
                state.last_clicked_button_hash = Some(hash);
            }
        }

        if is_mouse_button_down(MouseButton::Left) && state.last_clicked_button_hash == Some(hash) {
            pressing = true;
            draw_ui_three_patch(position, width, &sprites[1]);
        } else {
            draw_ui_three_patch(position, width, &sprites[0]);
        }

        if is_mouse_button_released(MouseButton::Left) {
            let prev_hash = state.last_clicked_button_hash;
            state.last_clicked_button_hash = None;
            pressed = prev_hash == Some(hash);
        } else {
            pressed = false;
        }
    } else if disabled {
        draw_ui_three_patch(position, width, &sprites[2]);
        pressed = false;
    } else {
        draw_ui_three_patch(position, width, &sprites[0]);
        pressed = false;
    }
    
    let text_size = measure_text(name, None, 16, 1.0);
    let text_size = vec2(text_size.width, text_size.height);

    let x = rect.x + (rect.w-text_size.x)/2.0;
    let mut y = rect.y + height/2.0;

    if pressing || disabled {
        y += 1.0;
    }

    if let Some(out_pressing) = out_pressing {
        *out_pressing = pressing;
    }
    
    draw_text(name, x, y, 16.0, WHITE);

    pressed
}
