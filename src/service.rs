use chrono::{Datelike, Local};
use std::{collections::HashMap, fs::File};

use crate::config::{get_intake_dir, get_main_intake};
use reqwest::Error;

async fn get_schedules() -> Result<Vec<serde_json::Value>, Error> {
    let url = "https://s3-ap-southeast-1.amazonaws.com/open-ws/weektimetable";
    let response: Vec<serde_json::Value> = reqwest::get(url).await?.json().await?;
    Ok(response)
}

async fn clean_json() -> HashMap<String, Vec<HashMap<String, String>>> {
    let response = get_schedules().await.unwrap();
    let mut cleaned_hash: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    let required_info = [
        "DATESTAMP_ISO",
        "GROUPING",
        // "INTAKE",
        "MODULE_NAME",
        "ROOM",
        "TIME_FROM",
        "TIME_TO",
    ];
    for obj in response {
        let mut hash_buff = HashMap::new();
        for item in required_info {
            let value = obj
                .get(item.to_string())
                .unwrap()
                .to_string()
                .replace("\"", "")
                .trim()
                .to_string();
            hash_buff.insert(item.to_string(), value);
        }
        let key = obj
            .get("INTAKE".to_string())
            .unwrap()
            .to_string()
            .replace("\"", "")
            .trim()
            .to_string();
        if !cleaned_hash.contains_key(&key) {
            let new_vec: Vec<HashMap<String, String>> = Vec::new();
            cleaned_hash.insert(key.to_string(), new_vec);
        }
        cleaned_hash.get_mut(&key).unwrap().push(hash_buff);
    }
    cleaned_hash
}

pub async fn sync_schedules_locally() {
    println!("Syncing schedules...");
    for intake in clean_json().await {
        let intake_code = intake.0;
        let value = intake.1;
        let intake_dir = get_intake_dir(&intake_code).unwrap();
        let intake_file_name = format!("{}.json", &intake_code);
        let intake_file = intake_dir.join(intake_file_name);
        let file = File::create(intake_file).unwrap();
        serde_json::to_writer_pretty(file, &value).unwrap();
    }
    println!("Schedules are up to date!");
}

fn read_schedules(intake: &str, group: &str) -> Vec<HashMap<String, String>> {
    let intake_dir = get_intake_dir(intake).unwrap();
    let intake_file_name = format!("{}.json", &intake);
    let intake_file = intake_dir.join(intake_file_name);
    let file = File::open(intake_file).unwrap();
    // let reader = Buf
    let schedule: Vec<HashMap<String, String>> = serde_json::from_reader(file).unwrap();
    let mut group_schedule = Vec::new();
    for class in schedule {
        if class.get("GROUPING").unwrap() == group {
            group_schedule.push(class);
        }
    }
    group_schedule
}
fn get_today() -> String {
    let today_num = Local::now().day();
    if today_num < 10 {
        format!("0{}", today_num)
    } else {
        today_num.to_string()
    }
}
fn get_month() -> String {
    let month_num = Local::now().month();
    if month_num < 10 {
        format!("0{}", month_num)
    } else {
        month_num.to_string()
    }
}

pub fn get_today_schedule(intake: &str, group: &str) -> Vec<HashMap<String, String>> {
    let today = get_today();
    let month = get_month();
    let mut today_schedule = Vec::new();
    let intake_schedule = read_schedules(intake, group);
    for schedule in intake_schedule {
        let date: Vec<&str> = schedule.get("DATESTAMP_ISO").unwrap().split("-").collect();
        let class_day = date[2];
        let class_month = date[1];
        if today.eq(&class_day) && class_month.eq(&month) {
            today_schedule.push(schedule);
        }
    }
    today_schedule
}

pub fn check_for_sync() -> bool {
    let today = get_today();
    let month = get_month();
    let intake = get_main_intake();
    let schedules = read_schedules(&intake.code.unwrap(), &intake.group.unwrap());
    let last_day_date: Vec<&str> = schedules
        .last()
        .unwrap()
        .get("DATESTAMP_ISO")
        .unwrap()
        .split("-")
        .collect();
    let last_day = last_day_date[2].to_string();
    let last_day_month = last_day_date[1].to_string();
    if last_day.eq(&today) && last_day_month.eq(&month) {
        return true;
    }
    false
}
