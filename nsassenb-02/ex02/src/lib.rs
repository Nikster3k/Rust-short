#[derive(Debug, Copy, Clone)]
enum PizzaStatus {
	Ordered,
	Cooking = 2,
	Cooked = 7,
	Delivering = 10,
	Delivered = 17,
}

#[allow(dead_code)]
impl PizzaStatus {
    fn from_delivery_time(ordered_days_ago: u32) -> Self {
		match ordered_days_ago {
			(0..2) => PizzaStatus::Ordered,
			(2..7) => PizzaStatus::Cooking,
			(7..10) => PizzaStatus::Cooked,
			(10..17) => PizzaStatus::Delivering,
			_ => PizzaStatus::Delivered
		}
	}

    fn get_delivery_time_in_days(&self) -> u32 {
		PizzaStatus::Delivered as u32 - (*self) as u32
	}
}

#[cfg(test)]
mod test{
	use super::*;
	
	#[test]
	fn test_status_from_order_days_ago() {
		std::assert_eq!(PizzaStatus::from_delivery_time(6) as u32, PizzaStatus::Cooking as u32);
	}

	#[test]
	fn get_delivery_time_in_days_test() {
		let stat = PizzaStatus::Cooking;

		std::assert_eq!(stat.get_delivery_time_in_days(), 15);
	}
}
