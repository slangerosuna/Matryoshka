use bevy::{
    prelude::*,
};
use cgmath::{
    Matrix2,
    SquareMatrix
};
struct ObjectData {
    model_path: &str,
    texture_path: &str,
    transform: Transform,
}

static MODELS_TO_LOAD: [ObjectData; 1] = [
    ObjectData { model_path: "", texture_path: "",
    transform: Transform {
        translation: Vec3::new(0.0, 0.0, 0.0),
        ..default()
    }}
];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(render_setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn render_setup(mut commands: Commands){
    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3::new(5.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(control_system);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("setup");

    commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 100.0,
                shadows_enabled: true,
                range: 1000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
    });

    for model in MODELS_TO_LOAD {
        let texture_handle = asset_server.load(model.texture_path);
        let model_handle = asset_server.load(model.model_path);

        //TODO model loading
        
        commands.spawn (
            (
            PbrBundle {
                transform: model.transform,
                mesh: model_handle.clone(),
                material: materials.add(
                    StandardMaterial {
                        base_color_texture: Some(texture_handle.clone()),
                        ..default()
                    }
                ),
                ..default()
            }
            )
        );
    }
}

fn control_system(

) {

}
