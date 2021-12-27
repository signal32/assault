use std::ops::Add;
use bevy::prelude::*;
use crate::common::Health;
use crate::player::Player;

#[derive(Component)]
struct  ScoreText;

fn interface_setup_sys(mut cmd: Commands, asset_server: Res<AssetServer>) {

    cmd.spawn_bundle(UiCameraBundle::default());

    cmd.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Merry\nChristmas!",
            TextStyle {
                font: asset_server.load("fonts/ChargeVector.otf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    }).insert(ScoreText);
}

fn score_update_sys(mut score_text: Query<&mut Text, With<ScoreText>>, player: Query<(&Player, &Health), Or<(Changed<Health>, Changed<Player>)>>) {
    for mut text in score_text.iter_mut() {
        let mut new_text = String::from("");
        for (player, health) in player.iter() {
            //new_text.push_str(&format!("Health: {}\n Score: {}", health.health, player.score));
            text.sections[0].value = format!("Health: {}\n Score: {}", health.health, player.score);
        }

        //text.sections[0].value = new_text;
    }
}

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(interface_setup_sys)
            .add_system(score_update_sys);

    }
}