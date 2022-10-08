use chrono::{Datelike, Local, NaiveDate};
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let date = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d")
			.expect("Input date must follow format 2022-06-01");
		juneth(date);
	} else {
		let now = Local::now().date().naive_local();
		juneth(now);
	}
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
