pub mod lib {
	pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
		let mut q = 0;
		let mut p = 0;
	
		while p < pattern.len(){
			if pattern[p] == b'*' {
				while p < pattern.len() && pattern[p] == b'*' {
					p += 1;
				}
				if p == pattern.len() {
					return true;
				}
				while q < query.len() && query[q] != pattern[p] {
					q += 1;
				}
			}
			if q == query.len() {
				return false;
			} 
			if pattern[p] != query[q] {
				return false;
			}
			p += 1;
			q += 1;
		}
		if q != query.len() {
			return false;
		}
		true
	}
}

// #[cfg(test)]
// mod test {
// 	use crate::lib::strpcmp;

// 	#[test]
// 	fn test_exact_star() {
// 		std::assert_eq!(strpcmp(b"This is the test String!", b"This*"), true);
// 	}

// 	#[test]
// 	fn test_star_exact() {
// 		std::assert_eq!(strpcmp(b"This is the test String!", b"*String!"), true);
// 	}

// 	#[test]
// 	fn test_star_exact_star() {
// 		std::assert_eq!(strpcmp(b"This is the test String!", b"*is*"), true);
// 	}

// 	#[test]
// 	fn test_exact_star_exact_star_exact() {
// 		std::assert_eq!(strpcmp(b"This is the test String!", b"This*the*String!"), true);
// 	}

// }