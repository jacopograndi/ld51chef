use bevy::asset::LoadState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use core::f32::consts::PI;
use rand::prelude::*;
use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    app.insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "perfect chef".to_string(),
            width: 1200.,
            height: 700.,
            ..default()
        })
        .init_resource::<SpriteHandles>()
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .add_system_set(
            SystemSet::on_update(AppState::Finished)
                .with_system(use_hand)
                .with_system(move_hand)
                .with_system(refresh_shelf)
                .with_system(restock_shelf)
                .with_system(eat_anim)
                .with_system(refresh_objectives),
        )
        .add_system_to_stage(CoreStage::PreUpdate, mouse_pos)
        .init_resource::<AssetHandles>()
        .init_resource::<AtlasHandles>()
        .init_resource::<Hand>()
        .init_resource::<MousePos>()
        .init_resource::<Score>()
        .add_event::<RefreshShelfEvent>()
        .add_event::<RestockShelfEvent>()
        .add_event::<EatEvent>();

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }

    app.run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Default)]
struct AtlasHandles {
    handles: HashMap<String, Handle<TextureAtlas>>,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("sprites").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    mut atlas_handles: ResMut<AtlasHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut refresh_event: EventWriter<RefreshShelfEvent>,
) {
    let info = Info {
        food: vec![
            FoodInfo {
                sprite: "leg".to_string(),
                flavor: Flavor(HashMap::from([
                    (Taste::Sweet, 0.5),
                    (Taste::Salty, 0.5),
                    (Taste::Savory, 1.0),
                    (Taste::Spicy, 0.333),
                ])),
            },
            FoodInfo {
                sprite: "chili".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Spicy, 3.0)])),
            },
            FoodInfo {
                sprite: "chocolate".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Bitter, 1.0)])),
            },
        ],
    };
    commands.insert_resource(info.clone());

    let res = Vec2::new(1200.0, 700.0);
    let halfres = res / 2.0;

    let camera_bundle = Camera2dBundle::new_with_far(100.0);
    commands.spawn_bundle(camera_bundle);

    let mut atlases = vec![
        vec!["dude", "dude-gnam"],
        vec!["obj0", "obj1", "obj2", "obj3", "obj4", "obj5"],
        vec!["pan"],
    ];
    for food_info in info.food.iter() {
        atlases.push(vec![&food_info.sprite]);
    }
    for i in 0..8 {
        atlases.push(vec![&Taste::as_str(&Taste::from_u32(i))]);
    }
    for atlas in atlases.iter() {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        for name in atlas {
            let handle = asset_server.get_handle("sprites/".to_string() + name + ".png");
            let texture = textures.get(&handle).expect("no texture");
            texture_atlas_builder.add_texture(handle, texture);
        }
        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas.clone());
        atlas_handles
            .handles
            .insert(atlas[0].to_string(), atlas_handle);
    }

    commands
        .spawn()
        .insert(Transform {
            translation: Vec3::new(0.0, halfres.y - 256.0, 1.0),
            ..default()
        })
        .insert(DudePoint);

    let atlas_handle = atlas_handles.handles.get("dude").unwrap();
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, halfres.y - 256.0, 1.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Dude {
            cycles: 0,
            timer: Timer::new(Duration::from_millis(200), false),
        });
    commands
        .spawn()
        .insert(Transform {
            translation: Vec3::new(0.0, halfres.y - 192.0, 1.0),
            ..default()
        })
        .insert(MouthPoint);

    for i in -1..2 {
        commands
            .spawn()
            .insert(Transform {
                translation: Vec3::new(-halfres.x + 128.0, 200.0 * i as f32, 3.0),
                ..default()
            })
            .insert(Shelf);
    }

    let atlas_handle = atlas_handles.handles.get("pan").unwrap();
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -halfres.y + 128.0, 2.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Pan {});

    for i in 0..8 {
        commands
            .spawn()
            .insert(Transform {
                translation: Vec3::new(
                    halfres.x - 256.0,
                    halfres.y - 550.0 + 500.0 * i as f32 / 8.0,
                    2.0,
                ),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            })
            .insert(ObjectivePoint {
                taste: Taste::from_u32(i),
            });
    }

    refresh_event.send(RefreshShelfEvent { clear: true });
}

#[derive(Default)]
struct MousePos {
    world: Vec2,
}

fn mouse_pos(
    mut cursor_moved_events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    mut mousepos: ResMut<MousePos>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some((camera, camera_transform)) = query_camera.get_single().ok() {
        if let Some(window) = windows.get_primary() {
            for event in cursor_moved_events.iter() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = (event.position / window_size) * 2.0 - Vec2::ONE;
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
                let world_pos: Vec2 = world_pos.truncate();
                mousepos.world = world_pos;
            }
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
enum Taste {
    #[default]
    Sweet,
    Sour,
    Salty,
    Bitter,
    Savory,
    Spicy,
    Cool,
    Dry,
}

impl Taste {
    fn from_u32(value: u32) -> Taste {
        match value {
            0 => Taste::Sweet,
            1 => Taste::Sour,
            2 => Taste::Salty,
            3 => Taste::Bitter,
            4 => Taste::Savory,
            5 => Taste::Spicy,
            6 => Taste::Cool,
            7 => Taste::Dry,
            _ => panic!("Unknown value: {}", value),
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Taste::Sweet => "Sweet",
            Taste::Sour => "Sour",
            Taste::Salty => "Salty",
            Taste::Bitter => "Bitter",
            Taste::Savory => "Savory",
            Taste::Spicy => "Spicy",
            Taste::Cool => "Cool",
            Taste::Dry => "Dry",
        }
    }
}

#[derive(Clone, Default, Debug)]
struct Flavor(HashMap<Taste, f32>);

impl Add for Flavor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = Self::default();
        for i in 0..8 {
            let taste = Taste::from_u32(i);
            let value = self.0.get(&taste).unwrap_or(&0.0) + other.0.get(&taste).unwrap_or(&0.0);
            sum.0.insert(taste, value);
        }
        sum
    }
}

#[derive(Clone)]
struct FoodInfo {
    sprite: String,
    flavor: Flavor,
}

#[derive(Clone)]
struct Info {
    food: Vec<FoodInfo>,
}

#[derive(Component)]
struct MouthPoint;

#[derive(Component)]
struct Pan {}

#[derive(Component)]
struct Dude {
    timer: Timer,
    cycles: u32,
}

#[derive(Component)]
struct Objective {
    taste: Taste,
}

#[derive(PartialEq)]
enum FoodState {
    Shelved,
    Held,
    Eaten,
    Cooking,
}

#[derive(Default)]
struct Score {
    successes: u32,
    losses: u32,
}

struct RefreshShelfEvent {
    clear: bool,
}

struct RestockShelfEvent {
    shelf: Entity,
}

#[derive(Component)]
struct ObjectivePoint {
    taste: Taste,
}

struct EatEvent {}

#[derive(Component)]
struct Food {
    state: FoodState,
    shelf: Entity,
    info: FoodInfo,
}

#[derive(Default)]
struct Hand {
    holding: Option<Entity>,
}

fn eat_anim(
    mut eat_event_read: EventReader<EatEvent>,
    mut dude_query: Query<(&mut Dude, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    let (mut dude, mut id) = dude_query.single_mut();
    for event in eat_event_read.iter() {
        dude.cycles += 4;
        dude.timer.reset();
    }
    dude.timer.tick(time.delta());
    if dude.timer.finished() {
        if dude.cycles > 0 {
            id.index = if id.index == 0 { 1 } else { 0 };
            dude.cycles -= 1;
            dude.timer.reset();
        }
    }
}

fn refresh_objectives(
    mut commands: Commands,
    obj_query: Query<&Objective>,
    food_query: Query<&Food>,
    obj_points: Query<(&ObjectivePoint, &Transform)>,
    atlas_handles: ResMut<AtlasHandles>,
) {
    let mut sum = Flavor::default();
    for food in food_query.iter() {
        if food.state == FoodState::Eaten {
            sum = sum + food.info.flavor.clone();
        }
    }

    for (points, tr) in &obj_points {
        let value = *sum.0.get(&points.taste).unwrap_or(&0.0);

        let mut sum = 0;
        for obj in &obj_query {
            if obj.taste == points.taste {
                sum += 1
            }
        }

        for i in sum..(value as u32) {
            let atlas_handle = atlas_handles.handles.get(points.taste.as_str()).unwrap();
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform {
                        translation: tr.translation
                            + Vec3::new(i as f32 * 10.0, 0.0, i as f32 * 0.01),
                        scale: Vec3::new(0.5, 0.5, 0.5),
                        ..default()
                    },
                    ..default()
                })
                .insert(Objective {
                    taste: points.taste.clone(),
                });
        }
    }
}

fn use_hand(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mousepos: Res<MousePos>,
    mut hand: ResMut<Hand>,
    mut food_query: Query<(Entity, &mut Food, &Transform)>,
    pan_query: Query<(&Pan, &Transform)>,
    mouth_query: Query<(&MouthPoint, &Transform)>,
    mut refresh_event: EventWriter<RefreshShelfEvent>,
    mut eat_event: EventWriter<EatEvent>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state.is_pressed() {
            if let Some(held) = hand.holding {
                let (_mouth, mouth_tr) = mouth_query.get_single().unwrap();
                let (_pan, pan_tr) = pan_query.get_single().unwrap();
                if mousepos
                    .world
                    .distance_squared(mouth_tr.translation.truncate())
                    < 150.0 * 150.0
                {
                    let (_, mut food, _) = food_query.get_mut(held).unwrap();
                    food.state = FoodState::Eaten;
                    dbg!("event in mouth!");
                    hand.holding = None;
                    refresh_event.send(RefreshShelfEvent { clear: true });
                    eat_event.send(EatEvent {});
                } else if mousepos
                    .world
                    .distance_squared(pan_tr.translation.truncate())
                    < 200.0 * 200.0
                {
                    let (_, mut food, _) = food_query.get_mut(held).unwrap();
                    food.state = FoodState::Cooking;
                    dbg!("event in pan!");
                    hand.holding = None;
                    refresh_event.send(RefreshShelfEvent { clear: false });
                    eat_event.send(EatEvent {});
                }
            } else {
                for (ent, mut food, tr) in &mut food_query {
                    if food.state == FoodState::Shelved
                        && mousepos.world.distance_squared(tr.translation.truncate())
                            < 100.0 * 100.0
                    {
                        food.state = FoodState::Held;
                        hand.holding = Some(ent);
                    }
                }
            }
        }
    }
}

fn move_hand(mut commands: Commands, mousepos: Res<MousePos>, hand: Res<Hand>) {
    if let Some(held) = hand.holding {
        commands.entity(held).insert(Transform {
            translation: Vec3::new(mousepos.world.x, mousepos.world.y, 3.0),
            ..default()
        });
    }
}

#[derive(Default)]
struct AssetHandles {
    images: HashMap<String, Handle<Image>>,
}

#[derive(Component)]
struct DudePoint;

#[derive(Component)]
struct Shelf;

fn refresh_shelf(
    mut commands: Commands,
    foods: Query<(Entity, &Food)>,
    shelves: Query<(Entity, &Shelf)>,
    mut refresh_event: EventReader<RefreshShelfEvent>,
    mut restock_event: EventWriter<RestockShelfEvent>,
) {
    for event in refresh_event.iter() {
        let mut occupied: Vec<Entity> = vec![];
        for (ent, food) in &foods {
            if food.state == FoodState::Shelved {
                if event.clear {
                    occupied.push(food.shelf);
                } else {
                    commands.entity(ent).despawn();
                }
            }
        }
        for (ent, _shelf) in &shelves {
            if !occupied.contains(&ent) {
                restock_event.send(RestockShelfEvent { shelf: ent });
            }
        }
    }
}

fn restock_shelf(
    mut commands: Commands,
    shelves: Query<(Entity, &Shelf, &Transform)>,
    mut restock_event: EventReader<RestockShelfEvent>,
    asset_server: Res<AssetServer>,
    mut atlas_handles: ResMut<AtlasHandles>,
    info: Res<Info>,
) {
    for event in restock_event.iter() {
        let (ent, _shelf, tr) = shelves.get(event.shelf).unwrap();

        let mut rng = thread_rng();
        let food_info = &info.food[rng.gen_range(0..info.food.len())];

        let atlas_handle = atlas_handles.handles.get(&food_info.sprite).unwrap();
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: atlas_handle.clone(),
                transform: Transform {
                    translation: tr.translation,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Food {
                state: FoodState::Shelved,
                shelf: ent,
                info: food_info.clone(),
            });
    }
}
