use bevy::prelude::*;
use bevy::math::UVec2;
use std::collections::HashMap;

#[derive(Resource, Default)]
struct ClickCounter(u32);

#[derive(Component)]
struct AnimatedCat;

#[derive(Component)]
struct CounterText;

#[derive(Resource)]
struct DebugMode {
    enabled: bool,
}

#[derive(Component)]
struct DebugOverlay;

#[derive(Clone)]
struct Animation {
    name: String,
    frames: Vec<usize>,  // List of frame indices
    frame_duration: f32, // Duration per frame in seconds
}

#[derive(Resource)]
struct AnimationLibrary {
    animations: HashMap<String, Animation>,
}

impl AnimationLibrary {
    fn new() -> Self {
        let mut animations = HashMap::new();
        
        // Define animations based on sprite sheet grid (8 columns x 9 rows)
        // Row 0: Idle animation
        animations.insert("idle".to_string(), Animation {
            name: "idle".to_string(),
            frames: (0..6).collect(), // First 6 frames of row 0
            frame_duration: 0.5,
        });
        
        // Row 1: Walk animation
        animations.insert("walk".to_string(), Animation {
            name: "walk".to_string(),
            frames: (8..11).collect(), // First 3 frames of row 1
            frame_duration: 0.2,
        });
        
        // Row 2: Sleep animation
        animations.insert("sleep".to_string(), Animation {
            name: "sleep".to_string(),
            frames: vec![16, 17, 18, 19], // Row 2, columns 0-3
            frame_duration: 1.0,
        });
        
        // Row 3: Grooming animation
        animations.insert("groom".to_string(), Animation {
            name: "groom".to_string(),
            frames: (24..34).collect(), // Row 3, columns 0-9 (10 frames)
            frame_duration: 0.15,
        });
        
        // Row 4: Play animation
        animations.insert("play".to_string(), Animation {
            name: "play".to_string(),
            frames: (32..38).collect(), // Row 4, first 6 frames
            frame_duration: 0.1,
        });
        
        // Row 5: Jump animation
        animations.insert("jump".to_string(), Animation {
            name: "jump".to_string(),
            frames: (40..48).collect(), // Row 5, all 8 frames
            frame_duration: 0.1,
        });
        
        // Row 6: Box cat (cute) animation
        animations.insert("cute".to_string(), Animation {
            name: "cute".to_string(),
            frames: (48..56).collect(), // Row 6, all 8 frames
            frame_duration: 0.15,
        });
        
        // Row 7: More box cats
        animations.insert("box_play".to_string(), Animation {
            name: "box_play".to_string(),
            frames: (56..64).collect(), // Row 7, all 8 frames
            frame_duration: 0.2,
        });
        
        AnimationLibrary { animations }
    }
    
    fn get(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }
}

#[derive(Component)]
struct AnimationState {
    current_animation: String,
    current_frame: usize,
    timer: Timer,
    next_animation: Option<(String, Timer)>, // Animation to play after timer expires
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets/cat_black".into(),
            ..default()
        }))
        .insert_resource(ClickCounter(0))
        .insert_resource(DebugMode { enabled: false })
        .insert_resource(AnimationLibrary::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            animate_sprite, 
            update_counter, 
            update_text,
            toggle_debug,
            (update_debug_overlay, update_debug_text).chain(),
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load the sprite sheet texture
    let texture = asset_server.load("cat_spritesheet.png");


    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8, // Columns
        9, // Rows
        None,
        None,
    );

    let layout_handle = layouts.add(texture_atlas_layout);

    // Spawn the animated cat as a sprite
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::splat(4.0)), // Scale up the sprite
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)), // Set exact sprite size
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: layout_handle,
            index: 0, // Start with the first frame
        },
        AnimatedCat,
        AnimationState {
            current_animation: "idle".to_string(),
            current_frame: 0,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            next_animation: None,
        },
    ));

    // Spawn the counter text (UI)
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Clicks: 0",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ))
                .insert(CounterText);
        });
}

fn animate_sprite(
    time: Res<Time>,
    animation_library: Res<AnimationLibrary>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlas), With<AnimatedCat>>,
) {
    for (mut state, mut atlas) in &mut query {
        // Handle animation transition timer
        if let Some((next_anim_name, timer)) = &mut state.next_animation {
            timer.tick(time.delta());
            if timer.just_finished() {
                // Switch to next animation
                state.current_animation = next_anim_name.clone();
                state.current_frame = 0;
                state.next_animation = None;
                
                // Update timer for new animation
                if let Some(animation) = animation_library.get(&state.current_animation) {
                    state.timer = Timer::from_seconds(animation.frame_duration, TimerMode::Repeating);
                }
            }
        }
        
        // Get current animation data
        if let Some(animation) = animation_library.get(&state.current_animation) {
            // Handle frame timing
            state.timer.tick(time.delta());
            if state.timer.just_finished() {
                state.current_frame = (state.current_frame + 1) % animation.frames.len();
            }
            
            // Update texture atlas index
            if let Some(&frame_index) = animation.frames.get(state.current_frame) {
                atlas.index = frame_index;
            }
        }
    }
}

fn update_counter(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut counter: ResMut<ClickCounter>,
    cat_query: Query<(&GlobalTransform, &Sprite), With<AnimatedCat>>,
    mut animation_query: Query<&mut AnimationState, With<AnimatedCat>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = cameras.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                if let Ok((cat_transform, sprite)) = cat_query.get_single() {
                    let size = sprite.custom_size.unwrap_or(Vec2::new(32.0, 32.0)) * cat_transform.compute_transform().scale.xy();
                    let half_size = size / 2.0;
                    let cat_pos = cat_transform.translation().xy();
                    let min = cat_pos - half_size;
                    let max = cat_pos + half_size;
                    if world_pos.x >= min.x && world_pos.x <= max.x && world_pos.y >= min.y && world_pos.y <= max.y {
                        counter.0 += 1;
                        if let Ok(mut state) = animation_query.get_single_mut() {
                            // Play cute animation, then return to idle after 2 seconds
                            state.current_animation = "cute".to_string();
                            state.current_frame = 0;
                            state.timer = Timer::from_seconds(0.15, TimerMode::Repeating);
                            state.next_animation = Some((
                                "idle".to_string(),
                                Timer::from_seconds(2.0, TimerMode::Once)
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn update_text(mut text_query: Query<&mut Text, With<CounterText>>, counter: Res<ClickCounter>) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!("Clicks: {}", counter.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_counter_default() {
        let counter = ClickCounter::default();
        assert_eq!(counter.0, 0);
    }

    #[test]
    fn test_click_counter_increment() {
        let mut counter = ClickCounter(5);
        counter.0 += 1;
        assert_eq!(counter.0, 6);
    }

    #[test]
    fn test_animation_state_default() {
        let state = AnimationState::default();
        assert_eq!(state.base_index, 0);
        assert_eq!(state.current_frame, 0);
        assert_eq!(state.total_frames, 0);
        assert_eq!(state.frame_rate, 0.0);
    }

    #[test]
    fn test_animation_state_custom() {
        let state = AnimationState {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            base_index: 16,
            current_frame: 0,
            total_frames: 8,
            frame_rate: 0.1,
            reset_timer: None,
        };
        assert_eq!(state.base_index, 16);
        assert_eq!(state.total_frames, 8);
        assert_eq!(state.frame_rate, 0.1);
    }
}

fn toggle_debug(
    keyboard: Res<ButtonInput<KeyCode>>,
    animation_library: Res<AnimationLibrary>,
    mut debug_mode: ResMut<DebugMode>,
    mut commands: Commands,
    debug_overlays: Query<Entity, With<DebugOverlay>>,
    mut cat_query: Query<&mut AnimationState, With<AnimatedCat>>,
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
                (KeyCode::Digit3, "sleep"),
                (KeyCode::Digit4, "groom"),
                (KeyCode::Digit5, "play"),
                (KeyCode::Digit6, "jump"),
                (KeyCode::Digit7, "cute"),
                (KeyCode::Digit8, "box_play"),
            ];
            
            for (key, anim_name) in animations {
                if keyboard.just_pressed(key) {
                    println!("Playing {} animation", anim_name);
                    state.current_animation = anim_name.to_string();
                    state.current_frame = 0;
                    state.next_animation = None;
                    
                    // Set correct timer for the new animation
                    if let Some(animation) = animation_library.get(anim_name) {
                        state.timer = Timer::from_seconds(animation.frame_duration, TimerMode::Repeating);
                    }
                    break;
                }
            }
        }
    }
}

fn update_debug_overlay(
    debug_mode: Res<DebugMode>,
    mut commands: Commands,
    cat_query: Query<(&TextureAtlas, &AnimationState, &Handle<Image>), With<AnimatedCat>>,
    _atlas_layouts: Res<Assets<TextureAtlasLayout>>,
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
        let texture = asset_server.load("cat_spritesheet.png");
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

fn update_debug_text(
    debug_mode: Res<DebugMode>,
    animation_library: Res<AnimationLibrary>,
    mut text_query: Query<&mut Text, With<DebugOverlay>>,
    cat_query: Query<(&TextureAtlas, &AnimationState), With<AnimatedCat>>,
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
