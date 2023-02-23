use chrono::{Datelike, Local, NaiveDate};
use std::{env, process::exit};

fn main() {
	juneth(if let Some(arg) = env::args().nth(1) {
		NaiveDate::parse_from_str(&arg, "%Y-%m-%d").unwrap_or_else(|err| {
			eprintln!(
				"ERROR: {err} > \"{}\" \n\
				\0└╴     Date must follow format \"YYYY-MM-DD\"",
				&arg
			);
			exit(-1);
		})
	} else {
		Local::now().date_naive()
	});
}

fn juneth(now: NaiveDate) {
	let last_june_1st = NaiveDate::from_ymd(now.year() - i32::from(now.month() < 6), 6, 1);
	let juneth = (now - last_june_1st).num_days() + 1;
	let ending = match juneth % 10 {
		1 => "st",
		2 => "nd",
		3 => "rd",
		_ => "th",
	};

	println!("June {juneth}{ending}");
}
