use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use rand::prelude::*;
use crate::common::{Direction, *};
use crate::projectile::*;
use crate::player::*;

const WINDOW_MARGIN: f32             = 100.;
const DEFAULT_MOVE_SPEED: f32        = 150.;
const DEFAULT_PROJECTILE_DAMAGE: i32 = 10;
const ENEMY_VERT_SPACING: f32        = 50.;

#[derive(Component)]
pub struct Enemy {
    move_direction: Direction,
    move_speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            move_direction: Direction::RIGHT,
            move_speed: DEFAULT_MOVE_SPEED,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub shooter: Shooter,
    pub collision_box: CollisionBox,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Default::default(),
            health: Default::default(),
            shooter: Default::default(),
            sprite: Default::default(),
            collision_box: CollisionBox{ size: Vec2::new(15., 15.)}
        }
    }
}

/// System to move [`Enemy`] horizontally over screen and swap their direction when they reach each side.
pub fn enemy_move_sys(mut enemy_transforms: Query<(&mut Enemy, &mut Transform), With<Enemy>>, time: Res<Time>, windows: Res<Windows>) {
    match windows.get_primary() {
        Some(window) => {
            for (mut enemy, mut transform) in enemy_transforms.iter_mut() {

                // Enemy direction should change when they are at left or right side of screen
                if transform.translation.x > (window.width() / 2.0) - WINDOW_MARGIN {
                    enemy.move_direction = Direction::LEFT
                } else if  transform.translation.x < (-window.width() / 2.0) + WINDOW_MARGIN {
                    enemy.move_direction = Direction::RIGHT
                }

                match &enemy.move_direction {
                    Direction::LEFT  => { transform.translation.x = transform.translation.x - enemy.move_speed * time.delta_seconds() }
                    Direction::RIGHT => { transform.translation.x = transform.translation.x + enemy.move_speed * time.delta_seconds() }
                    _ => {}
                }
            }
        }
        None => { panic!("Primary display required but was not found.") }
    }
}

/// Enemies shoot straight down by random choice and interval
pub fn enemy_shoot_sys(mut cmd: Commands, mut enemy_shooter: Query<(Entity, &mut Shooter, &Transform), With<Enemy>>) {
    let other_enemies: Vec<Entity> = enemy_shooter.iter().map(|(e, _, _)| { e}).collect();
    for (entity, shooter, transform) in enemy_shooter.iter_mut() {
        if shooter.fire_rate > rand::random::<f32>() {
            cmd.spawn_bundle(ProjectileBundle {
                projectile: Projectile {
                    direction: Direction::DOWN,
                    damage: DEFAULT_PROJECTILE_DAMAGE,
                    speed_multiplier: Default::default(),
                    origin: Some(entity.clone()),
                    whitelist: other_enemies.clone(),
                },
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::CRIMSON,
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(transform.translation.x, transform.translation.y, transform.translation.z),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

/// Reduces enemy health and increases player score when hit
fn enemy_hit_sys(
    mut hit_events: EventReader<CollisionEvent>,
    mut players: Query<&mut Player>,
    mut enemy_healths: Query<&mut Health, With<Enemy>>,
    projectiles: Query<&Projectile>) {
    for &CollisionEvent{a, b, ..} in hit_events.iter() {
        match enemy_healths.get_mut(b) {
            Ok(mut enemy_health) => {
                match projectiles.get(a) {
                    Ok(projectile) => {
                        match players.get_mut(projectile.origin.unwrap()){
                            Ok(mut player) => {
                                player.score += 10;//todo calculate enemy destroy score from enemy stats
                                enemy_health.health = enemy_health.health - projectile.damage;
                            }
                            Err(_) => {/*warn!("Enemy just shot itself.")*/}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}

pub fn enemy_remove_sys(mut cmd: Commands, enemies: Query<(Entity, &Health), With<Enemy>>) {
    for (entity, health) in enemies.iter() {
        if health.health <= 0 {
            info!("Removing enemy Entity {}: Health is {}", entity.id(), health.health);
            cmd.entity(entity).despawn();
        }
    }
}


/// Creates [`Enemy`] entities at random positions on top half of screen.
pub fn enemy_startup_sys(mut cmd: Commands, state: Res<EnemyPlugin>, windows: Res<Windows>) {
    let width = windows.get_primary().unwrap().width() / 2.0;
    let height = windows.get_primary().unwrap().height() / 2.0;
    let mut vertical_pos = height - ENEMY_VERT_SPACING;
    let mut rng = rand::thread_rng();

    for _ in 0..state.enemy_count {
        let horizontal_pos = rng.gen_range(-width..width );

        cmd.spawn_bundle(EnemyBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::FUCHSIA,
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(horizontal_pos, vertical_pos, 0.0),
                ..Default::default()
            },
            shooter: Shooter { ammo_count: 10, fire_rate: state.fire_rate },
            ..Default::default()
        });
        vertical_pos -= ENEMY_VERT_SPACING;
    }
}

pub struct EnemyPlugin {
    pub enemy_count: i32,
    pub fire_rate: f32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {

        let x = Self {
            enemy_count: self.enemy_count,
            fire_rate: self.fire_rate,
        };

        app .insert_resource(x)
            .add_startup_system(enemy_startup_sys)
            .add_system(enemy_move_sys)
            .add_system(enemy_shoot_sys)
            .add_system(enemy_remove_sys)
            .add_system(enemy_hit_sys);
    }
}