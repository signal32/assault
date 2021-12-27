use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;
use bevy::utils::HashMap;

const DEFAULT_INIT_HEALTH: i32 = 100;
const DEFAULT_AMMO_COUNT: i32 = 10;
const DEFAULT_FIRE_RATE: f32 = 0.01;

/* Health Component */

#[derive(Component)]
pub struct Health {
    pub health: i32
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health: DEFAULT_INIT_HEALTH,
        }
    }
}

pub fn health_event_sys(mut cmd: Commands, health_entities: Query<(Entity, &Health)>) {
    for (entity, health) in health_entities.iter(){
        if health.health <= 0 {
            info!("De-spawning Entity {}: Health is {}", entity.id(), health.health);
            cmd.entity(entity).despawn();
        }
    }
}

/* Shooter (turret) Component */

#[derive(Component)]
pub struct Shooter {
    pub ammo_count: i32,
    pub fire_rate: f32,
}

impl Default for Shooter {
    fn default() -> Self {
        Self {
            ammo_count: DEFAULT_AMMO_COUNT,
            fire_rate: DEFAULT_FIRE_RATE,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn opposite(direction: &Self) -> Self{
        match direction {
            Direction::UP    => { Direction::DOWN }
            Direction::DOWN  => { Direction::UP }
            Direction::LEFT  => { Direction::RIGHT }
            Direction::RIGHT => { Direction::LEFT }
        }
    }
}

/* Collision box Component */

#[derive(Component)]
pub struct CollisionBox {
    pub size: Vec2
}

pub struct CollisionEvent {
    pub a: Entity,
    pub b: Entity,
    pub collision: Collision,
}

fn collision_sys(mut colliders: Query<(Entity, &Transform, &CollisionBox)>, mut hit_event_writer: EventWriter<CollisionEvent>) {
    let mut hits: HashMap<Entity, Vec<Entity>> = HashMap::default(); // Tracks which entities have already collided to avoid duplicate/ghost collisions
    for (entity, transform, shape) in colliders.iter() {
        for (hit_entity, hit_transform, hit_shape) in colliders.iter(){
            let mut collisions = hits.entry(hit_entity).or_insert(Vec::new()); // Entities which `hit_entity` has already collided with this frame
            if entity == hit_entity || collisions.contains(&entity) { continue };

            match collide(transform.translation, shape.size, hit_transform.translation, hit_shape.size) {
                Some(collision) => {
                    collisions.push(hit_entity);
                    hit_event_writer.send(CollisionEvent {
                        a: entity,
                        b: hit_entity,
                        collision,
                    });
                }
                None => {}
            }
        }
    }
}

fn collision_debug_sys(mut collision_events: EventReader<CollisionEvent>) {
    for CollisionEvent { a, b, collision } in collision_events.iter() {
        let side = match collision {
            Collision::Left => {"Left"}
            Collision::Right => {"Right"}
            Collision::Top => {"Top"}
            Collision::Bottom => {"Bottom"}
        };
        info!("Collision between {} -> {}. Side: {}", a.id(), b.id(), side);
    }
}


pub struct GameCommonPlugin;

impl Plugin for GameCommonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system(health_event_sys)
            .add_system(collision_sys)
            .add_system(collision_debug_sys)
            .add_event::<CollisionEvent>();
    }
}