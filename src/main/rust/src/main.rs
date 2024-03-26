use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use rand::Rng;
use std::time::Instant;

fn check_args(file_args: &[String]) {
    if file_args.len() != 2 || file_args[1].parse::<i32>().unwrap_or(0) <= 0 {
        println!("Usage:  create_measurements.sh <positive integer number of records to create>");
        println!("        You can use underscore notation for large number of records.");
        println!("        For example:  1_000_000_000 for one billion");
        std::process::exit(1);
    }
}

fn build_weather_station_name_list() -> Vec<String> {
    let mut station_names = Vec::new();
    let file_contents = std::fs::read_to_string("../../../data/weather_stations.csv").unwrap();
    for station in file_contents.lines() {
        if !station.contains("#") {
            let station_name = station.split(';').next().unwrap().to_string();
            station_names.push(station_name);
        }
    }
    station_names.sort();
    station_names.dedup();
    station_names
}

fn convert_bytes(num: f64) -> String {
    let units = ["bytes", "KiB", "MiB", "GiB"];
    let mut num = num;
    for unit in &units {
        if num < 1024.0 {
            return format!("{:.1} {}", num, unit);
        }
        num /= 1024.0;
    }
    format!("{:.1} TiB", num)
}

fn format_elapsed_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    let hours = minutes / 60;
    let minutes = minutes % 60;
    if hours > 0 {
        format!("{} hours {} minutes {} seconds", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{} minutes {} seconds", minutes, seconds)
    } else {
        format!("{} seconds", seconds)
    }
}

fn estimate_file_size(weather_station_names: &[String], num_rows_to_create: i32) -> String {
    let mut max_string = 0;
    let mut min_string = std::usize::MAX;
    let mut per_record_size = 0.0;

    for station in weather_station_names {
        let station_len = station.len();
        if station_len > max_string {
            max_string = station_len;
        }
        if station_len < min_string {
            min_string = station_len;
        }
        per_record_size = ((max_string + min_string * 2) + 7) as f64;
    }

    let total_file_size = (num_rows_to_create as f64 * per_record_size) as u64;
    let human_file_size = convert_bytes(total_file_size as f64);

    format!("Estimated max file size is:  {}.\nTrue size is probably much smaller (around half).", human_file_size)
}

fn build_test_data(weather_station_names: &[String], num_rows_to_create: i32) {
    let start_time = Instant::now();
    let coldest_temp = -99.9;
    let hottest_temp = 99.9;
    let mut rng = rand::thread_rng();
    let mut station_names_10k_max = Vec::new();
    for _ in 0..10_000 {
        let random_index = rng.gen_range(0..weather_station_names.len());
        station_names_10k_max.push(weather_station_names[random_index].clone());
    }
    let batch_size = 10_000;
    let progress_step = std::cmp::max(1, (num_rows_to_create / batch_size) / 100);
    println!("Building test data...");

    let file_path = Path::new("../../../data/measurements.txt");
    let mut file = File::create(file_path).unwrap();

    for s in 0..(num_rows_to_create / batch_size) {
        let mut batch = Vec::new();
        for _ in 0..batch_size {
            let random_index = rng.gen_range(0..station_names_10k_max.len());
            let station = &station_names_10k_max[random_index];
            let temperature = rng.gen_range(coldest_temp..=hottest_temp);
            let line = format!("{};{:.1}\n", station, temperature);
            batch.push(line);
        }
        let batch_str = batch.join("");
        file.write_all(batch_str.as_bytes()).unwrap();

        // Update progress bar every 1%
        if s % progress_step == 0 || s == (num_rows_to_create / batch_size) - 1 {
            let progress = (s + 1) * 50 / (num_rows_to_create / batch_size);
            let progress_usize = progress as usize;
            print!("\r[{}{}] {}%", "=".repeat(progress_usize), " ".repeat(50 - progress_usize), (s + 1) * 100 / (num_rows_to_create / batch_size));
            std::io::stdout().flush().unwrap();
        }
    }
    println!();
    let elapsed_time = start_time.elapsed().as_secs();
    let file_size = file_path.metadata().unwrap().len();
    let human_file_size = convert_bytes(file_size as f64);

    println!("Test data successfully written to 1brc/data/measurements.txt");
    println!("Actual file size:  {}", human_file_size);
    println!("Elapsed time: {}", format_elapsed_time(elapsed_time));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);
    let num_rows_to_create = args[1].parse::<i32>().unwrap();
    let weather_station_names = build_weather_station_name_list();
    println!("{}", estimate_file_size(&weather_station_names, num_rows_to_create));
    build_test_data(&weather_station_names, num_rows_to_create);
    println!("Test data build complete.");
}