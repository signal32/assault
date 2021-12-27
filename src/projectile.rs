use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::utils::HashMap;
use crate::common::{Direction, *};

#[derive(Component)]
pub struct Projectile {
    pub direction: Direction,
    pub damage: i32,
    pub speed_multiplier: f32,
    pub origin: Option<Entity>,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            direction: Direction::DOWN,
            damage: 10,
            speed_multiplier: 1.0,
            origin: Option::None,
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub collision_box: CollisionBox,

    #[bundle]
    pub sprite: SpriteBundle,
}

fn projectile_move_sys(mut projectile_transforms: Query<(&mut Projectile, &mut Transform)>) {
    for (mut projectile, mut transforms) in projectile_transforms.iter_mut() {
        match &projectile.direction {
            Direction::UP    => { transforms.translation.y = transforms.translation.y + 10.0 }
            Direction::DOWN  => { transforms.translation.y = transforms.translation.y - 10.0 }
            Direction::LEFT  => { transforms.translation.x = transforms.translation.x + 10.0 }
            Direction::RIGHT => { transforms.translation.x = transforms.translation.x - 10.0 }
        }
    }

}

fn projectile_remove_sys(mut cmd: Commands, mut projectile_entities: Query<(Entity, &Transform), With<Projectile>>, windows: Res<Windows>){
    let bounds_top =  windows.get_primary().unwrap().height();
    let bounds_right =  windows.get_primary().unwrap().width();

    for (entity, transform) in projectile_entities.iter_mut(){
        let mut remove = false;
        if transform.translation.x >  bounds_top   { remove = true; }
        if transform.translation.x < -bounds_top   { remove = true; }
        if transform.translation.y >  bounds_right { remove = true; }
        if transform.translation.y < -bounds_right { remove = true; }

        if remove {
            debug!("Despawning projectile entity {} at {}", entity.id(), transform.translation);
            cmd.entity(entity).despawn();
        }
    }
}

pub struct ProjectileHitEvent {
    pub projectile: Entity,
    pub other: Entity,
}

fn projectile_hit_sys(
    mut cmd: Commands,
    mut projectiles: Query<(Entity, &Projectile, &Transform)>,
    mut health_transforms: Query<(Entity, &mut Health, &Transform)>,
    mut hit_event_writer: EventWriter<ProjectileHitEvent>) {
    for (entity, projectile, transform) in projectiles.iter() {
        for (hit_entity, mut hit_health, hit_transform) in health_transforms.iter_mut() {

            // This prevents a projectile hitting it's origin entity
            match projectile.origin {
                Some(origin) => if origin.eq(&hit_entity){ continue }
                None => {}
            }

            if transform.translation.distance(hit_transform.translation) <= 25. { //todo remove need for constant
                hit_health.health = hit_health.health - projectile.damage;
                hit_event_writer.send(ProjectileHitEvent { projectile: entity, other: hit_entity});
                info!("Removing projectile entity {}: hit entity {} (health: {}, damage_dealt: {})", &entity.id(), &hit_entity.id(), hit_health.health, projectile.damage );
                cmd.entity(entity).despawn();
            }
        }
    }
}



impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            projectile: Default::default(),
            collision_box: CollisionBox{size: Vec2::new(10.0, 10.0)},
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::LIME_GREEN,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: Default::default(),
                global_transform: Default::default(),
                texture: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default()
            }
        }
    }
}

//--- Plugin
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app .add_system(projectile_move_sys)
            .add_system(projectile_remove_sys)
            .add_system(projectile_hit_sys)
            .add_event::<ProjectileHitEvent>();
    }
}