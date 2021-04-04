use bevy::{
    prelude::*,
    render::pass::ClearColor};

const VENTANA_TAM: [f32;2] = [800.0, 600.0];

const PELOTA_INI_POS: [f32;3] = [0.0, 0.0, 0.0];
const PELOTA_TAM: f32 = 20.0;

struct Pelota {
    velocity: Vec3,
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
    
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(PELOTA_INI_POS[0], PELOTA_INI_POS[1], PELOTA_INI_POS[2])),
            sprite: Sprite {
                size: Vec2::new(PELOTA_TAM, PELOTA_TAM),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Pelota {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}

fn ball_movement_system(time: Res<Time>, mut query: Query<(&Pelota, &mut Transform)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (ball, mut transform) in query.iter_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::BLACK.into()))
        .add_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: VENTANA_TAM[0],
            height: VENTANA_TAM[1],
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_movement_system.system())
        .run();
}