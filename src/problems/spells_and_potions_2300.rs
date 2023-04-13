#[test]
fn run_leetcode_ex1() {
	let spells = vec![5, 1, 3];
	let potions = vec![1, 2, 3, 4, 5];
	let success = 7;
	assert_eq!(successful_pairs(spells, potions, success), [4, 0, 3]);
}

#[test]
fn run_leetcode_ex2() {
	let spells = vec![3, 1, 2];
	let potions = vec![8, 5, 8];
	let success = 16;
	assert_eq!(successful_pairs(spells, potions, success), [2, 0, 2]);
}

#[test]
fn run_ex1() {
	let spells = vec![5];
	let potions = vec![6, 4, 5];
	let success = 8;
	assert_eq!(successful_pairs(spells, potions, success), [3]);
}

#[test]
fn run_ex2() {
	let spells = vec![5, 2, 1];
	let potions = vec![6];
	let success = 10;
	assert_eq!(successful_pairs(spells, potions, success), [1, 1, 0]);
}

fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
	use std::cmp::Ordering;

	// Sort `potions` for less comparisons.
	potions.sort_unstable();
	// And make it immutable again.
	let potions = potions;

	for s in spells.iter_mut() {
		// Search for some number that equals `success` or is close to it.
		let loc = potions
			.binary_search_by(|p| {
				let p = (*p as i64) * (*s as i64);
				match p.cmp(&success) {
					Ordering::Less => Ordering::Less,
					// We want to stop on the lesser element.
					Ordering::Equal => Ordering::Greater,
					Ordering::Greater => Ordering::Greater,
				}
				// Since an element will never be found that is equal, unwrap the error.
			})
			.unwrap_err();

		*s = (potions.len() - loc) as i32;
	}
	spells
}
