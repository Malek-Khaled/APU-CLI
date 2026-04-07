use std::{
    collections::HashMap,
    env::home_dir,
    fs::{File, create_dir},
    io::Error,
    path::PathBuf,
};

fn get_config_dir() -> Result<PathBuf, Error> {
    let home_dir = home_dir().unwrap();
    let config_dir = home_dir.join(".config/apu_schedules/");
    if !&config_dir.is_dir() {
        create_dir(&config_dir)?;
    }
    Ok(config_dir)
}

fn get_saved_intakes_file() -> Result<PathBuf, Error> {
    let config_file_path = get_config_dir()?.join("saved_intakes.json");
    if !config_file_path.is_file() {
        let config_hash = HashMap::from([("MAIN_INTAKE", ""), ("SECONDRY_INTAKES", "")]);
        let file = File::create(&config_file_path)?;
        serde_json::to_writer_pretty(file, &config_hash)?;
    }
    Ok(config_file_path)
}
fn get_schedules_dir() -> Result<PathBuf, Error> {
    let config_file_path = get_config_dir()?.join("schedules/");
    if !config_file_path.is_dir() {
        create_dir(&config_file_path)?;
    }
    Ok(config_file_path)
}

pub fn get_intake_dir(intake_code: &str) -> Result<PathBuf, Error> {
    let intake_dir_string = format!("{}/", &intake_code[..4]);
    let intake_dir = get_schedules_dir()?.join(intake_dir_string);
    if !intake_dir.is_dir() {
        create_dir(&intake_dir)?;
    }
    let intake_dir_string = format!("{}/", &intake_code[..7]);
    let intake_dir = intake_dir.join(intake_dir_string);
    if !intake_dir.is_dir() {
        create_dir(&intake_dir)?;
    }
    Ok(intake_dir)
}

pub fn set_main_intake(intake: &str) {
    let intake_file_path = get_saved_intakes_file().unwrap();
    let intake_file_reader = File::open(&intake_file_path).unwrap();
    let mut intake_hash: HashMap<String, String> =
        serde_json::from_reader(intake_file_reader).unwrap();
    intake_hash
        .entry("MAIN_INTAKE".to_string())
        .insert_entry(intake.to_string());
    let file = File::create(intake_file_path).unwrap();
    serde_json::to_writer_pretty(file, &intake_hash).unwrap();
}

pub fn get_main_intake() -> String {
    let intake_file_path = get_saved_intakes_file().unwrap();
    let intake_file_reader = File::open(&intake_file_path).unwrap();
    let intake_hash: HashMap<String, String> = serde_json::from_reader(intake_file_reader).unwrap();
    intake_hash.get("MAIN_INTAKE").unwrap().clone()
}
pub fn set_secondary_intake(intake: &str) {
    let intake_file_path = get_saved_intakes_file().unwrap();
    let intake_file_reader = File::open(&intake_file_path).unwrap();
    let mut intake_hash: HashMap<String, String> =
        serde_json::from_reader(intake_file_reader).unwrap();
    intake_hash
        .entry("SECONDRY_INTAKES".to_string())
        .insert_entry(intake.to_string());
    let file = File::create(intake_file_path).unwrap();
    serde_json::to_writer_pretty(file, &intake_hash).unwrap();
}

pub fn get_secondary_intake() -> String {
    let intake_file_path = get_saved_intakes_file().unwrap();
    let intake_file_reader = File::open(&intake_file_path).unwrap();
    let intake_hash: HashMap<String, String> = serde_json::from_reader(intake_file_reader).unwrap();
    intake_hash.get("SECONDRY_INTAKES").unwrap().clone()
}
