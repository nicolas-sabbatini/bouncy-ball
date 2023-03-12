use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{rngs::ThreadRng, Rng};

use crate::config::{WIN_HEIGHT, WIN_WIDTH};

const BALL_RADIUS: f32 = 20.0;
const BALLS_AMOUNT: usize = 120;

#[derive(Debug, Component)]
struct Ball;

#[derive(Debug, Component)]
struct Speed {
    x: f32,
    y: f32,
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_ball);

        app.add_system(move_ball);
    }
}

fn random_color(rng: &mut ThreadRng) -> Color {
    Color::Hsla {
        hue: rng.gen_range(0.0..360.0),
        saturation: 1.0,
        lightness: 0.5,
        alpha: 1.0,
    }
}

fn init_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for i in 0..BALLS_AMOUNT {
        commands.spawn((
            Ball,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(random_color(&mut rng))),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, i as f32)),
                ..default()
            },
            Speed {
                x: (rng.gen_range(0.0..10.0) - 5.0) * 100.0,
                y: (rng.gen_range(0.0..10.0) - 5.0) * 100.0,
            },
        ));
    }
}

fn change_material(
    materials: &mut ResMut<Assets<ColorMaterial>>,
    material_handle: &Handle<ColorMaterial>,
    rng: &mut ThreadRng,
) {
    let mut material = materials.get_mut(material_handle).unwrap();
    material.color = random_color(rng);
}

fn move_ball(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Speed, &Handle<ColorMaterial>), With<Ball>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let half_width = (WIN_WIDTH - BALL_RADIUS) / 2.0;
    let half_height = (WIN_HEIGHT - BALL_RADIUS) / 2.0;

    let mut rng = rand::thread_rng();

    for (mut pos, mut speed, material_handle) in &mut query {
        pos.translation.x += speed.x * time.delta().as_secs_f32();
        pos.translation.y += speed.y * time.delta().as_secs_f32();

        if pos.translation.x < -half_width {
            pos.translation.x = -half_width;
            speed.x *= -1.0;
            change_material(&mut materials, material_handle, &mut rng);
        }
        if pos.translation.x > half_width {
            pos.translation.x = half_width;
            speed.x *= -1.0;
            change_material(&mut materials, material_handle, &mut rng);
        }
        if pos.translation.y < -half_height {
            pos.translation.y = -half_height;
            speed.y *= -1.0;
            change_material(&mut materials, material_handle, &mut rng);
        }
        if pos.translation.y > half_height {
            pos.translation.y = half_height;
            speed.y *= -1.0;
            change_material(&mut materials, material_handle, &mut rng);
        }
    }
}
