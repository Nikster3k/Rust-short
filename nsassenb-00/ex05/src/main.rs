const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_leap_year(year: u32) -> bool {
	std::assert!(year >= 1);
	(year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn num_days_in_month(year: u32, month: u32) -> u32{
	assert!(month > 0 && month < 13);
	if month == 2 && is_leap_year(year){
		DAYS[(month - 1) as usize] + 1
	} else {
		DAYS[(month - 1) as usize]
	}
}

fn print_day(year: u32, month: u32) {
	println!("Friday, {} 13, {}", MONTHS[(month - 1) as usize], year);
}

fn main() {
	let mut week_day: u8 = 1;
	for year in 1..2025{
		for month in 1..13{
			for day in 1..(num_days_in_month(year, month) + 1){
				if day == 13 && week_day == 5 {
					print_day(year, month);
				}
				week_day += 1;
				if week_day == 8{
					week_day = 1;
				}
				if day == 7 && month == 10 && year == 2024 {
					return;
				}
			}
		}
	}
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn check_1600_is_leap(){
		std::assert_eq!(is_leap_year(1600), true);
	}

	#[test]
	fn check_1500_is_regular(){
		std::assert_eq!(is_leap_year(1500), false);
	}

	#[test]
	fn check_2004_is_regular(){
		std::assert_eq!(is_leap_year(2004), true);
	}

	#[test]
	fn check_2003_is_regular(){
		std::assert_eq!(is_leap_year(2003), false);
	}

	#[test]
	fn check_feburary_days(){
		std::assert_eq!(num_days_in_month(2004, 2), 29);
		std::assert_eq!(num_days_in_month(2003, 2), 28);
	}

	#[test]
	fn check_other_months_days_regular(){
		for month in 1..13{
			if month == 2{
				continue;
			}
			std::assert_eq!(num_days_in_month(2003, month), num_days_in_month(2004, month));
		}
	}

	#[test]
	#[should_panic]
	fn check_panic_wrong_moth(){
		num_days_in_month(1, 44);
	}

	#[test]
	#[should_panic]
	fn check_year_0(){
		is_leap_year(0);
	}
}
