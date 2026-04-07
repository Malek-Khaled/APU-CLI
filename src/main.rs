use clap::{Parser, arg, clap_derive, command};
use core::panic;
use std::env::args;

use crate::{
    config::{get_main_intake, get_secondary_intake, set_main_intake, set_secondary_intake},
    service::{get_today_schedule, sync_schedules_locally},
};

mod config;
mod service;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// To save main intake
    #[arg(long, default_value = "")]
    add_main: String,
    /// To save secondary intake
    #[arg(long, default_value = "")]
    add_sec: String,

    /// For showing secondary_intake
    #[arg(short)]
    bring_secondary_intake: bool,
}

#[tokio::main]
async fn main() {
    // sync_schedules_locally().await;
    let args = Cli::parse();
    let mut show_schedules = true;
    if !args.add_sec.is_empty() {
        set_secondary_intake(&args.add_sec);
        println!("Added in the config file.");
        show_schedules = false;
    }
    if !args.add_main.is_empty() {
        set_main_intake(&args.add_sec);
        println!("Added in the config file.");
        show_schedules = false;
    }
    if show_schedules {
        let secondary_intake = get_secondary_intake();
        let main_intake = get_main_intake();
        draw_schedules(&main_intake, "G2");
        if args.bring_secondary_intake {
            println!("-------------Secondary Intake----------");
            draw_schedules(&secondary_intake, "G1");
        }
    }
    //APU2F2602SE
}

fn draw_schedules(intake: &str, group: &str) {
    let schedules = get_today_schedule(intake, group);
    println!("---------------------------------------");
    for class in schedules {
        println!("|{}|", class.get("MODULE_NAME").unwrap());
        println!("From: {}", class.get("TIME_FROM").unwrap());
        println!("To: {}", class.get("TIME_TO").unwrap());
        println!("Place: {}", class.get("ROOM").unwrap());
        println!("---------------------------------------");
    }
}
