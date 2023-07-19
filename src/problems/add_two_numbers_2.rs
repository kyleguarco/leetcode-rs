//! #2. Add Two Numbers
//! https://leetcode.com/problems/add-two-numbers/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

impl ListNode {
	#[inline]
	fn new(val: i32) -> Self {
		Self { next: None, val }
	}
}

impl<const N: usize> From<[i32; N]> for ListNode {
	fn from(value: [i32; N]) -> Self {
		let mut list = Self::new(value[0]);

		if let Some(value) = value.get(1..) {
			let mut curr = &mut list;
			for num in value {
				curr.next = Some(Box::new(Self::new(*num)));
				curr = curr.next.as_mut().unwrap();
			}
		}

		list
	}
}

// struct ListIter<'a> {
// 	curr: Option<&'a ListNode>,
// }

// impl<'a> Iterator for ListIter<'a> {
// 	type Item = &'a ListNode;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		let val = self.curr;

// 		self.curr = match self.curr {
// 			Some(node) => node.next.as_deref(),
// 			None => None,
// 		};

// 		val
// 	}
// }

struct ListIterMut<'a> {
	curr: Option<&'a mut ListNode>,
}

impl<'a> Iterator for ListIterMut<'a> {
	type Item = &'a mut ListNode;

	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			let mval = self
				.curr
				.as_deref_mut()
				.map(|node| &mut *(node as *mut ListNode));

			let node = self
				.curr
				.as_deref_mut()
				.map(|node| &mut *(node as *mut ListNode));
			self.curr = match node {
				Some(node) => node.next.as_deref_mut(),
				None => None,
			};

			mval
		}
	}
}

struct ZipLong<A, B> {
	first: A,
	second: B,
}

trait ZipLongest<B>
where
	Self: Sized,
{
	fn zip_longest(self, second: B) -> ZipLong<Self, B> {
		ZipLong { first: self, second }
	}
}

impl<A, B> Iterator for ZipLong<A, B>
where
	A: Iterator,
	B: Iterator,
{
	type Item = (Option<A::Item>, Option<B::Item>);

	fn next(&mut self) -> Option<Self::Item> {
		match (self.first.next(), self.second.next()) {
			(None, None) => None,
			(a, b) => Some((a, b)),
		}
	}
}

impl<I, B> ZipLongest<B> for I
where
	Self: Iterator + Sized,
	B: Iterator,
{
}

impl ListNode {
	fn iter_mut(&mut self) -> ListIterMut<'_> {
		ListIterMut { curr: Some(self) }
	}
}

#[test]
fn leetcode_ex1() {
	let l1 = Box::new(ListNode::from([2, 4, 3]));
	let l2 = Box::new(ListNode::from([5, 6, 4]));
	add_two_numbers(Some(l1), Some(l2));
}

#[test]
fn leetcode_ex3() {
	let l1 = Box::new(ListNode::from([9, 9, 9, 9, 9, 9, 9]));
	let l2 = Box::new(ListNode::from([9, 9, 9, 9]));
	add_two_numbers(Some(l1), Some(l2));
}

fn add_two_numbers(
	mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
	// Since we have ownership of `l1` and `l2`, we can destroy them however we see fit.
	let l1r = l1.as_mut().unwrap();
	let l2r = l2.as_mut().unwrap();

	#[derive(Debug)]
	enum Choice<'a> {
		Neither,
		L1(&'a mut ListNode),
		L2(&'a mut ListNode),
	}

	// The carry bit used for manual addition.
	let mut carry = 0;
	let mut choice = Choice::Neither;

	fn get_val<'a>(list: &'a Option<&'a mut ListNode>) -> &'a i32 {
		list.as_deref().map(|n| &n.val).unwrap_or(&0)
	}

	for (l1m, l2m) in l1r.iter_mut().zip_longest(l2r.iter_mut()) {
		let mut new = get_val(&l1m) + get_val(&l2m) + carry;
		carry = 0; // If `new` is less than 10, then we should set this to zero since we just used the carry.

		if new >= 10 {
			carry = new / 10; // Floors and returns an i32.
			new %= 10;
		}

		// Now we overwrite both, since we don't know which one is the longest slice.
		l1m.map(|n| {
			(*n).val = new;
			choice = Choice::L1(n);
		});
		l2m.map(|n| {
			(*n).val = new;
			choice = Choice::L2(n);
		});
	}

	let next = match carry {
		0 => None,
		_ => Some(Box::new(ListNode::new(carry))),
	};

	// We're guaranteed to return the longest slice here.
	match choice {
		Choice::L1(tail) => {
			tail.next = next;
			l1
		}
		Choice::L2(tail) => {
			tail.next = next;
			l2
		}
		Choice::Neither => unreachable!(),
	}
}
