mod camera;
mod cursor;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use camera::spawn_main_camera;
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 400.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

pub fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
		.init_resource::<cursor::Cursor>()
		// .add_systems(Startup, cursor::setup)
		.add_systems(Startup, spawn_main_camera)
		.add_systems(Startup, spawn_player)
		.add_systems(Update, cursor::system)
		// .add_systems(Startup, spawn_enemies)
		.add_systems(Update, player_movement)
		.add_systems(Update, player_rotation)
		.add_systems(Update, confine_player_movement)
		.run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_player(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
) {
	let window: &Window = window_query.get_single().unwrap();

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
				texture: asset_server.load("player_icon.png"),
				..default()
			},
			Player {}
		)
	);
}

pub fn spawn_enemies(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
) {
	let window: &Window = window_query.get_single().unwrap();

	for _ in 0..NUMBER_OF_ENEMIES {
		let random_x: f32 = random::<f32>() * window.width();
		let random_y: f32 = random::<f32>() * window.height();

		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(random_x, random_y, 0.0),
					texture: asset_server.load("enemy_icon.png"),
					..default()
				},
				Enemy {},
			)
		);
	}
}

pub fn spawn_camera(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
) {
	let window: &Window = window_query.get_single().unwrap();

	commands.spawn(
		Camera2dBundle{
			transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
			..default()
		}
	);
}

pub fn player_movement(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut player_query: Query<&mut Transform, With<Player>>,
	time: Res<Time>,
) {
	if let Ok(mut transform) = player_query.get_single_mut() {
		let mut direction = Vec3::ZERO;

		if keyboard_input.pressed(KeyCode::KeyW) {
			direction += Vec3::new(0.0, 1.0, 0.0);
		}
		if keyboard_input.pressed(KeyCode::KeyA) {
			direction += Vec3::new(-1.0, 0.0, 0.0);
		}
		if keyboard_input.pressed(KeyCode::KeyS) {
			direction += Vec3::new(0.0, -1.0, 0.0);
		}
		if keyboard_input.pressed(KeyCode::KeyD) {
			direction += Vec3::new(1.0, 0.0, 0.0);
		}

		if direction.length() > 0.0 {
			direction = direction.normalize();
		}

		transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
	}
}

pub fn player_rotation(
	mut player_query: Query<&mut Transform, With<Player>>,
	cursor: Res<cursor::Cursor>,
) {
	let mut player_transform = player_query.get_single_mut().unwrap();
	let player_pos = player_transform.translation.xy();

	let direction = cursor.pos - player_pos;
	let angle = direction.y.atan2(direction.x);

	player_transform.rotation = Quat::from_rotation_z(angle);
}

pub fn confine_player_movement(
	mut player_query: Query<&mut Transform, With<Player>>,
	window_query: Query<&Window, With<PrimaryWindow>>,
) {
	if let Ok(mut player_transform) = player_query.get_single_mut() {
		let window = window_query.get_single().unwrap();

		let half_player_size: f32 = PLAYER_SIZE / 2.0;
		let x_min: f32 = 0.0 + half_player_size;
		let y_min: f32 = 0.0 + half_player_size;
		let x_max: f32 = window.width() - half_player_size;
		let y_max: f32 = window.height() - half_player_size;

		let mut translation = player_transform.translation;

		if translation.x < x_min {
			translation.x = x_min;
		} else if translation.x > x_max {
			translation.x = x_max;
		}

		if translation.y < y_min {
			translation.y = y_min;
		} else if translation.y > y_max {
			translation.y = y_max;
		}

		player_transform.translation = translation;
	}
}
