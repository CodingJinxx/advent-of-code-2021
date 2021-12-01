use std::{env, fs};

enum MeasurementChange {
    Nothing,
    Increase,
    Decrease
}

fn main() {
    let path = env::args().nth(1).expect("Please provide a path");
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
    println!("\n\nTotal Increases: {}\nTotal Decreases: {}", measurement_increases, measurement_decreases);
}
