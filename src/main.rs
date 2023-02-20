use chrono::{Datelike, Local, NaiveDate};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    let date = if args.len() > 1 {
        match NaiveDate::parse_from_str(&args[1], "%Y-%m-%d") {
            Ok(a) => a,
            Err(_) => {
                eprintln!("ERROR: Input date must follow format YYYY-MM-DD");
                exit(-1);
            }
        }
    } else {
        Local::now().date().naive_local()
    };

    juneth(date);
}

fn juneth(now: NaiveDate) {
    let last_june_1st = if now.month() >= 6 {
        NaiveDate::from_ymd(now.year(), 6, 1)
    } else {
        NaiveDate::from_ymd(now.year() - 1, 6, 1)
    };

    let juneth = now.num_days_from_ce() - last_june_1st.num_days_from_ce() + 1;
    let ending = match juneth % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    println!("June {juneth}{ending}");
}
