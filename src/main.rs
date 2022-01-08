use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(initialize)
        .add_system(greet_people)
        .run();
}

fn initialize(mut commands: Commands) {
    commands.spawn().insert(Player).insert(Resource("Copper".to_string()));
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Resource(String);

fn greet_people(query: Query<&Resource, With<Player>>) {
    for name in query.iter() {
        println!("Resource: {}!", name.0);
    }
}
