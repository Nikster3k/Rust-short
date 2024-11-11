struct Node<T> {
	value: T,
	next: Option<Box<Node<T>>>,
}

#[derive(Default)]
struct List<T> {
	head: Option<Box<Node<T>>>
}

#[allow(dead_code)]
impl<T> List<T> {
	fn new() -> Self {
		List{ head: None }
	}

	fn push_front(&mut self, value: T) {
		let mut new_node = Node{value, next: None};
		if let Some(head) = self.head.take() {
			new_node.next = Some(head);
		}
		self.head = Some(Box::new(new_node));
	}

	fn push_back(&mut self, value: T) {
		let new_node = Box::new(Node { value, next: None });
		match self.head {
			None => self.head = Some(new_node),
			Some(ref mut head) => {
				let mut tail = head;
				while let Some(ref mut next) = tail.next {
					tail = next;
				}
				tail.next = Some(new_node);
			}
		}
	}

	fn count(&self) -> usize {
		let mut len = 0;
		let mut current_node = &self.head;
		while let Some(node) = current_node {
			current_node = &node.next;
			len += 1;
		}
		len
	}

	fn get(&self, i: usize) -> Option<&T> {
		let mut current_node = &self.head;
		let mut idx = 0;
		while let Some(node) = current_node {
			if idx == i {
				return Some(&node.value);
			}
			current_node = &node.next;
			idx += 1;
		}
		None
	}

	fn get_mut(&mut self, i: usize) -> Option<&mut T> {
		let mut current_node = &mut self.head;
		let mut idx = 0;
		while let Some(node) = current_node {
			if idx == i {
				return Some(&mut node.value);
			}
			current_node = &mut node.next;
			idx += 1;
		}
		None
	}

	//learned cool map from Copilot thx
	fn remove_front(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			self.head = node.next;
			node.value
		})
	}

    /**
     * this is pure Copilot. U mad?
     */
	fn remove_back(&mut self) -> Option<T> {
		self.head.as_ref()?;
	
		if self.head.as_ref().unwrap().next.is_none() {
			return self.head.take().map(|node| node.value);
		}
	
		let mut current_node = &mut self.head;
		while let Some(ref mut node) = current_node {
			if node.next.as_ref().unwrap().next.is_none() {
				// Remove the last node
				return node.next.take().map(|node| node.value);
			}
			current_node = &mut node.next;
		}
	
		None
	}

	fn clear(&mut self) {
		while self.remove_front().is_some() { }
	}
}

impl<T: Clone> std::clone::Clone for List<T> {
	fn clone(&self) -> Self {
		let mut clone_list = List::new();
		let mut list_iter = &self.head;
		while let Some(ref x) = list_iter  {
			clone_list.push_back(x.value.clone());
			list_iter = &x.next;
		}
		clone_list
	}
}

impl<T> std::ops::Index<usize> for List<T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		match self.get(index) {
			Some(val) => val,
			_ => panic!("tried to access out of bound index {}", index)
		}
	}
}

impl<T> std::ops::IndexMut<usize> for List<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match self.get_mut(index) {
			Some(val) => val,
			_ => panic!("tried to access out of bound index {}", index)
		}
	}
}

#[cfg(test)]
#[test]
fn test_add_and_count() {
	let mut list: List<i32> = List::new();
	list.push_front(5);
	list.push_front(4);
	list.push_front(3);
	list.push_front(2);
	list.push_front(1);
	assert_eq!(list.count(), 5);
}

#[cfg(test)]
#[test]
fn test_get() {
	let mut list: List<i32> = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	list.push_back(5);
	assert_eq!(*list.get(2).unwrap(), 3);
}

#[cfg(test)]
#[test]
fn test_remove() {
	let mut list: List<i32> = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	list.push_back(5);
	list.remove_back();
	assert_eq!(list.count(), 4);
}

#[cfg(test)]
#[test]
fn test_push() {
	let mut list: List<i32> = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	list.push_back(5);
	assert_eq!(list.count(), 5);
}

#[cfg(test)]
#[test]
fn test_remove_empty() {
	let mut list: List<i32> = List::new();
	list.remove_back();
	list.remove_back();
	list.remove_back();
	assert!(list.remove_back().is_none());
}

#[cfg(test)]
#[test]
fn test_mut_get() {
	let mut list: List<i32> = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	match list.get_mut(2) {
		Some(valid) => *valid = 6,
		_ => ()
	}
	assert_eq!(*list.get(2).unwrap(), 6);
}

#[cfg(test)]
#[test]
fn test_indexing() {
	let mut list: List<i32> = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);

	assert_eq!(list[3], 4);
}

#[cfg(test)]
#[test]
fn test_indexing_mutable() {
	let mut list: List<i32> = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	list[3] = 15;
	list.push_back(5);
	list.push_back(6);

	assert_eq!(list[3], 15);
}

#[cfg(test)]
#[test]
fn test_indexing_mutable_add() {
	let mut list: List<i32> = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	list.push_back(5);
	list[4] += 15;
	list.push_back(6);

	assert_eq!(list[4], 20);
}


#[cfg(test)]
#[test]
fn test_list_clear() {
	let mut list: List<i32> = List::new();
	for i in 0..100 {
		list.push_front(i);
	}
	list.clear();
	assert_eq!(list.count(), 0);
}


#[cfg(test)]
#[test]
fn default_list_is_empty() {
    let list: List<i32> = Default::default();
    assert_eq!(list.count(), 0);
}

#[cfg(test)]
#[test]
fn cloned_list_are_equal() {
    let mut list = List::new();
    list.push_back(String::from("Hello"));
    list.push_back(String::from("World"));

    let cloned = list.clone();
    assert_eq!(cloned.count(), list.count());
    assert_eq!(&cloned[0], &list[0]);
    assert_eq!(&cloned[1], &list[1]);
}

#[cfg(test)]
#[test]
#[should_panic(expected = "tried to access out of bound index 10")]
fn out_of_bound_access_panics() {
    let mut list: List<u32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list[10], 42);
}
