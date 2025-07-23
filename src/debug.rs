use bevy::prelude::*;
use crate::animation::{AnimationLibrary, AnimationState};

#[derive(Resource)]
pub struct DebugMode {
    pub enabled: bool,
}

impl Default for DebugMode {
    fn default() -> Self {
        Self { enabled: false }
    }
}

#[derive(Component)]
pub struct DebugOverlay;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugMode::default())
            .add_systems(Update, (
                toggle_debug_system,
                update_debug_overlay_system,
                update_debug_text_system,
            ).chain());
    }
}

pub fn toggle_debug_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    animation_library: Res<AnimationLibrary>,
    mut debug_mode: ResMut<DebugMode>,
    mut commands: Commands,
    debug_overlays: Query<Entity, With<DebugOverlay>>,
    mut cat_query: Query<&mut AnimationState>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        debug_mode.enabled = !debug_mode.enabled;
        println!("Debug mode: {}", debug_mode.enabled);
        
        if !debug_mode.enabled {
            // Remove all debug overlays
            for entity in debug_overlays.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
    
    // Animation testing shortcuts (only in debug mode)
    if debug_mode.enabled {
        if let Ok(mut state) = cat_query.get_single_mut() {
            let animations = [
                (KeyCode::Digit1, "idle"),
                (KeyCode::Digit2, "walk"),
                (KeyCode::Digit3, "pancake"),
                (KeyCode::Digit4, "sleep"),
                (KeyCode::Digit5, "play"),
                (KeyCode::Digit6, "run"),
                (KeyCode::Digit7, "jump"),
                (KeyCode::Digit8, "box_play"),
                (KeyCode::Digit9, "dance"),
                (KeyCode::Digit0, "damage"),
            ];
            
            for (key, anim_name) in animations {
                if keyboard.just_pressed(key) {
                    println!("Playing {} animation", anim_name);
                    state.play_animation(anim_name, &animation_library);
                    break;
                }
            }
        }
    }
}

pub fn update_debug_overlay_system(
    debug_mode: Res<DebugMode>,
    mut commands: Commands,
    cat_query: Query<(&TextureAtlas, &AnimationState, &Handle<Image>)>,
    existing_overlays: Query<Entity, With<DebugOverlay>>,
    asset_server: Res<AssetServer>,
) {
    if !debug_mode.enabled {
        return;
    }
    
    // Create debug overlay if it doesn't exist
    if existing_overlays.is_empty() {
        println!("Creating debug overlay");
        
        // Debug info text at the top
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Debug Info",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(1.0, 1.0, 1.0),
                            ..default()
                        },
                    ),
                );
            })
            .insert(DebugOverlay);
        
        // Show the entire sprite sheet with grid overlay
        let texture = asset_server.load("cat_black/cat_spritesheet.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(400.0, 0.0, 10.0)
                    .with_scale(Vec3::splat(0.5)),
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                    ..default()
                },
                ..default()
            },
            DebugOverlay,
        ));
        
        // Draw grid lines
        let grid_size = 20.0; // Half of 40 for the overlay
        let padding = 4.0; // Half of 8
        let total_size = grid_size + padding;
        let start_x = 252.0; // Adjusted for offset
        let start_y = 148.0;
        
        // Draw grid cells for visualization
        for row in 0..10 {
            for col in 0..12 {
                let x = start_x + col as f32 * total_size;
                let y = start_y - row as f32 * total_size;
                
                // Grid cell outline
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(1.0, 0.0, 0.0, 0.5),
                            custom_size: Some(Vec2::new(grid_size, grid_size)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y, 11.0),
                        ..default()
                    },
                    DebugOverlay,
                ));
                
                // Frame number
                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            format!("{}", row * 12 + col),
                            TextStyle {
                                font_size: 10.0,
                                color: Color::srgb(1.0, 1.0, 0.0),
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(x, y, 12.0),
                        ..default()
                    },
                    DebugOverlay,
                ));
            }
        }
    }
    
    // Update debug text  
    if debug_mode.enabled {
        if let Ok((atlas, state, _)) = cat_query.get_single() {
            println!("Frame: {} (animation: {}, current: {})", 
                atlas.index, state.current_animation, state.current_frame);
        }
    }
}

pub fn update_debug_text_system(
    debug_mode: Res<DebugMode>,
    animation_library: Res<AnimationLibrary>,
    mut text_query: Query<&mut Text, With<DebugOverlay>>,
    cat_query: Query<(&TextureAtlas, &AnimationState)>,
) {
    if !debug_mode.enabled {
        return;
    }
    
    if let Ok((atlas, state)) = cat_query.get_single() {
        let next_anim_info = if let Some((next_name, timer)) = &state.next_animation {
            format!("Next: {} in {:.1}s", next_name, timer.remaining_secs())
        } else {
            "Next: None".to_string()
        };
        
        let frame_info = if let Some(animation) = animation_library.get(&state.current_animation) {
            format!("{}/{}", state.current_frame + 1, animation.frames.len())
        } else {
            "Unknown".to_string()
        };
        
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!(
                "Debug Mode (Press D to toggle)\n\
                Current Animation: {}\n\
                Frame Index: {}\n\
                Frame: {}\n\
                {}\n\
                \n\
                Animation Shortcuts:\n\
                1: Idle  2: Walk  3: Sleep  4: Groom\n\
                5: Play  6: Jump  7: Cute   8: BoxPlay\n\
                \n\
                Click on cat for cute animation",
                state.current_animation,
                atlas.index,
                frame_info,
                next_anim_info
            );
        }
    }
}
