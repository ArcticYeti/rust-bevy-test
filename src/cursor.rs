use crate::camera;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource, Default)]
pub struct Cursor {
	pub pos: Vec2,
}

pub fn system(
	mut cursor_res: ResMut<Cursor>,
	q_window: Query<&Window, With<PrimaryWindow>>,
	q_camera: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
) {
	let (camera, camera_transform) = q_camera.single();
	let window = q_window.single();

	if let Some(world_position) = window.cursor_position()
		.and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
		.map(|ray| ray.origin.truncate())
	{
		cursor_res.pos = world_position;
	}
}
