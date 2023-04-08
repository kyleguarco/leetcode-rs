/// #704. Binary Search
///
/// Given an array of integers `nums` which is sorted in ascending order, and an integer `target`, write a function to search `target` in `nums`.
/// If `target` exists, then return its index. Otherwise, return `-1`.
///
/// You must write an algorithm with O(log n) runtime complexity.

#[test]
fn run_leetcode_ex1() {
	let nums = vec![-1, 0, 3, 5, 9, 12];
	let target = 9;
	assert_eq!(search(nums, target), 4);
}

#[test]
fn run_leetcode_ex2() {
	let nums = vec![-1, 0, 3, 5, 9, 12];
	let target = 2;
	assert_eq!(search(nums, target), -1);
}

#[test]
fn run_leetcode_case5() {
	let nums = vec![-5];
	let target = 5;
	assert_eq!(search(nums, target), -1);
}

#[test]
fn run_leetcode_case6() {
	let nums = vec![2, 5];
	let target = 5;
	assert_eq!(search(nums, target), 1);
}

#[test]
fn run_example() {
	let nums = vec![5];
	let target = 5;
	assert_eq!(search(nums, target), 0);
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
	use std::cmp::Ordering;

	let (mut start, mut end) = (0, nums.len() - 1);
	while start <= end {
		let middle = (start + end) / 2;
		dbg!((start, middle, end));

		match nums.get(middle) {
			Some(n) => match n.cmp(&target) {
				Ordering::Less => start = middle + 1,
				Ordering::Equal => return middle as i32,
				Ordering::Greater => end = middle - 1,
			},
			None => return -1,
		}
	}

	-1
}
