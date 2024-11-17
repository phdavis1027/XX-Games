use bevy::{prelude::{Bundle, Component, Entity, Rectangle, Transform}, sprite::{ColorMaterial, MaterialMesh2dBundle}};

#[derive(Component)]
pub struct BallMarkerComponent;

#[derive(Bundle)]
pub struct Ball {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub _marker: BallMarkerComponent
}


#[derive(Component)]
pub struct PaddleMarkerComponent;

#[derive(Component)]
pub struct ScoreComponent(pub i32);

#[derive(Bundle)]
pub struct Paddle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub score: ScoreComponent,
    pub _marker: PaddleMarkerComponent
}

#[derive(Component)]
pub struct ScoreBoxMarkerComponent;

#[derive(Component)]
pub struct ScoreBoxPlayerComponent(pub Entity);

#[derive(Component)]
pub struct HitBoxComponent(pub Rectangle);

#[derive(Bundle)]
pub struct ScoreBoxColliderBundle {
    pub transform: Transform,
    pub rect: HitBoxComponent,
}

#[derive(Bundle)]
pub struct ScoreBox {
    pub player: ScoreBoxPlayerComponent,
    pub collider: ScoreBoxColliderBundle,
    pub _marker: ScoreBoxMarkerComponent
}
