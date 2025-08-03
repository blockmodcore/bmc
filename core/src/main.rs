use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use chunk_generator;
mod tool;

/// Vsync
// const PRESENT_MODE: PresentMode = PresentMode::Fifo;
const PRESENT_MODE: PresentMode = PresentMode::AutoNoVsync;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PRESENT_MODE,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(
            Startup,
            (
                setup,
                tool::noclip_cam::setup_noclip_camera,
                tool::fps::setup_fps_counter,
            ),
        )
        .add_systems(
            Update,
            (
                tool::noclip_cam::update_noclip_camera,
                tool::fps::text_update_system,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    // commands.spawn((
    //     Mesh3d(meshes.add(Circle::new(4.0))),
    //     MeshMaterial3d(materials.add(Color::WHITE)),
    //     Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    // ));

    // my chunk
    let chunk_mesh_handle: Handle<Mesh> = meshes.add(chunk_generator::generate());
    commands.spawn((
        Mesh3d(chunk_mesh_handle),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(
            -(chunk_generator::CHUNK_SIZE_H as f32) / 2.0,
            -(chunk_generator::CHUNK_SIZE_V as f32) / 2.0,
            -(chunk_generator::CHUNK_SIZE_H as f32) / 2.0,
        ),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 8.0, 0.0),
    ));
}
