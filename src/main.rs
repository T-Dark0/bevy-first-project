mod components;
mod resources;

use std::time::Duration;

use bevy::prelude::*;
use components::{Bullet, BulletSpawner, GlobalTimer};
use resources::Materials;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        //Resources
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(GlobalTimer(Timer::new(Duration::from_millis(16), true)))
        //Startup systems:
        .add_startup_system(setup_materials.system())
        .add_startup_stage("donut_spawner", SystemStage::single(spawn_spawner.system()))
        //Systems:
        .add_system(tick_global_timer.system())
        .add_system(spawn_donut.system())
        .add_system(move_donut.system())
        .run();
}

fn setup_materials(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let red_donut = asset_server.load("red_square_donut.png");
    let donut_gun = asset_server.load("donut_gun.png");
    commands.insert_resource(Materials {
        red_donut,
        donut_gun,
    });
}

fn spawn_spawner(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.donut_gun.clone(),
            ..Default::default()
        })
        .with(BulletSpawner);
}

fn move_donut(timer: Res<GlobalTimer>, mut query: Query<(&Bullet, &mut Transform)>) {
    if !timer.0.finished() {
        return;
    }

    for (_, mut transform) in query.iter_mut() {
        transform.translation.x += 1.0;
        transform.rotate(Quat::from_rotation_z(0.1));
    }
}

fn tick_global_timer(time: Res<Time>, mut timer: ResMut<GlobalTimer>) {
    timer.0.tick(time.delta_seconds());
}

fn spawn_donut(
    commands: &mut Commands,
    materials: Res<Materials>,
    timer: Res<GlobalTimer>,
    query: Query<(&BulletSpawner, &Transform)>,
) {
    if !timer.0.finished() {
        return;
    }

    for (_, transform) in query.iter() {
        commands
            .spawn(SpriteBundle {
                material: materials.red_donut.clone(),
                transform: *transform,
                ..Default::default()
            })
            .with(Bullet);
    }
}
