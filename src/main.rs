extern crate clap;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};
use rayon::prelude::*;
use std::time::Instant;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
struct MyData {
    name: String,
    value: i32,
}

fn main() {
    let start_time = Instant::now();

    let matches = App::new("Data Processor")
        .version("1.0")
        .author("Your Name")
        .about("Process JSON and YAML data with Serde and Rayon")
        .arg(
            Arg::with_name("json_input")
                .short("j")
                .long("json-input")
                .value_name("JSON_FILE")
                .help("Sets the input JSON file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("yaml_input")
                .short("y")
                .long("yaml-input")
                .value_name("YAML_FILE")
                .help("Sets the input YAML file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output JSON file")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let json_input_filename = matches.value_of("json_input").unwrap();
    let yaml_input_filename = matches.value_of("yaml_input").unwrap();
    let output_filename = matches.value_of("output.json").unwrap_or("output.json");
    let output_filename_yaml = matches.value_of("output2.json").unwrap_or("output2.json");

    // Read JSON data from the JSON input file
    let mut json_input_file = File::open(json_input_filename).expect("Failed to open JSON input file");
    let mut json_string = String::new();
    json_input_file
        .read_to_string(&mut json_string)
        .expect("Failed to read JSON input file");

    // Deserialize JSON into a Vec<MyData>
    let json_data: Vec<MyData> = serde_json::from_str(&json_string).expect("Failed to parse JSON");

    // Read YAML data from the YAML input file
    let mut yaml_input_file = File::open(yaml_input_filename).expect("Failed to open YAML input file");
    let mut yaml_string = String::new();
    yaml_input_file
        .read_to_string(&mut yaml_string)
        .expect("Failed to read YAML input file");

    // Deserialize YAML into a Vec<MyData>
    let yaml_data: Vec<MyData> = serde_yaml::from_str(&yaml_string).expect("Failed to parse YAML");

    // Process JSON and YAML data in parallel using Rayon
    let processed_json_data: Vec<MyData> = json_data
        .par_iter()
        .map(|item| {
            // Simulate some processing on each item (e.g., converting name to uppercase)
            MyData {
                name: item.name.to_uppercase(),
                value: item.value,
            }
        })
        .collect();

    let processed_yaml_data: Vec<MyData> = yaml_data
        .par_iter()
        .map(|item| {
            // Simulate some processing on each item (e.g., doubling the value)
            MyData {
                name: item.name.to_uppercase(),
                value: item.value,
            }
        })
        .collect();

    // Serialize processed JSON data back to JSON
    let processed_json = serde_json::to_string(&processed_json_data).expect("Failed to serialize JSON");

    // Serialize processed YAML data back to JSON (you can change this to YAML if needed)
    let processed_yaml = serde_json::to_string(&processed_yaml_data).expect("Failed to serialize JSON");

    // Write the processed JSON data to the output file
    let mut output_file = File::create(output_filename).expect("Failed to create output file");

    output_file
        .write_all(processed_json.as_bytes())
        .expect("Failed to write to output file");

    let mut output_file2 = File::create(output_filename_yaml).expect("Failed to create output file");

    output_file2
        .write_all(processed_yaml.as_bytes())
        .expect("Failed to write to output file");

    // Calculate and print the elapsed time
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!(
        "Processing took {}.{:03} seconds",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}
