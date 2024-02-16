use bevy::prelude::*;

use serde::Deserialize;

use rand::Rng;

use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Resource, Deserialize, Debug)]
struct ColorMap {
    colors: HashMap<String, String>,
}

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {

    let exe_dir = env::current_exe().expect("Failed to get executable directory");
    let exe_dir = exe_dir.parent().expect("Failed to get parent directory");
    println!("{}", exe_dir.display());
    env::set_current_dir(exe_dir).expect("Failed to set current directory");

    println!("{}", std::env::current_dir().unwrap().display());

    let mut file = File::open("colors.json").expect("Failed to open JSON file");
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).expect("Failed to read JSON file");

    let color_map: ColorMap = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .insert_resource(color_map)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input_system, change_clear_color,update_position, print_position))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>) {
    // Log the entity ID and position of each entity with a `Position` component.
    for (entity, position) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, position);
    }
}

fn change_clear_color(
    input: Res<Input<KeyCode>>,
    mut clear_color: ResMut<ClearColor>,
    color_map: Res<ColorMap>,  // Access the ColorMap resource
) {
    if input.just_pressed(KeyCode::Space) {
        let random_color_index = rand::thread_rng().gen_range(0..color_map.colors.len());
        let random_color_name = color_map.colors.keys().nth(random_color_index).unwrap();
        let random_color_value = color_map.colors.get(random_color_name).unwrap();

        // Parse the color value into a Color
        if let Ok(color) = parse_color_from_string(random_color_value) {
            clear_color.0 = color;
        }
    }
}

// Function to parse a color from the string representation
fn parse_color_from_string(color_str: &str) -> Result<Color, &str> {
    if let Some(start) = color_str.find('(') {
        if let Some(end) = color_str.find(')') {
            let values_str = &color_str[start + 1..end];
            let values: Vec<&str> = values_str.split(',').collect();
            if values.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    values[0].trim().parse(),
                    values[1].trim().parse(),
                    values[2].trim().parse(),
                ) {
                    return Ok(Color::rgb(r, g, b));
                }
            }
        }
    }
    Err("Failed to parse color")
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}