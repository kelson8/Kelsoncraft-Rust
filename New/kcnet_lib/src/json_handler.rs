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
use std::io::{BufWriter, Write};

// Vector2D and Vector3D
// https://www.reddit.com/r/rust/comments/1k4pj9s/made_a_library_with_common_3d_operations_that_is/
// https://crates.io/crates/euclidean

// use crate::segment_segment_intersection;

use ::core::f32::consts::PI;

// TODO Move these types elsewhere later.
pub type Vec2D = nalgebra::Vector2<f32>;
pub type Vec3D = nalgebra::Vector3<f32>;


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
///
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

// Well this breaks the Vec3D setup, I wonder why.
// TODO Is this needed for anything?
// #[derive(Serialize, Deserialize)]
/// Location of the map for the JSON location list.
pub struct MapLocation {
    // position: Vector2D<f32>,
    pub name: String,
    // pub pos_x: f32,
    // pub pos_y: f32,
    // pub pos_z: f32,
    pub pos: Vec3D,
    pub heading: f32,
}

// TODO Figure out how to use this for getters and setters later.
// https://www.slingacademy.com/article/encapsulation-in-rust-getter-and-setter-methods-for-struct-fields/
// impl MapLocation {
//     pub fn name(&self) -> String {
//         self.name
//     }
//
//     pub fn pos(&self) -> Vec3D {
//         self.pos
//     }
//
//     pub fn heading(&self) -> f32 {
//         self.heading
//     }
//
//     pub fn setup_location(&mut self, name: &str, pos: Vec3D, heading: f32) {
//         self.name = name.to_string();
//         self.pos = pos;
//         self.heading = heading;
//     }
// }

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
///
/// Now this prints from a JSON file, I need to figure out how to parse it, and write back into this.
/// I may build a location reader/writer in Rust for my ReVC Lua scripts, this was moved out of misc-test.
///
///
/// `json_path` Path to the JSON file.
///
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

/// Test for writing to a JSON file.
///
/// TODO Make this write a list of positions to a JSON file.
/// Currently, this just hard-codes a few values for testing.
///
/// <https://en.perfcode.com/rust/serde/process-files>
pub fn write_locations(json_path: &str) {
    // let data = fs::read_to_string(json_path).expect("Unable to read file");
    // let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse json");

    let location1 = MapLocation {
        name: "Location1".to_string(),
        pos: Vec3D::new(180.0, 180.0, 14.0),
        // pos_x: 180.0,
        // pos_y: 180.0,
        // pos_z: 14.0,
        heading: 90.0,
    };

    let location2 = MapLocation {
        name: "Location2".to_string(),
        pos: Vec3D::new(140.2, 150.3, 14.5),
        // pos_x: 140.2,
        // pos_y: 150.3,
        // pos_z: 14.5,
        heading: 40.0,
    };

    // https://devtoolsbuilder.com/learn/json/rust
    // This is one way to format the JSON data.
    // I figured this out! I looked at the JSON variable below, and it showed how I can do this.
    let location_json = json!({
        "Location1":  {
            "name": location1.name,
            "pos_x": location1.pos.x,
            "pos_y": location1.pos.y,
            "pos_z": location1.pos.z,
        },

        // https://stackoverflow.com/questions/28655362/how-does-one-round-a-floating-point-number-to-a-specified-number-of-digits#28656825
        // https://doc.rust-lang.org/std/fmt/

        "Location2":  {
            "name": location2.name,
            // Well now these get converted to a string in the JSON output.
            // This did something but now they don't have the values stripped from it anymore.
            "pos_x": format!("{:.3}", location2.pos.x).parse::<f32>().unwrap(),
            "pos_y": format!("{:.2}", location2.pos.y).parse::<f32>().unwrap(),
            "pos_z": format!("{:.2}", location2.pos.z).parse::<f32>().unwrap(),
        }
    });


    let file = File::create(json_path).unwrap();
    let writer = BufWriter::new(file);

    // serde_json::to_writer_pretty(writer, &location1).unwrap();
    serde_json::to_writer_pretty(writer, &location_json).unwrap();

    // Ok(())
}

/// Test for writing data to a JSON file.
///
/// TODO Make this write multiple values at once, so Location1, Location2 and so on.
///
/// Well this needs to be adapted to my write_locations format, I guess I messed something up in here...
///
/// https://www.slingacademy.com/article/reading-and-writing-json-files-in-rust-with-serde/
///
/// Example usage:
///
/// ```rust
/// use kcnet_lib::json_handler;///
///
/// let location1 = MapLocation {
///     name: "Location1".to_string(),
///     pos_x: 180.0,
///     pos_y: 180.0,
///     pos_z: 14.0,
///     heading: 90.0,
///  };
///
///  // Parse the location JSON data directly here.
///  // So it is adapted to my format.
///  let location_json = json!({
///     "Location1":  {
///     "name": location1.name,
///     "pos_x": location1.pos_x,
///     "pos_y": location1.pos_y,
///     "pos_z": location1.pos_z
///     }
///  });
///
/// json_handler::write_location("location-file.json", location_json);
/// ```
///
// pub fn write_location(json_path: &str, location: MapLocation) {
pub fn write_location(json_path: &str, location: serde_json::value::Value) {
    let json_data = serde_json::to_string_pretty(&location).unwrap();
    let mut file = File::create(json_path).expect("Unable to create file");
    file.write_all(&json_data.as_bytes()).expect("Unable to write data");
}
