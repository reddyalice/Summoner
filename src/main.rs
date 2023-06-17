
use bevy::window::{CompositeAlphaMode, PresentMode};
use summoner::prelude::*;


fn main() {
    App::new()   
        .insert_resource(ClearColor(Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Summoner".into(),
                    present_mode : PresentMode::AutoNoVsync,
                    //transparent : true,
                    //composite_alpha_mode : CompositeAlphaMode::Auto, 
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .build()
        )
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::None)
        )
        .add_plugin(CameraPlugin)
        .add_plugin(DuelPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(start_duel.in_set(OnUpdate(GameState::None)))
        .add_system(end_duel.in_set(OnUpdate(GameState::Duel)))
        .run();
}


pub fn start_duel(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
){
    if input.just_pressed(KeyCode::K) {
        
        game_state.set(GameState::Duel);
    }
}

pub fn end_duel(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
){
    if input.just_pressed(KeyCode::P) {
        game_state.set(GameState::None);
    }
}

