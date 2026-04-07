use is_empty::IsEmpty;
use std::{
    collections::HashMap,
    env::home_dir,
    fs::{File, create_dir},
    io::Error,
    path::PathBuf,
};

#[derive(Debug, IsEmpty, Clone)]
pub struct Intake {
    pub code: Option<String>,
    pub group: Option<String>,
}
impl Intake {
    pub fn from(code: &str, group: &str) -> Intake {
        Intake {
            code: Option::Some(code.to_string()),
            group: Option::Some(group.to_string()),
        }
    }
}

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
        let config_hash = HashMap::from([
            ("MAIN_INTAKE", ""),
            ("MAIN_GROUP", ""),
            ("SECONDRY_INTAKE", ""),
            ("SECONDRY_GROUP", ""),
        ]);
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

fn set_intake(intake: &str, group: &str, kind: &str) {
    let intake_key = format!("{}_INTAKE", kind);
    let intake_group_key = format!("{}_GROUP", kind);
    let intake_file_path = get_saved_intakes_file().unwrap();
    let intake_file_reader = File::open(&intake_file_path).unwrap();
    let mut intake_hash: HashMap<String, String> =
        serde_json::from_reader(intake_file_reader).unwrap();
    intake_hash
        .entry(intake_key)
        .insert_entry(intake.to_string());
    intake_hash
        .entry(intake_group_key)
        .insert_entry(group.to_string());
    let file = File::create(intake_file_path).unwrap();
    serde_json::to_writer_pretty(file, &intake_hash).unwrap();
}
fn get_intake(kind: &str) -> Intake {
    let intake_key = format!("{}_INTAKE", kind);
    let intake_group_key = format!("{}_GROUP", kind);
    let intake_file_path = get_saved_intakes_file().unwrap();
    let intake_file_reader = File::open(&intake_file_path).unwrap();
    let intake_hash: HashMap<String, String> = serde_json::from_reader(intake_file_reader).unwrap();
    Intake::from(
        intake_hash.get(&intake_key).unwrap(),
        intake_hash.get(&intake_group_key).unwrap(),
    )
}

pub fn set_main_intake(intake: &str, group: &str) {
    set_intake(intake, group, "MAIN");
}

pub fn get_main_intake() -> Intake {
    get_intake("MAIN")
}

pub fn set_secondary_intake(intake: &str, group: &str) {
    set_intake(intake, group, "SECONDRY");
}

pub fn get_secondary_intake() -> Intake {
    get_intake("SECONDRY")
}
