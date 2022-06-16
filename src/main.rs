use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_there)
        .add_system(greet_people)
        .run();
}

fn hello_there() {
    println!("hello there");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0)
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Blabol Blabolovic".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Prasopes Fik".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Blb Blboun".to_string()));
}
