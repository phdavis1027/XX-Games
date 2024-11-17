use bevy::{prelude::{Bundle, Component}, sprite::{ColorMaterial, MaterialMesh2dBundle}};

pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct WallMarkerComponent(pub Side);

#[derive(Bundle)]
pub struct Wall {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub _marker: WallMarkerComponent,
}
