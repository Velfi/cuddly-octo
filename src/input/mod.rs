use super::*;

pub fn handle_key_down(state: &mut MainState, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
        Keycode::Up => {
            state.player_y -= 1.0 * state.player_move_speed;
            state.player_needs_update = true;
        },
        Keycode::Right => {
            state.player_x += 1.0 * state.player_move_speed;
            state.player_needs_update = true;
        },
        Keycode::Down => {
            state.player_y += 1.0 * state.player_move_speed;
            state.player_needs_update = true;
        },
        Keycode::Left => {
            state.player_x -= 1.0 * state.player_move_speed;
            state.player_needs_update = true;
        },
        _ => (), // Do nothing
    }
}

pub fn handle_key_up(state: &mut MainState, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            // Keycode::Up => {
            //     println!("up");
            // },
            // Keycode::Space => {
            //     self.text = graphics::Text::new(ctx, "Spacebar pressed", &self.font).unwrap()
            // },
            Keycode::Escape => _ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
