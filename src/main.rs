use crate::{
    config::{get_main_intake, get_secondary_intake, set_main_intake, set_secondary_intake},
    service::{get_today_schedule, sync_schedules_locally},
};
use clap::Parser;

mod config;
mod service;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// To save main intake
    #[arg(long, default_value = "")]
    add_main: String,
    /// Choosing between G1 or G2
    #[arg(short, default_value = "1")]
    main_group: String,
    /// To save secondary intake
    #[arg(long, default_value = "")]
    add_sec: String,
    /// Choosing between G1 or G2
    #[arg(short, default_value = "1")]
    sec_group: String,

    /// For showing secondary_intake
    #[arg(short)]
    bring_secondary_intake: bool,
}

#[tokio::main]
async fn main() {
    // sync_schedules_locally().await;
    let args = Cli::parse();
    let mut writing_message = String::from("Added in the config file.");

    let mut show_schedules = true;
    if !args.add_sec.is_empty() {
        let group = format!("G{}", &args.sec_group);
        set_secondary_intake(&args.add_sec, &group);
        println!("{}", &writing_message);
        writing_message = String::new();
        show_schedules = false;
    }
    if !args.add_main.is_empty() {
        let group = format!("G{}", &args.main_group);
        set_main_intake(&args.add_main, &group);
        println!("{}", writing_message);
        show_schedules = false;
    }
    if show_schedules {
        let secondary_intake = get_secondary_intake();
        let main_intake = get_main_intake();

        draw_schedules(&main_intake.code.unwrap(), &main_intake.group.unwrap());
        if args.bring_secondary_intake {
            println!("-------------Secondary Intake----------");
            draw_schedules(
                &secondary_intake.code.unwrap(),
                &secondary_intake.group.unwrap(),
            );
        }
    }
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
