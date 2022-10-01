use bevy::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_system(setup)
        .init_resource::<AssetHandles>();

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }

    app.run();
}

#[derive(Default)]
struct AssetHandles {
    images: HashMap<String, Handle<Image>>,
}

fn setup(
    mut commands: Commands,
    mut handles: ResMut<AssetHandles>,
    asset_server: Res<AssetServer>,
) {
    let camera_bundle = Camera2dBundle::new_with_far(100.0);
    commands.spawn_bundle(camera_bundle);
    handles
        .images
        .insert("dude".to_string(), asset_server.load("dude.png"));

    commands.spawn_bundle(SpriteBundle {
        texture: handles.images.get("dude").unwrap().clone_weak(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    });
}
