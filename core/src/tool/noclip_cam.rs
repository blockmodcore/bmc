use bevy::{
    input::mouse::{MouseMotion},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

/// max vertical angel
const MAX_ANGLE: f32 = 90.0f32.to_radians();

const SPEED: f32 = 5.0;
const ROT_SPEED: f32 = 0.005;

#[derive(Component, Resource, Default)]
pub struct NoclipCam {
    pub rot_v: f32,
    pub rot_h: f32,
}

pub fn setup_noclip_camera(mut commands: Commands) {
    commands.insert_resource(NoclipCam {
        rot_v: 0.0,
        rot_h: 0.0,
    });
    commands.spawn((
        NoclipCam::default(),
        Camera3d::default(),
        Transform::from_xyz(8.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

use bevy::window::Window;

pub fn update_noclip_camera(
    mut camera: Single<&mut Transform, With<NoclipCam>>,
    mut camera_data: ResMut<NoclipCam>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    time: Res<Time>,
    button_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    // grab
    let mut primary_window = q_windows.single_mut().unwrap();
    if button_input.just_pressed(KeyCode::Escape) {
        if let CursorGrabMode::Locked = primary_window.cursor_options.grab_mode {
            primary_window.cursor_options.grab_mode = CursorGrabMode::None;
            primary_window.cursor_options.visible = true;
        } 
    } else if mouse_button_input.just_pressed(MouseButton::Left) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor_options.visible = false;
    }

    if let CursorGrabMode::Locked = primary_window.cursor_options.grab_mode {
        // move
        let mut direction = Vec3::ZERO;
        if button_input.pressed(KeyCode::KeyW) {
            direction += *camera.forward();
        }
        if button_input.pressed(KeyCode::KeyS) {
            direction += *camera.back();
        }
        if button_input.pressed(KeyCode::KeyA) {
            direction += *camera.left();
        }
        if button_input.pressed(KeyCode::KeyD) {
            direction += *camera.right();
        }
        if button_input.pressed(KeyCode::Space) {
            direction += *camera.up();
        }
        if button_input.pressed(KeyCode::ControlLeft) {
            direction += *camera.down();
        }
        let speed_multiplier = button_input.pressed(KeyCode::ShiftLeft).then_some(10.0).unwrap_or(1.0);
        camera.translation += direction * SPEED * time.delta_secs() * speed_multiplier;

        // rotate
        for ev in mouse_motion.read() {
            camera_data.rot_h -= ev.delta.x * ROT_SPEED;
            camera_data.rot_v -= ev.delta.y * ROT_SPEED;

            if camera_data.rot_v > MAX_ANGLE {
                camera_data.rot_v = MAX_ANGLE;
            } else if camera_data.rot_v < -MAX_ANGLE {
                camera_data.rot_v = -MAX_ANGLE;
            }
        }

        camera.rotation = Quat::IDENTITY;
        camera.rotate_x(camera_data.rot_v);
        camera.rotate_y(camera_data.rot_h);
    }
}
