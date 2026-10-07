use serde::{Deserialize, Serialize};
// use serde_json::{Result, json};
// use std::{env, fs};

use std::error::Error;
use std::fmt;
use vector2d::Vector2D;
// use vector2d::Vector2D;

use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

// https://stackoverflow.com/questions/51550167/how-to-manually-return-a-result-boxdyn-error

/// The state for the basic player, where are they currently at.
#[derive(Debug)]
pub enum PlayerState {
    MainMenu,
    Playing,
    Paused,
    Dead,
}

/// Player struct
/// Very basic player for testing structs
pub struct Player {
    pub name: String,
    // pub position: Vector2D::new(0, 0.0);
    pub health: i32,
    pub armor: i32,
    pub hunger: i32,
    pub status: PlayerState,
    // pub ipv4_address: String,
    // pub ipv6_address: String,
    pub position: Vector2D<f32>,
    // pub camera_position: Vector2D<f32>,
    pub heading: f32,
}
//

#[derive(Serialize, Deserialize)]
/// Location of the map for the JSON location list.
pub struct MapLocation {
    // position: Vector2D<f32>,
    name: String,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    heading: f32,
}

#[derive(Debug)]
struct JsonError(String);

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for JsonError {}

/// This is a stub function, it doesn't do anything just yet.
pub fn read_json_file(path: &str) {
    println!("Reading json file: {}", path);
}

/// Print a test location with JSON.
/// Now this prints from a JSON file, I need to figure out how to parse it.
/// And write back into this.
/// I may build a location reader/writer in Rust for my ReVC Lua scripts.
/// Moved out of misc-test
/// `json_path` Path to the JSON file.
/// `location_name` The location name to search for in the JSON.
pub fn print_location(json_path: &str, location_name: &str) -> Result<(), Box<dyn Error>> {
    // https://stackoverflow.com/questions/63657897/how-to-read-json-file-with-serde
    let data = fs::read_to_string(json_path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse json");

    // Serialize it to a JSON string
    // let j = serde_json::to_string_pretty(&res)?;
    // let j = serde_json::to_string(&res)?;

    // https://jsonic.io/guides/parse-json-rust

    // let locations: Vec<MapLocation> = serde_json::from_str(&j)?;
    // let locations: Vec<MapLocation> = serde_json::from_str(&j)?;
    // for location in &locations {
    //     println!("Name: {} X: {} Y: {} Z: {}\n Heading: {}", location.name, location.pos_x, location.pos_y, location.pos_z, location.heading);
    // }

    // This works for reading the location values like this.
    // println!("{}", res["location1"]);

    if res[location_name].is_null() {
        // println!("Error reading location {}", location_name);
        return Err(Box::new(JsonError(
            "Location was null, or contained no data".into(),
        )));
    }

    println!("Location: {}", res[location_name]);

    // }

    // println!("{}", j);

    Ok(())
}

// Test for writing to a JSON file.
// https://en.perfcode.com/rust/serde/process-files
// fn write_location(json_path: &str) -> Result<()> {
pub fn write_locations(json_path: &str) {
    // let data = fs::read_to_string(json_path).expect("Unable to read file");
    // let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse json");

    let location1 = MapLocation {
        name: "Location1".to_string(),
        pos_x: 180.0,
        pos_y: 180.0,
        pos_z: 14.0,
        heading: 90.0,
    };

    let location2 = MapLocation {
        name: "Location2".to_string(),
        pos_x: 140.2,
        pos_y: 150.3,
        pos_z: 14.5,
        heading: 40.0,
    };

    // https://devtoolsbuilder.com/learn/json/rust
    // This is one way to format the JSON data.
    // I figured this out! I looked at the JSON variable below, and it showed how I can do this.
    let location_json = json!({
        "Location1":  {
            "name": location1.name,
            "pos_x": location1.pos_x,
            "pos_y": location1.pos_y,
            "pos_z": location1.pos_z,
        },

        // https://stackoverflow.com/questions/28655362/how-does-one-round-a-floating-point-number-to-a-specified-number-of-digits#28656825
        // https://doc.rust-lang.org/std/fmt/

        "Location2":  {
            "name": location2.name,
            // Well now these get converted to a string in the JSON output.
            // This did something but now they don't have the values stripped from it anymore.
            "pos_x": format!("{:.3}", location2.pos_x).parse::<f32>().unwrap(),
            "pos_y": format!("{:.2}", location2.pos_y).parse::<f32>().unwrap(),
            "pos_z": format!("{:.2}", location2.pos_z).parse::<f32>().unwrap(),
        }
    });


    let file = File::create(json_path).unwrap();
    let writer = BufWriter::new(file);

    // serde_json::to_writer_pretty(writer, &location1).unwrap();
    serde_json::to_writer_pretty(writer, &location_json).unwrap();

    // Ok(())
}
