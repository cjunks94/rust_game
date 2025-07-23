use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<usize>,  // List of frame indices
    pub frame_duration: f32, // Duration per frame in seconds
}

#[derive(Resource)]
pub struct AnimationLibrary {
    animations: HashMap<String, Animation>,
}

impl AnimationLibrary {
    pub fn new() -> Self {
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
    
    pub fn get(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }
    
    pub fn get_animation_names(&self) -> Vec<&String> {
        self.animations.keys().collect()
    }
}

#[derive(Component)]
pub struct AnimationState {
    pub current_animation: String,
    pub current_frame: usize,
    pub timer: Timer,
    pub next_animation: Option<(String, Timer)>, // Animation to play after timer expires
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_animation: "idle".to_string(),
            current_frame: 0,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            next_animation: None,
        }
    }
}

impl AnimationState {
    pub fn new(animation_name: &str, frame_duration: f32) -> Self {
        Self {
            current_animation: animation_name.to_string(),
            current_frame: 0,
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            next_animation: None,
        }
    }
    
    pub fn play_animation(&mut self, animation_name: &str, animation_library: &AnimationLibrary) {
        self.current_animation = animation_name.to_string();
        self.current_frame = 0;
        self.next_animation = None;
        
        // Set correct timer for the new animation
        if let Some(animation) = animation_library.get(animation_name) {
            self.timer = Timer::from_seconds(animation.frame_duration, TimerMode::Repeating);
        }
    }
    
    pub fn play_animation_then_return(&mut self, animation_name: &str, return_to: &str, duration: f32, animation_library: &AnimationLibrary) {
        self.current_animation = animation_name.to_string();
        self.current_frame = 0;
        self.next_animation = Some((
            return_to.to_string(),
            Timer::from_seconds(duration, TimerMode::Once)
        ));
        
        // Set correct timer for the new animation
        if let Some(animation) = animation_library.get(animation_name) {
            self.timer = Timer::from_seconds(animation.frame_duration, TimerMode::Repeating);
        }
    }
}

pub fn animate_sprite_system(
    time: Res<Time>,
    animation_library: Res<AnimationLibrary>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlas)>,
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