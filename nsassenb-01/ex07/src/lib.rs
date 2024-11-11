struct Task{
	start_time: u32,
	end_time: u32,
	cookies: u32,
}

#[allow(dead_code)]
fn time_manager(tasks: &mut Vec<Task>) -> u32 {
	let mut dp = vec![0; tasks.len() + 1];

	//need this as_mut_slice because compiler complains that I dont have to use vector
	tasks.as_mut_slice().sort_by(|a, b| a.end_time.cmp(&b.end_time));

	for i in 0..tasks.len() {
		let mut closest: i32 = -1;
		for x in (0..i).rev() {
			if tasks[x].end_time <= tasks[i].start_time {
				closest = x as i32;
				break;
			}
		};
		if closest != -1 {
			dp[i + 1] = dp[closest as usize + 1] + tasks[i].cookies;
		} else {
			dp[i + 1] = tasks[i].cookies;
		}
		dp[i + 1] = dp[i + 1].max(dp[i]);
	}

	dp[tasks.len()]
}

// AI is better
// if let Some(j) = (0..i).rfind(|&j| tasks[j].end_time <= tasks[i].start_time) {
// 	dp[i + 1] = dp[j + 1] + tasks[i].cookies;
// } else {
// 	dp[i + 1] = tasks[i].cookies;
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		let mut tasks = vec![
			Task{start_time: 1u32, end_time: 2u32, cookies: 5u32},
			Task{start_time: 3u32, end_time: 5u32, cookies: 10u32}
		];
		std::assert_eq!(time_manager(&mut tasks), 15);
	}

	#[test]
	fn test2() {
		let mut tasks = vec![
			Task{start_time: 0, end_time: 3, cookies: 10},
			Task{start_time: 4, end_time: 5, cookies: 5},
			Task{start_time: 6, end_time: 10, cookies: 25}
		];
		std::assert_eq!(time_manager(&mut tasks), 40);
	}

	#[test]
	fn test3() {
		let mut tasks = vec![
			Task{start_time: 0, end_time: 3, cookies: 10},
			Task{start_time: 3, end_time: 5, cookies: 5},
			Task{start_time: 5, end_time: 10, cookies: 25}
		];
		std::assert_eq!(time_manager(&mut tasks), 40);
	}

	#[test]
	fn test4() {
		let mut tasks = vec![
			Task{start_time: 0, end_time: 5, cookies: 10},
			Task{start_time: 3, end_time: 7, cookies: 5},
			Task{start_time: 5, end_time: 10, cookies: 25}
		];
		std::assert_eq!(time_manager(&mut tasks), 35);
	}

	#[test]
	fn test5() {
		let mut tasks = vec![
			Task{start_time: 0, end_time: 5, cookies: 10},
			Task{start_time: 3, end_time: 7, cookies: 5},
			Task{start_time: 5, end_time: 10, cookies: 25}
		];
		std::assert_eq!(time_manager(&mut tasks), 35);
	}

	#[test]
	fn test6() {
		let mut tasks = vec![
			Task{start_time: 0, end_time: 5, cookies: 1},
			Task{start_time: 3, end_time: 7, cookies: 30},
			Task{start_time: 5, end_time: 10, cookies: 25}
		];
		std::assert_eq!(time_manager(&mut tasks), 30);
	}

	#[test]
	fn test7() {
		let mut tasks = vec![
			Task{start_time: 0, end_time: 5, cookies: 1},
			Task{start_time: 3, end_time: 7, cookies: 24},
			Task{start_time: 5, end_time: 10, cookies: 25}
		];
		std::assert_eq!(time_manager(&mut tasks), 26);
	}

	//continue 185
	#[test]
	fn test8() {
		let mut tasks = vec![
			Task{start_time: 0, end_time: 5, cookies: 1},
			Task{start_time: 3, end_time: 7, cookies: 25},
			Task{start_time: 5, end_time: 10, cookies: 25}
		];
		std::assert_eq!(time_manager(&mut tasks), 26);
	}
}
