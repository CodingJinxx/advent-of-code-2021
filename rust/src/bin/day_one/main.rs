use std::{env, fs};
use std::fmt::format;
use std::fs::metadata;
use std::path::PathBuf;

enum MeasurementChange {
    Nothing,
    Increase,
    Decrease
}

fn main() {
    let path = env::args().nth(1).expect("Please provide a path");
    let path_metadata = metadata(&path).expect("Path invalid");
    let mut file_paths = Vec::new();
    if path_metadata.is_dir() {
        let directory = fs::read_dir(&path).expect("Unexpected Error, Path not Directory?");
        for file in directory {
            let file = file.expect("Unable to unwrap directory entry");
            let file_metadata = file.metadata().expect("Unable to parse metadata of directory entry");
            if file_metadata.is_file() {
                file_paths.push(file.path())
            }
        }
    }
    else {
        file_paths.push(PathBuf::from(path));
    }


    let mut totals : Vec<(String, String)> = vec!();
    for path in file_paths {
        let path_string = path.to_str().expect("Unable to convert pathbuf to string").to_string();
        println!("Executing Input File: {}", path_string);
        let file = fs::read_to_string(path).expect("Error reading file");

        let mut sonar_sweep_report = Vec::new();

        let mut previous_measurement : i32 = 0;
        let mut first_measurement = true;
        let mut measurement_increases = 0;
        let mut measurement_decreases = 0;
        for line in file.lines() {
            let measurement = line.parse::<i32>().unwrap();
            if first_measurement {
                previous_measurement = measurement;
                first_measurement = false;
            }
            let change = match measurement {
                i if i > previous_measurement =>
                    {
                        measurement_increases += 1;
                        MeasurementChange::Increase
                    },
                i if i < previous_measurement =>
                    {
                        measurement_decreases += 1;
                        MeasurementChange::Decrease
                    },
                _ => MeasurementChange::Nothing
            };
            previous_measurement = measurement;
            sonar_sweep_report.push((measurement, change));
        }

        for report in sonar_sweep_report {
            let change_output = match report.1 {
                MeasurementChange::Increase => "increased",
                MeasurementChange::Decrease => "decreased",
                MeasurementChange::Nothing => "N/A - no previous measurement"
            };
            println!("{} ({})", report.0, change_output);
        }
        let output = format!("\n\nTotal Increases: {}\nTotal Decreases: {}", measurement_increases, measurement_decreases);
        println!("{}", output);
        totals.push((path_string, output));
    }
    println!("\nSummary:\n");
    for total in totals {
        println!("Input File: {} {}", total.0, total.1);
    }
}
