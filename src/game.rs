use bevy::prelude::*;
use crate::animation::{AnimationState, AnimationLibrary};
use rand::Rng;

#[derive(Resource, Default)]
pub struct ClickCounter(pub u32);

#[derive(Component)]
pub struct AnimatedCat;

#[derive(Component)]
pub struct CounterText;

#[derive(Component)]
pub struct Background;

#[derive(Resource)]
pub struct BackgroundConfig {
    backgrounds: Vec<String>,
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        Self {
            backgrounds: vec![
                "backgrounds/summer 2/Summer2.png".to_string(),
                "backgrounds/summer 3/Summer3.png".to_string(),
                "backgrounds/summer 4/Summer4.png".to_string(),
                "backgrounds/summer5/Summer5.png".to_string(),
                "backgrounds/summer6/Summer6.png".to_string(),
                "backgrounds/summer7/Summer7.png".to_string(),
                "backgrounds/summer8/Summer8.png".to_string(),
            ],
        }
    }
}

impl BackgroundConfig {
    pub fn get_random_background(&self) -> &String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.backgrounds.len());
        &self.backgrounds[index]
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClickCounter::default())
            .insert_resource(BackgroundConfig::default())
            .add_systems(Update, (
                handle_cat_clicks_system,
                update_counter_text_system,
                change_background_on_click_system,
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
                    let size = sprite.custom_size.unwrap_or(Vec2::new(128.0, 128.0)) * cat_transform.compute_transform().scale.xy();
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
    background_config: Res<BackgroundConfig>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn random background
    let background_path = background_config.get_random_background();
    let background_texture: Handle<Image> = asset_server.load(background_path.clone());
    println!("Loading background: {}", background_path);

    commands.spawn((
        SpriteBundle {
            texture: background_texture,
            transform: Transform::from_xyz(0.0, 0.0, -1.0), // Behind everything
            sprite: Sprite {
                // Scale to fit screen - adjust as needed
                custom_size: Some(Vec2::new(1200.0, 800.0)),
                ..default()
            },
            ..default()
        },
        Background,
    ));

    // Load the sprite sheet texture
    let texture = asset_server.load("cat_black/cat_spritesheet.png");

    // Create texture atlas layout
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        12, // Columns
        19, // Rows
        None,
        None,
    );

    let layout_handle = layouts.add(texture_atlas_layout);

    // Spawn the animated cat as a sprite
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::splat(1.2)), // Scale up the sprite
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

pub fn change_background_on_click_system(
    counter: Res<ClickCounter>,
    background_query: Query<(Entity, &Handle<Image>), With<Background>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    background_config: Res<BackgroundConfig>,
) {
    // Change background every 5 clicks
    if counter.0 > 0 && counter.0 % 5 == 0 && counter.is_changed() {
        if let Ok((background_entity, _)) = background_query.get_single() {
            // Get a new random background
            let new_background_path = background_config.get_random_background();
            let new_background_texture: Handle<Image> = asset_server.load(new_background_path.clone());
            println!("Changing background to: {}", new_background_path);

            // Update the background sprite
            commands.entity(background_entity).insert(new_background_texture);
        }
    }
}
