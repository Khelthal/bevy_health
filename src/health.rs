use bevy::prelude::*;
use std::time::Duration;

//constants

const BAR_HEIGHT: f32 = 5.;
const BAR_WIDTH: f32 = 50.;

//enums

//structs

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(activate_bars)
            .add_system(countdown_bars)
            .add_system(update_bars);
    }
}

#[derive(Component)]
pub struct Health {
    pub max_val: f32,
    pub cur_val: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            max_val: health,
            cur_val: health,
        }
    }
}

#[derive(Component)]
pub struct HealthBar {
    pub offset: Vec2,
}

#[derive(Component)]
struct DisplayHealthBar {
    time_left: Timer,
}

//systems

fn activate_bars(
    mut commands: Commands,
    changed_health: Query<(Entity, Option<&Children>, &HealthBar, &Health), Changed<Health>>,
    mut displaying_health_bars: Query<&mut DisplayHealthBar>,
) {
    for (entity, children, health_bar, health) in changed_health.iter() {
        let mut bar_child: Option<Entity> = None;

        if let Some(children) = children {
            for child in children.into_iter() {
                if let Ok(_) = displaying_health_bars.get_mut(*child) {
                    bar_child = Some(*child);
                }
            }
        }

        if let Some(entity) = bar_child {
            let mut displaying_health_bar = displaying_health_bars.get_mut(entity).unwrap();
            displaying_health_bar.time_left.reset();
        } else {
            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(
                                (health.cur_val / health.max_val) * BAR_WIDTH,
                                BAR_HEIGHT,
                            )),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(health_bar.offset.extend(0.)),
                        ..Default::default()
                    })
                    .insert(DisplayHealthBar {
                        time_left: Timer::new(Duration::from_millis(10000), false),
                    });
            });
        }
    }
}

fn countdown_bars(
    mut commands: Commands,
    mut bars: Query<(Entity, &mut DisplayHealthBar)>,
    time: Res<Time>,
) {
    for (bar_entity, mut bar) in bars.iter_mut() {
        if bar.time_left.tick(time.delta()).just_finished() {
            commands.entity(bar_entity).despawn_recursive();
        }
    }
}

fn update_bars(
    changed_bars: Query<(&Health, &Children), (Changed<Health>, With<HealthBar>)>,
    mut spawned_bars: Query<&mut Sprite, With<DisplayHealthBar>>,
) {
    for (health, children) in changed_bars.iter() {
        for child in children.iter() {
            if let Ok(mut health_bar) = spawned_bars.get_mut(*child) {
                health_bar.custom_size = Some(Vec2::new(
                    (health.cur_val / health.max_val) * BAR_WIDTH,
                    BAR_HEIGHT,
                ))
            }
        }
    }
}
