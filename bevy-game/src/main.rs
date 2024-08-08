use bevy::prelude::*;

#[derive(Component)]
struct Xp(u32);

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

fn level_up(
    mut query: Query<(&mut Xp, &mut Health)>,
) {
    for (mut xp, mut health) in query.iter_mut() {
        if xp.0 > 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
        }
    }
    println!("Hello from level up!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, level_up)
        .run();
}
