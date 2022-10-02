use bevy::asset::LoadState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
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
        .init_resource::<RawHandles>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state(AppState::Init)
        .add_system_set(SystemSet::on_update(AppState::Init).with_system(init))
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_all))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_all))
        .add_system_set(SystemSet::on_exit(AppState::Setup).with_system(setup))
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_dude))
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(move_hand)
                .with_system(use_hand)
                .with_system(refresh_shelf)
                .with_system(restock_shelf)
                .with_system(eat_anim)
                .with_system(cooking)
                .with_system(spawn_objectives)
                .with_system(move_objs)
                .with_system(match_timers)
                .with_system(update_ui)
                .with_system(update_ui_timer),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Game)
                .with_system(clear_hand)
                .with_system(ready_anim_objs),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Reward)
                .with_system(pan_anim)
                .with_system(eat_anim)
                .with_system(move_objs)
                .with_system(match_timers)
                .with_system(update_ui)
                .with_system(update_ui_timer)
                .with_system(tally),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Reward)
                .with_system(pan_reset)
                .with_system(reset_ui),
        )
        .add_system_to_stage(CoreStage::PreUpdate, mouse_pos)
        .init_resource::<AtlasHandles>()
        .init_resource::<AudioHandles>()
        .init_resource::<Hand>()
        .init_resource::<MousePos>()
        .init_resource::<Score>()
        .init_resource::<MatchTimers>()
        .insert_resource(Difficulty { threshold: 30 })
        .add_event::<RefreshShelfEvent>()
        .add_event::<RestockShelfEvent>()
        .add_event::<EatEvent>()
        .add_event::<PanSmashEvent>();

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }

    app.run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Init,
    Setup,
    Game,
    Reward,
}

fn init(mut commands: Commands, mut state: ResMut<State<AppState>>) {
    let info = Info {
        atlases: vec![],
        food: vec![
            FoodInfo {
                sprite: "leg".to_string(),
                flavor: Flavor(HashMap::from([
                    (Taste::Sweet, 1.0),
                    (Taste::Salty, 1.0),
                    (Taste::Savory, 2.0),
                    (Taste::Spicy, 1.0),
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
            FoodInfo {
                sprite: "ice-cream".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Dry, 1.0), (Taste::Cool, 2.0)])),
            },
            FoodInfo {
                sprite: "fish".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Savory, 1.0), (Taste::Salty, 1.0)])),
            },
            FoodInfo {
                sprite: "coffee".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Dry, 1.0), (Taste::Bitter, 2.0)])),
            },
            FoodInfo {
                sprite: "lemon".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Sour, 3.0)])),
            },
            FoodInfo {
                sprite: "cheese".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Sweet, 1.0), (Taste::Savory, 1.0)])),
            },
            FoodInfo {
                sprite: "cinnamon".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Dry, 1.0), (Taste::Spicy, 1.0)])),
            },
            FoodInfo {
                sprite: "mint".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Cool, 2.0)])),
            },
            FoodInfo {
                sprite: "apple".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Sweet, 1.0)])),
            },
            FoodInfo {
                sprite: "blueberry".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Sweet, 2.0), (Taste::Sour, 1.0)])),
            },
            FoodInfo {
                sprite: "feather".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Dry, 2.0)])),
            },
            FoodInfo {
                sprite: "gas".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Spicy, 2.0), (Taste::Bitter, 2.0)])),
            },
            FoodInfo {
                sprite: "dynamite".to_string(),
                flavor: Flavor(HashMap::from([(Taste::Spicy, 2.0), (Taste::Savory, 1.0)])),
            },
            FoodInfo {
                sprite: "chips".to_string(),
                flavor: Flavor(HashMap::from([
                    (Taste::Sweet, 1.0),
                    (Taste::Salty, 3.0),
                    (Taste::Dry, 1.0),
                ])),
            },
            FoodInfo {
                sprite: "onion".to_string(),
                flavor: Flavor(HashMap::from([
                    (Taste::Sweet, 1.0),
                    (Taste::Sour, 2.0),
                    (Taste::Spicy, 1.0),
                ])),
            },
        ],
        dude: vec![
            DudeInfo {
                sprite: "dude".to_string(),
            },
            DudeInfo {
                sprite: "ale".to_string(),
            },
            DudeInfo {
                sprite: "elena".to_string(),
            },
        ],
    };
    commands.insert_resource(info);
    state.set(AppState::Setup).unwrap();
}

#[derive(Default)]
struct RawHandles {
    sprites: Vec<Handle<Image>>,
    audio: Vec<Handle<AudioSource>>,
}

#[derive(Default)]
struct AudioHandles {
    handles: HashMap<String, Handle<AudioSource>>,
}

#[derive(Default)]
struct AtlasHandles {
    handles: HashMap<String, Handle<TextureAtlas>>,
}

fn load_all(
    mut raw_handles: ResMut<RawHandles>,
    asset_server: Res<AssetServer>,
    mut audio_handles: ResMut<AudioHandles>,
    mut info: ResMut<Info>,
) {
    let mut atlases = vec![
        vec!["dude", "dude-gnam", "dude-puke"],
        vec!["elena", "elena-gnam", "elena-puke"],
        vec!["ale", "ale-gnam", "ale-puke"],
        vec!["pan", "pan-anim1", "pan-anim2"],
        vec!["Time"],
        vec!["guuut"],
        vec!["bad"],
        vec!["resist"],
        vec!["stomach"],
        vec!["goal"],
        vec!["pan-icon"],
        vec!["background"],
    ];
    let food = info.food.clone();
    for food_info in food.iter() {
        atlases.push(vec![&food_info.sprite]);
    }
    for i in 0..8 {
        atlases.push(vec![&Taste::as_str(&Taste::from_u32(i))]);
    }
    for atlas in atlases {
        let mut v = vec![];
        for name in atlas {
            let path = "sprites/".to_string() + name + ".png";
            raw_handles.sprites.push(asset_server.load(&path));
            v.push(name.to_string());
        }
        info.atlases.push(v);
    }
    let audio_names = vec![
        "lol",
        "elena-gnam",
        "elena-puke",
        "elena-yeah",
        "dude-gnam",
        "dude-puke",
        "dude-yeah",
        "ale-gnam",
        "ale-puke",
        "ale-yeah",
    ];
    for name in audio_names {
        let handle = asset_server.load(&("audio/".to_string() + name + ".ogg"));
        raw_handles.audio.push(handle.clone());
        audio_handles.handles.insert(name.to_string(), handle);
    }
}

fn check_all(
    mut state: ResMut<State<AppState>>,
    handles: ResMut<RawHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(handles.sprites.iter().map(|handle| handle.id))
    {
        if let LoadState::Loaded =
            asset_server.get_group_load_state(handles.audio.iter().map(|handle| handle.id))
        {
            state.set(AppState::Game).unwrap();
        }
    }
}

fn setup(
    mut commands: Commands,
    mut atlas_handles: ResMut<AtlasHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut refresh_event: EventWriter<RefreshShelfEvent>,
    info: Res<Info>,
    audio_handles: Res<AudioHandles>,
    audio: Res<Audio>,
) {
    let res = Vec2::new(1200.0, 700.0);
    let halfres = res / 2.0;

    let camera_bundle = Camera2dBundle::new_with_far(100.0);
    commands.spawn_bundle(camera_bundle);

    for atlas in info.atlases.iter() {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        for name in atlas {
            let handle = asset_server.get_handle("sprites/".to_string() + name + ".png");
            let texture = textures
                .get(&handle)
                .expect(&("no texture ".to_string() + name));
            texture_atlas_builder.add_texture(handle, texture);
        }
        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas.clone());
        atlas_handles
            .handles
            .insert(atlas[0].to_string(), atlas_handle);
    }
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: atlas_handles.handles.get("background").unwrap().clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: atlas_handles.handles.get("guuut").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(150.0, halfres.y - 256.0 + 64.0, 1.0),
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        })
        .insert(DudePreferencePoint {
            preference: Preference::Like,
        });
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: atlas_handles.handles.get("bad").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(150.0, halfres.y - 256.0, 1.0),
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        })
        .insert(DudePreferencePoint {
            preference: Preference::Dislike,
        });
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: atlas_handles.handles.get("resist").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(150.0, halfres.y - 256.0 - 64.0, 1.0),
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        })
        .insert(DudePreferencePoint {
            preference: Preference::Resist,
        });

    commands
        .spawn()
        .insert(Transform {
            translation: Vec3::new(0.0, halfres.y - 256.0, 0.1),
            ..default()
        })
        .insert(DudePoint);

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

    let atlas_handle = atlas_handles.handles.get("Time").unwrap();
    for i in 0..10 {
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        500.0 / 2.0 - 500.0 / 20.0 - 500.0 * i as f32 / 10.0,
                        halfres.y - 48.0,
                        2.9 - i as f32 / 20.0,
                    ),
                    scale: Vec3::splat(0.5),
                    ..default()
                },
                ..default()
            })
            .insert(UiTimer { num: i });
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
        .insert(ObjectivePoint {
            zone: ObjectiveZone::Pan,
        })
        .insert(Pan {
            timer: Timer::new(Duration::from_millis(2000), false),
            from: Vec3::new(0.0, -halfres.y + 128.0, 2.0),
            goto: Vec3::new(0.0, halfres.y - 192.0, 2.0),
            smashed: false,
        });

    let font = asset_server.load("fonts/SztyletBd.ttf");
    let style = TextStyle {
        font: font.clone(),
        font_size: 72.0,
        color: Color::BLACK,
    };
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: atlas_handles.handles.get("goal").unwrap().clone(),
        transform: Transform {
            translation: Vec3::new(halfres.x - 64.0, halfres.y - 128.0, 1.0),
            scale: Vec3::splat(0.5),
            ..default()
        },
        ..default()
    });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", style.clone()).with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: Vec3::new(halfres.x - 64.0, halfres.y - 128.0, 7.0),
                ..default()
            },
            ..default()
        })
        .insert(UiTag {
            name: UiName::Palate,
        });

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: atlas_handles.handles.get("stomach").unwrap().clone(),
        transform: Transform {
            translation: Vec3::new(-20.0 + halfres.x - 64.0, 0.0, 0.1),
            scale: Vec3::splat(0.5),
            ..default()
        },
        ..default()
    });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", style.clone()).with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: Vec3::new(halfres.x - 64.0, 0.0, 7.0),
                ..default()
            },
            ..default()
        })
        .insert(UiTag {
            name: UiName::Stomach,
        });

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: atlas_handles.handles.get("pan-icon").unwrap().clone(),
        transform: Transform {
            translation: Vec3::new(halfres.x - 64.0, -halfres.y + 128.0, 1.0),
            scale: Vec3::splat(0.5),
            ..default()
        },
        ..default()
    });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", style.clone()).with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: Vec3::new(halfres.x - 64.0, -halfres.y + 128.0, 7.0),
                ..default()
            },
            ..default()
        })
        .insert(UiTag { name: UiName::Pan });

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "OH YEAH!",
                TextStyle {
                    font: font.clone(),
                    font_size: 144.0,
                    color: Color::DARK_GREEN,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..default()
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(UiTag { name: UiName::Win });

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "DISGUSTING",
                TextStyle {
                    font: font.clone(),
                    font_size: 144.0,
                    color: Color::RED,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..default()
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(UiTag { name: UiName::Lose });

    refresh_event.send(RefreshShelfEvent { clear: true });

    audio.play_with_settings(
        audio_handles.handles.get("lol").unwrap().clone(),
        PlaybackSettings {
            repeat: true,
            volume: 0.1,
            ..default()
        },
    );
}

fn spawn_dude(
    mut commands: Commands,
    atlas_handles: ResMut<AtlasHandles>,
    info: Res<Info>,
    dude_point: Query<(&DudePoint, &Transform)>,
    pref_point: Query<(&DudePreferencePoint, &Transform)>,
    dude_query: Query<(Entity, &Dude)>,
    token_query: Query<(Entity, &PreferenceToken)>,
) {
    let mut rng = thread_rng();
    for (ent, _) in &dude_query {
        commands.entity(ent).despawn();
    }
    for (ent, _) in &token_query {
        commands.entity(ent).despawn();
    }
    let (_, tr) = dude_point.single();

    let palate = Flavor::gen();

    let info = &info.dude[rng.gen_range(0..info.dude.len())];
    let atlas_handle = atlas_handles.handles.get(&info.sprite).unwrap();
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
        .insert(Dude {
            puking: false,
            cycles: 0,
            timer: Timer::new(Duration::from_millis(rng.gen_range(80..150)), false),
            palate: palate.clone(),
            info: info.clone(),
            yeah: false,
        });

    commands
        .spawn()
        .insert(Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(ObjectivePoint {
            zone: ObjectiveZone::Stomach,
        });

    let mut places = Vec::<Preference>::new();
    for i in 0..8 {
        let taste = Taste::from_u32(i);
        let atlas_handle = atlas_handles.handles.get(taste.as_str()).unwrap();

        let pref: f32 = *palate.0.get(&taste).unwrap_or(&0.0);
        let preference = Preference::from_f32(pref);

        if let Some((_, tr)) = pref_point
            .iter()
            .find(|(pt, _)| pt.preference == preference)
            .take()
        {
            let pos = tr.translation
                + Vec3::new(
                    places.iter().filter(|p| **p == preference).count() as f32 * 48.0 + 64.0,
                    0.0,
                    0.0,
                );
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform {
                        translation: pos,
                        scale: Vec3::splat(0.5),
                        ..default()
                    },
                    ..default()
                })
                .insert(PreferenceToken {});
        }
        places.push(preference.clone());
    }
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

impl Flavor {
    fn gen() -> Self {
        let mut rng = thread_rng();
        let mut ret = Self(HashMap::new());
        for i in 0..8 {
            let r = rng.gen_range(0..100);
            let value = if r < 10 {
                0.0
            } else if r < 20 {
                -1.0
            } else if r < 40 {
                2.0
            } else {
                1.0
            };
            ret.0.insert(Taste::from_u32(i), value);
        }
        ret
    }
}

#[derive(Clone)]
struct FoodInfo {
    sprite: String,
    flavor: Flavor,
}

#[derive(Clone)]
struct DudeInfo {
    sprite: String,
}

#[derive(Clone)]
struct Info {
    atlases: Vec<Vec<String>>,
    food: Vec<FoodInfo>,
    dude: Vec<DudeInfo>,
}

#[derive(Component)]
struct MouthPoint;

#[derive(Component)]
struct Pan {
    from: Vec3,
    goto: Vec3,
    timer: Timer,
    smashed: bool,
}

#[derive(Component)]
struct Dude {
    timer: Timer,
    cycles: u32,
    palate: Flavor,
    puking: bool,
    info: DudeInfo,
    yeah: bool,
}

#[derive(Component)]
struct Objective {
    taste: Taste,
    from: Vec3,
    goto: Vec3,
    zone: ObjectiveZone,
    timer: Timer,
}

#[derive(PartialEq)]
enum FoodState {
    Shelved,
    Held,
    Eaten,
    Cooking,
}

struct RefreshShelfEvent {
    clear: bool,
}

struct RestockShelfEvent {
    shelf: Entity,
}

#[derive(PartialEq, Clone)]
enum ObjectiveZone {
    Stomach,
    Pan,
}

#[derive(Component)]
struct ObjectivePoint {
    zone: ObjectiveZone,
}

struct EatEvent {
    from: Vec3,
    flavor: Flavor,
    to_zone: ObjectiveZone,
}

struct PanSmashEvent {}

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

struct MatchTimers {
    game: Timer,
    reward: Timer,
    cook: Timer,
}
impl Default for MatchTimers {
    fn default() -> Self {
        MatchTimers {
            game: Timer::new(Duration::from_secs(10), true),
            reward: Timer::new(Duration::from_secs(5), true),
            cook: Timer::new(Duration::from_secs(1), true),
        }
    }
}

#[derive(Default)]
struct Score {
    successes: u32,
    losses: u32,
}

struct Difficulty {
    threshold: i32,
}

#[derive(PartialEq)]
enum UiName {
    Stomach,
    Palate,
    Pan,
    Win,
    Lose,
}

#[derive(Component)]
struct UiTag {
    name: UiName,
}

#[derive(Component)]
struct UiTimer {
    num: u32,
}

fn match_timers(
    mut theme_lol: ResMut<MatchTimers>,
    time: Res<Time>,
    mut state: ResMut<State<AppState>>,
) {
    if state.current() == &AppState::Game {
        theme_lol.game.tick(time.delta());
        if theme_lol.game.finished() {
            state.set(AppState::Reward).unwrap();
        }
    } else if state.current() == &AppState::Reward {
        theme_lol.reward.tick(time.delta());
        if theme_lol.reward.finished() {
            state.set(AppState::Game).unwrap();
        }
    }
}

fn reset_ui(mut ui_query: Query<(&UiTag, &mut Visibility)>) {
    for (tag, mut vis) in &mut ui_query {
        if tag.name == UiName::Win || tag.name == UiName::Lose {
            vis.is_visible = false;
        }
    }
}

fn tally(
    mut commands: Commands,
    obj_query: Query<(Entity, &Objective)>,
    mut dude_query: Query<&mut Dude>,
    mut score: ResMut<Score>,
    mut difficulty: ResMut<Difficulty>,
    mut smash_event: EventReader<PanSmashEvent>,
    mut ui_query: Query<(&UiTag, &mut Visibility)>,
    mut refresh: EventWriter<RefreshShelfEvent>,
) {
    for _ in smash_event.iter() {
        let mut dude = dude_query.single_mut();

        let mut sum: f32 = 0.0;
        for (ent, obj) in &obj_query {
            let modifier = dude.palate.0.get(&obj.taste).unwrap_or(&1.0);
            let value = 1.0 * modifier;
            sum += value;
            commands.entity(ent).despawn();
            dude.cycles = 0;
        }

        refresh.send(RefreshShelfEvent { clear: false });

        if sum as i32 >= difficulty.threshold {
            score.successes += 1;
            difficulty.threshold += 10;
            if let Some((_, mut vis)) = ui_query
                .iter_mut()
                .find(|(tag, _)| tag.name == UiName::Win)
                .take()
            {
                vis.is_visible = true;
            }
        } else {
            score.losses += 1;
            difficulty.threshold -= 3;
            if let Some((_, mut vis)) = ui_query
                .iter_mut()
                .find(|(tag, _)| tag.name == UiName::Lose)
                .take()
            {
                vis.is_visible = true;
            }
        }
    }
}

fn update_ui(
    mut text_query: Query<(&mut Text, &UiTag)>,
    obj_query: Query<&Objective>,
    difficulty: Res<Difficulty>,
) {
    let mut stomach_sum = 0;
    let mut pan_sum = 0;
    for obj in &obj_query {
        if obj.zone == ObjectiveZone::Stomach {
            stomach_sum += 1
        } else {
            pan_sum += 1
        }
    }
    for (mut text, tag) in &mut text_query {
        match tag.name {
            UiName::Palate => text.sections[0].value = difficulty.threshold.to_string(),
            UiName::Stomach => text.sections[0].value = stomach_sum.to_string(),
            UiName::Pan => text.sections[0].value = pan_sum.to_string(),
            _ => (),
        }
    }
}

fn update_ui_timer(
    state: Res<State<AppState>>,
    theme_lol: ResMut<MatchTimers>,
    mut ui_timer_query: Query<(&UiTimer, &mut Visibility)>,
) {
    let perc = if state.current() == &AppState::Game {
        theme_lol.game.percent()
    } else {
        theme_lol.reward.percent()
    };
    for (uitimer, mut vis) in &mut ui_timer_query {
        if uitimer.num < (perc * 10.0) as u32 {
            vis.is_visible = false;
        } else {
            vis.is_visible = true;
        }
    }
}

fn ready_anim_objs(mut obj_query: Query<&mut Objective>) {
    for mut obj in &mut obj_query {
        if obj.zone == ObjectiveZone::Pan {
            obj.timer.reset();
        }
    }
}

fn pan_reset(mut pan_query: Query<&mut Pan>) {
    let mut pan = pan_query.single_mut();
    pan.smashed = false;
}

fn pan_anim(
    mut obj_query: Query<&mut Objective>,
    mut pan_query: Query<(&mut Pan, &mut Transform, &mut TextureAtlasSprite)>,
    mut event_smash: EventWriter<PanSmashEvent>,
    time: Res<Time>,
    audio_handles: Res<AudioHandles>,
    audio: Res<Audio>,
    mut dude_query: Query<(Entity, &mut Dude)>,
) {
    let (_, mut dude) = dude_query.single_mut();
    let (mut pan, mut tr, mut sprite) = pan_query.single_mut();
    if pan.smashed {
        return;
    }
    pan.timer.tick(time.delta());
    if pan.timer.just_finished() {
        event_smash.send(PanSmashEvent {});
        pan.timer.reset();
        tr.translation = pan.from;
        sprite.index = 0;
        pan.smashed = true;
    } else {
        let t = pan.timer.percent();
        if t > 0.5 && !dude.yeah {
            let audio_name = dude.info.sprite.clone() + "-yeah";
            audio.play(audio_handles.handles.get(&audio_name).unwrap().clone());
            dude.yeah = true;
        }
        let e = f32::min(1.0, t.powi(5) * 32.0);
        tr.translation = pan.from * (1.0 - e) + pan.goto * e;
        if t < 0.25 {
            sprite.index = 0
        } else if t < 0.5 {
            sprite.index = 1
        } else {
            sprite.index = 2
        }
        for mut obj in &mut obj_query {
            if obj.zone == ObjectiveZone::Pan {
                obj.goto = tr.translation + Vec3::new(0.0, 0.0, 1.0);
            }
        }
    }
}

fn move_objs(mut obj_query: Query<(&mut Objective, &mut Transform)>, time: Res<Time>) {
    for (mut obj, mut tr) in &mut obj_query {
        if !obj.timer.finished() {
            obj.timer.tick(time.delta());
            let t = obj.timer.percent();
            tr.translation = obj.goto * t + obj.from * (1.0 - t);
        } else {
            tr.translation = obj.goto;
        }
    }
}

fn cooking(
    mut commands: Commands,
    obj_points: Query<(&ObjectivePoint, &Transform)>,
    mut obj_query: Query<(Entity, &mut Objective, &Transform)>,
    mut match_timers: ResMut<MatchTimers>,
    atlas_handles: ResMut<AtlasHandles>,
    time: Res<Time>,
) {
    let mut rng = thread_rng();
    match_timers.cook.tick(time.delta());
    if match_timers.cook.finished() {
        let mut objs = Vec::<(Entity, Taste)>::new();
        for (ent, obj, _) in &obj_query {
            objs.push((ent, obj.taste.clone()));
        }
        for (ent, mut obj, tr) in &mut obj_query {
            if obj.zone == ObjectiveZone::Stomach {
                if rng.gen_ratio(1, 25) {
                    commands.entity(ent).despawn();
                }
            }
            if obj.zone == ObjectiveZone::Pan {
                if let Some(_) = objs
                    .iter()
                    .find(|(oth, taste)| *oth != ent && obj.taste == *taste)
                    .take()
                {
                    if rng.gen_ratio(24, 25) {
                        continue;
                    }
                    let (_, objtr) = obj_points
                        .iter()
                        .find(|(pt, _)| pt.zone == obj.zone)
                        .take()
                        .unwrap();

                    obj.timer.reset();
                    let from = tr.translation;
                    let goto = objtr.translation
                        + Vec3::new(
                            rng.gen_range(-150..150) as f32,
                            rng.gen_range(-60..80) as f32,
                            rng.gen_range(0..10000) as f32 / 100000.0,
                        );
                    let atlas_handle = atlas_handles.handles.get(obj.taste.as_str()).unwrap();
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(0),
                            texture_atlas: atlas_handle.clone(),
                            transform: Transform {
                                translation: objtr.translation
                                    + Vec3::new(
                                        rng.gen_range(-150..150) as f32,
                                        rng.gen_range(-60..80) as f32,
                                        rng.gen_range(0..10000) as f32 / 100000.0,
                                    ),
                                scale: Vec3::new(0.5, 0.5, 0.5),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Objective {
                            from,
                            goto,
                            taste: obj.taste.clone(),
                            zone: obj.zone.clone(),
                            timer: Timer::new(Duration::from_millis(500), false),
                        });
                }
            }
        }
    }
}

fn eat_anim(
    mut eat_event_read: EventReader<EatEvent>,
    mut dude_query: Query<(&mut Dude, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    audio_handles: Res<AudioHandles>,
    audio: Res<Audio>,
) {
    let (mut dude, mut id) = dude_query.single_mut();
    for event in eat_event_read.iter() {
        dude.puking = false;
        for i in 0..8 {
            let taste = Taste::from_u32(i);
            let val = *event.flavor.0.get(&taste).unwrap_or(&0.0);
            let pref = *dude.palate.0.get(&taste).unwrap_or(&0.0);
            if Preference::from_f32(pref) == Preference::Dislike && val > 0.0 {
                dude.puking = true;
                let audio_name = dude.info.sprite.clone() + "-puke";
                audio.play(audio_handles.handles.get(&audio_name).unwrap().clone());
            }
        }
        dude.cycles += 4;
        dude.timer.reset();
    }
    dude.timer.tick(time.delta());
    if dude.timer.finished() {
        if dude.cycles > 0 {
            id.index = if dude.puking {
                if dude.cycles == 1 {
                    0
                } else {
                    2
                }
            } else {
                if id.index == 0 {
                    let audio_name = dude.info.sprite.clone() + "-gnam";
                    audio.play(audio_handles.handles.get(&audio_name).unwrap().clone());
                    1
                } else {
                    0
                }
            };
            dude.cycles -= 1;
            dude.timer.reset();
        }
    }
}

fn spawn_objectives(
    mut commands: Commands,
    obj_points: Query<(&ObjectivePoint, &Transform)>,
    atlas_handles: ResMut<AtlasHandles>,
    mut eat_event: EventReader<EatEvent>,
) {
    let mut rng = thread_rng();
    for event in eat_event.iter() {
        for i in 0..8 {
            let taste = Taste::from_u32(i);
            let value = *event.flavor.0.get(&taste).unwrap_or(&0.0);

            let (_, objtr) = obj_points
                .iter()
                .find(|(pt, _)| pt.zone == event.to_zone)
                .take()
                .unwrap();

            for _ in 0..(value as u32) {
                let from = event.from
                    + Vec3::new(
                        rng.gen_range(-100..100) as f32,
                        rng.gen_range(-100..100) as f32,
                        0.1,
                    );
                let goto = if event.to_zone == ObjectiveZone::Stomach {
                    objtr.translation
                        + Vec3::new(
                            rng.gen_range(-20..20) as f32,
                            rng.gen_range(-10..10) as f32,
                            rng.gen_range(0..1000) as f32 / 10000.0,
                        )
                } else {
                    objtr.translation
                        + Vec3::new(
                            rng.gen_range(-150..150) as f32,
                            rng.gen_range(-60..80) as f32,
                            rng.gen_range(0..10000) as f32 / 100000.0,
                        )
                };

                let atlas_handle = atlas_handles.handles.get(taste.as_str()).unwrap();
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(0),
                        texture_atlas: atlas_handle.clone(),
                        transform: Transform {
                            translation: event.from,
                            scale: Vec3::new(0.5, 0.5, 0.5),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Objective {
                        from,
                        goto,
                        taste: taste.clone(),
                        zone: event.to_zone.clone(),
                        timer: Timer::new(Duration::from_millis(500), false),
                    });
            }
        }
    }
}

fn use_hand(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mousepos: Res<MousePos>,
    mut hand: ResMut<Hand>,
    mut food_query: Query<(Entity, &mut Food, &Transform)>,
    pan_query: Query<(&Pan, &Transform)>,
    mouth_query: Query<(&MouthPoint, &Transform)>,
    mut refresh_event: EventWriter<RefreshShelfEvent>,
    mut eat_event: EventWriter<EatEvent>,
) {
    for _event in mouse_button_input_events.iter() {
        if let Some(held) = hand.holding {
            let (_mouth, mouth_tr) = mouth_query.get_single().unwrap();
            let (_pan, pan_tr) = pan_query.get_single().unwrap();
            if mousepos
                .world
                .distance_squared(mouth_tr.translation.truncate())
                < 150.0 * 150.0
            {
                let (ent, mut food, tr) = food_query.get_mut(held).unwrap();
                food.state = FoodState::Eaten;
                refresh_event.send(RefreshShelfEvent { clear: true });
                eat_event.send(EatEvent {
                    from: tr.translation,
                    flavor: food.info.flavor.clone(),
                    to_zone: ObjectiveZone::Stomach,
                });
                hand.holding = None;
                commands.entity(ent).despawn();
            } else if mousepos
                .world
                .distance_squared(pan_tr.translation.truncate())
                < 200.0 * 200.0
            {
                let (ent, mut food, tr) = food_query.get_mut(held).unwrap();
                food.state = FoodState::Cooking;
                refresh_event.send(RefreshShelfEvent { clear: false });
                eat_event.send(EatEvent {
                    from: tr.translation,
                    flavor: food.info.flavor.clone(),
                    to_zone: ObjectiveZone::Pan,
                });
                hand.holding = None;
                commands.entity(ent).despawn();
            }
        } else {
            for (ent, mut food, tr) in &mut food_query {
                if food.state == FoodState::Shelved
                    && mousepos.world.distance_squared(tr.translation.truncate()) < 100.0 * 100.0
                {
                    food.state = FoodState::Held;
                    hand.holding = Some(ent);
                }
            }
        }
    }
}

fn move_hand(
    mut food_query: Query<(&Food, &mut Transform)>,
    mousepos: Res<MousePos>,
    hand: Res<Hand>,
) {
    if let Some(held) = hand.holding {
        if let Some((_, mut tr)) = food_query.get_mut(held).ok() {
            tr.translation = Vec3::new(mousepos.world.x, mousepos.world.y, 3.0);
        }
    }
}

fn clear_hand(mut commands: Commands, mut hand: ResMut<Hand>) {
    if let Some(held) = hand.holding {
        commands.entity(held).despawn();
    }
    hand.holding = None;
}

#[derive(PartialEq, Clone)]
enum Preference {
    Like,
    Dislike,
    Resist,
    Normal,
}

impl Preference {
    fn from_f32(n: f32) -> Self {
        if n < 0.0 {
            Preference::Dislike
        } else if n > 1.0 {
            Preference::Like
        } else if n > 0.0 {
            Preference::Normal
        } else {
            Preference::Resist
        }
    }
}

#[derive(Component)]
struct PreferenceToken;

#[derive(Component)]
struct DudePreferencePoint {
    preference: Preference,
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
    atlas_handles: ResMut<AtlasHandles>,
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
