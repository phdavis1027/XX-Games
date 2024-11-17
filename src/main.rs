use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    color::{
        palettes::css::WHITE,
        Color,
    },
    math::Vec2,
    prelude::{
        Camera2dBundle, Commands, EventReader, Mesh, Query, Rectangle, ResMut, Transform, With, 
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    utils::default,
    window::{PrimaryWindow, Window, WindowResized},
    DefaultPlugins,
};
use wall::{Side, Wall, WallMarkerComponent};

mod paddle;
mod wall;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, recenter_walls)
        .run();
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Vec2 {
        x: width,
        y: height,
    } = windows.single().size();

    commands.spawn(Camera2dBundle::default());
    commands.spawn(Wall {
        mesh: MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Rectangle::new(
                    20.,
                    height as f32 / 3.,
                )))
                .into(),
            material: materials.add(Color::from(WHITE)).into(),
            transform: Transform::from_xyz(width * 1. / 3., height / 8., 0.0),
            ..default()
        },
        _marker: WallMarkerComponent(Side::Left),
    });
    commands.spawn(Wall {
        mesh: MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Rectangle::new(
                    20.,
                    height as f32 / 3.,
                )))
                .into(),
            material: materials.add(Color::from(WHITE)).into(),
            transform: Transform::from_xyz(width * 2. / 3., height / 8., 0.0),
            ..default()
        },
        _marker: WallMarkerComponent(Side::Right),
    });
}

fn recenter_walls(
 mut window_resize_reader: EventReader<WindowResized>,
 mut wall_transform_query: Query<(&mut Transform, &WallMarkerComponent)>,
) {
    for event in window_resize_reader.read() {
        for (mut transform, wall_marker) in wall_transform_query.iter_mut() {
            match wall_marker.0 {
                Side::Left => {
                    transform.translation.x = event.width as f32 / 3.;
                }
                Side::Right => {
                    transform.translation.x = event.width as f32 * 2. / 3.;
                }
            }
        }
    }
}
