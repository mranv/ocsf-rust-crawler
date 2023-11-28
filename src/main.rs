use reqwest;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use serde_json::Value;

fn get_and_save_json(url: &str, filename: &str) {
    // Fetch JSON data from the URL
    let response = reqwest::blocking::get(url);

    // Check if the request was successful (status code 200)
    if let Ok(response) = response {
        if response.status().is_success() {
            // Parse JSON data
            let body = response.text().unwrap();
            let json_data: Value = serde_json::from_str(&body).unwrap();

            // Save JSON data to the specified file
            let mut file = File::create(filename).expect("Unable to create file");
            serde_json::to_writer_pretty(&mut file, &json_data).expect("Unable to write JSON to file");
            writeln!(&mut file, "\n \n \n \n \n").expect("Unable to write newline to file");

            println!("JSON data saved to {}", filename);
        } else {
            println!("Failed to fetch JSON data. Status code: {}", response.status());
        }
    } else {
        println!("Failed to fetch JSON data.");
    }
}

fn main() {
    // Replace the URL with your actual URL
    let url = "http://192.168.43.108:8080/sample/classes/account_change?profiles=";

    // Set the duration for the script to run (5 minutes)
    let duration = Duration::from_secs(5 * 60);

    // Generate a timestamp for the filename
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
    let filename = format!("ocsf/ocsf_schema_sample_data{}.json", timestamp);

    let start_time = Instant::now();

    while Instant::now() - start_time < duration {
        get_and_save_json(url, &filename);
        std::thread::sleep(Duration::from_secs(1));
    }

    println!("Script completed.");
}
