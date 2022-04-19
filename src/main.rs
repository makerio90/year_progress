use chrono::prelude::*;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
/// Simple program to show the progress of time
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// start date (%Y-%m-%d %H:%M:%S)
    #[clap(short, long)]
    start: Option<String>,

    /// end date (%Y-%m-%d %H:%M:%S)
    #[clap(short, long)]
    end: Option<String>,

    /// loop until end date reached
    #[clap(short)]
    l: bool,

    ///bar thickness
    #[clap(short, long, default_value_t = 50)]
    thickness: u32,

    ///hide percentage
    #[clap(short, long)]
    hide_percentage: bool,
}

fn main() {
    let args = Args::parse();
    // times
    let mut now = Utc::now();
    let start = match args.start {
        Some(s) => DateTime::<Utc>::from_utc(
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap(),
            Utc,
        ),
        None => Utc.ymd(now.year(), 1, 1).and_hms(0, 0, 0),
    };
    let end = match args.end {
        Some(s) => DateTime::<Utc>::from_utc(
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap(),
            Utc,
        ),
        None => Utc.ymd(now.year(), 12, 31).and_hms(12, 59, 59),
    };
    // math
    let total_seconds = (end.timestamp() - start.timestamp()) as u64;
    // bar
    let bar = ProgressBar::new(total_seconds);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(&format!("[{{bar:{}}}] {{msg}}", args.thickness))
            .progress_chars("=> "),
    );
    //bar.set_draw_delta(total_seconds / 100);
    loop {
        now = Utc::now();
        let seconds_ran = (now.timestamp() - start.timestamp()) as u64;
        let progress: f64 = (seconds_ran as f64 / total_seconds as f64) * 100.0;
        if progress > 100.0 {
            break;
        }
        bar.set_position(seconds_ran);
        if !args.hide_percentage {
            bar.set_message(format!("{}%", progress))
        }
        if !args.l {
            break;
        };
    }
    bar.finish_at_current_pos();
}
