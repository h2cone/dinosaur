use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};
use bevy_rapier2d::prelude::*;

const WIN_WIDTH: f32 = 640.;
const WIN_HEIGHT: f32 = 400.;

const FLOOR_HEIGHT: f32 = WIN_HEIGHT * 0.3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dinosaur".to_string(),
                resolution: WindowResolution::new(WIN_WIDTH, WIN_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, camera_follow_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, player_jump)
        .add_systems(Update, jump_reset)
        .run();
}

#[derive(Component)]
struct Player {
    width: f32,
    height: f32,
    speed: f32,
}

#[derive(Component)]
struct Jumper {
    speed: f32,
    jumping: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // Floor
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            custom_size: Some(Vec2::new(WIN_WIDTH * 2., FLOOR_HEIGHT)),
            ..default()
        },
        ..default()
    })
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from(Transform::from_translation(Vec3::new(0., (FLOOR_HEIGHT - WIN_HEIGHT) / 2., 0.))))
        .insert(Collider::cuboid((WIN_WIDTH / 2.) * 2., FLOOR_HEIGHT / 2.))
        .insert(Friction {
            coefficient: 5.,
            ..default()
        });
    // Player
    let player = Player {
        speed: 100.,
        width: 50.,
        height: 50.,
    };
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(player.width, player.height)),
            ..default()
        },
        ..default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Transform::from_translation(Vec3::new(0., (player.height - WIN_HEIGHT) / 2. + FLOOR_HEIGHT, 0.)))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(10.))
        .insert(Ccd::enabled())
        .insert(Collider::round_cuboid(player.width / 2., player.height / 2., 0.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(player)
        .insert(Jumper {
            speed: 150.,
            jumping: false,
        });
}

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player), With<Player>>,
) {
    for (mut vel, player) in query.iter_mut() {
        if input.pressed(KeyCode::ArrowLeft) {
            vel.linvel = Vect::new(-player.speed, vel.linvel.y);
        }
        if input.pressed(KeyCode::ArrowRight) {
            vel.linvel = Vect::new(player.speed, vel.linvel.y);
        }
    }
}

fn player_jump(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Jumper), With<Player>>,
) {
    for (mut vel, mut jumper) in query.iter_mut() {
        if input.pressed(KeyCode::ArrowUp) && !jumper.jumping {
            vel.linvel = Vect::new(vel.linvel.x, jumper.speed);
            jumper.jumping = true;
        }
    }
}

fn jump_reset(
    mut events: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut Jumper), With<Player>>,
) {
    for event in events.read() {
        for (entity, mut jumper) in query.iter_mut() {
            if let CollisionEvent::Started(entity1, entity2, _flag) = event {
                if entity1 == &entity || entity2 == &entity {
                    jumper.jumping = false;
                }
            }
        }
    }
}

fn camera_follow_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.translation.x;
        }
    }
}