use bevy::prelude::*;
use crate::common::{Health, Shooter, Direction};
use crate::{CollisionBox, CollisionEvent, Enemy};
use crate::projectile::{Projectile, ProjectileBundle};

const MOVE_SPEED: f32 = 600.;
const PLAYER_VERT_OFFSET: f32 = 200.;

#[derive(Component)]
pub struct Player {
    pub score: i32
}

impl Default for Player {
    fn default() -> Self {
        Self {
            score: 0,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub shooter: Shooter,
    pub collision_box: CollisionBox,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player {
                score: 10
            },
            health: Default::default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0,-PLAYER_VERT_OFFSET,0.0),
                ..Default::default()
            },
            shooter: Default::default(),
            collision_box: CollisionBox { size: Vec2::new(50.0, 50.0)},
        }
    }
}

fn player_move_sys(mut player_transforms: Query<&mut Transform, With<Player>>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>) {
    for mut transform in player_transforms.iter_mut() {

        let boost =
            if keyboard_input.pressed(KeyCode::LControl) && keyboard_input.pressed(KeyCode::LShift) {0.5}
            else if keyboard_input.pressed(KeyCode::LShift) { 2.0 }
            else { 1.0 };

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x = transform.translation.x - MOVE_SPEED * boost * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x = transform.translation.x + MOVE_SPEED * boost * time.delta_seconds();
        }
    }
}

fn player_shoot_sys(mut player_shooter: Query<(Entity, &Transform), With<Player>>, keyboard_input: Res<Input<KeyCode>>, mut cmd: Commands) {
    for (entity, transform) in player_shooter.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            info!("Player entity={} shooting", &entity.id());
            cmd.spawn_bundle(ProjectileBundle {
                projectile: Projectile {
                    direction: Direction::UP,
                    damage: 30,
                    speed_multiplier: Default::default(),
                    origin: Some(entity.clone()),
                    whitelist: Vec::new(),
                },
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(transform.translation.x, transform.translation.y, transform.translation.z),
                    ..Default::default()
                },
                ..Default::default()
            });
        };

    }
}

fn player_score_sys(mut hit_events: EventReader<CollisionEvent>, enemies: Query<&Enemy>, projectiles: Query<&Projectile>, mut players: Query<&mut Player>) {
    for &CollisionEvent{a, b, .. } in hit_events.iter() {
        match enemies.get_component::<Enemy>(b) { // hits an enemy
            Ok(_) => {
                match projectiles.get_component::<Projectile>(a) { // hit by projectile
                    Ok(projectile) => {
                        match players.get_mut(projectile.origin.unwrap()) { // projectiles origin entity
                            Ok(mut player) => { player.score = player.score + 10 }
                            Err(_) => { warn!("Could not apply player score: No player as origin for projectile")}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}

/// Reduces players health when hit by [`Projectile`].
fn player_hit_sys(
    mut hit_events: EventReader<CollisionEvent>,
    mut player_healths: Query<&mut Health, With<Player>>,
    projectiles: Query<&Projectile>) {
    for &CollisionEvent{a, b, ..} in hit_events.iter(){
        match player_healths.get_mut(b) { // Check is player w/ health
            Ok(mut player_health) => {
                match projectiles.get(a) {
                    Ok(projectile) => {
                        player_health.health = player_health.health - projectile.damage;
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}

fn player_startup_sys(mut cmd: Commands) {
    new_player(&mut cmd);
}

pub fn new_player(cmd: &mut Commands) {
    cmd.spawn_bundle(PlayerBundle::default());
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_startup_system(player_startup_sys)
            .add_system(player_move_sys)
            .add_system(player_shoot_sys)
            //.add_system(player_score_sys)
            .add_system(player_hit_sys);
    }
}