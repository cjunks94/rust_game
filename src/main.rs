use bevy::prelude::*;

mod animation;
mod debug;
mod game;

use animation::{AnimationLibrary, animate_sprite_system};
use debug::DebugPlugin;
use game::{GamePlugin, setup_game_entities};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets".into(),
            ..default()
        }))
        .insert_resource(AnimationLibrary::new())
        .add_plugins(GamePlugin)
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup_game_entities)
        .add_systems(Update, animate_sprite_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use animation::AnimationState;

    #[test]
    fn test_click_counter_default() {
        let counter = game::ClickCounter::default();
        assert_eq!(counter.0, 0);
    }

    #[test]
    fn test_click_counter_increment() {
        let mut counter = game::ClickCounter(5);
        counter.0 += 1;
        assert_eq!(counter.0, 6);
    }

    #[test]
    fn test_animation_state_default() {
        let state = AnimationState::default();
        assert_eq!(state.current_animation, "idle");
        assert_eq!(state.current_frame, 0);
    }

    #[test]
    fn test_animation_state_new() {
        let state = AnimationState::new("walk", 0.2);
        assert_eq!(state.current_animation, "walk");
        assert_eq!(state.current_frame, 0);
    }
}
