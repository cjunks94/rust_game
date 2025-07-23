use bevy::prelude::*;
use crate::animation::{AnimationState, AnimationLibrary};

#[derive(Resource, Default)]
pub struct ClickCounter(pub u32);

#[derive(Component)]
pub struct AnimatedCat;

#[derive(Component)]
pub struct CounterText;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClickCounter::default())
            .add_systems(Update, (
                handle_cat_clicks_system,
                update_counter_text_system,
            ));
    }
}

pub fn handle_cat_clicks_system(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut counter: ResMut<ClickCounter>,
    cat_query: Query<(&GlobalTransform, &Sprite), With<AnimatedCat>>,
    mut animation_query: Query<&mut AnimationState, With<AnimatedCat>>,
    animation_library: Res<AnimationLibrary>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = cameras.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                if let Ok((cat_transform, sprite)) = cat_query.get_single() {
                    let size = sprite.custom_size.unwrap_or(Vec2::new(64.0, 64.0)) * cat_transform.compute_transform().scale.xy();
                    let half_size = size / 2.0;
                    let cat_pos = cat_transform.translation().xy();
                    let min = cat_pos - half_size;
                    let max = cat_pos + half_size;
                    
                    if world_pos.x >= min.x && world_pos.x <= max.x && world_pos.y >= min.y && world_pos.y <= max.y {
                        counter.0 += 1;
                        if let Ok(mut state) = animation_query.get_single_mut() {
                            // Play cute animation, then return to idle after 2 seconds
                            state.play_animation_then_return("cute", "idle", 2.0, &animation_library);
                        }
                    }
                }
            }
        }
    }
}

pub fn update_counter_text_system(
    mut text_query: Query<&mut Text, With<CounterText>>, 
    counter: Res<ClickCounter>
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!("Clicks: {}", counter.0);
    }
}

pub fn setup_game_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Load the sprite sheet texture
    let texture = asset_server.load("cat_spritesheet.png");

    // Create texture atlas layout
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
        AnimationState::default(),
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