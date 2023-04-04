/// #443. String Compression
/// Given an array of characters `chars`, compress it using the following algorithm:
///
/// Begin with an empty string `s`. For each group of consecutive repeating characters in chars:
///
/// * If the group's length is 1, append the character to `s`.
/// * Otherwise, append the character followed by the group's length.
///
/// The compressed string `s` should not be returned separately, but instead,
/// be stored in the input character array `chars`. Note that group lengths that
/// are `10` or longer will be split into multiple characters in `chars`.
///
/// After you are done modifying the input array, return the new length of the array.
///
/// You must write an algorithm that uses only constant extra space.

#[test]
fn run_leetcode_ex1() {
	let mut string = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
	compress(&mut string);
	assert_eq!(string, ['a', '2', 'b', '2', 'c', '3']);
	assert_eq!(string.len(), 6);
}

#[test]
fn run_leetcode_ex2() {
	let mut string = vec!['a'];
	compress(&mut string);
	assert_eq!(string, ['a']);
	assert_eq!(string.len(), 1);
}

#[test]
fn run_leetcode_ex3() {
	let mut string = vec![
		'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
	];
	compress(&mut string);
	assert_eq!(string, ['a', 'b', '1', '2']);
	assert_eq!(string.len(), 4);
}

#[test]
fn run_leetcode_case58() {
	let mut string = vec!['l', 't', '[', '3', 's', ']', '_', 'g', 'a', 'N'];
	compress(&mut string);
	assert_eq!(string, ['l', 't', '[', '3', 's', ']', '_', 'g', 'a', 'N']);
	assert_eq!(string.len(), 10);
}

const FILLER: char = '\u{1B}';

fn compress(chars: &mut Vec<char>) -> i32 {
	if chars.len() < 2 {
		return chars.len() as i32;
	}

	// Begin by going chunk by chunk, counting
	// each element within each repeating subslice.
	let mut rem = chars.as_mut_slice();
	while let Some((head, tail)) = split_mut(rem) {
		rem = tail;

		// Continue if there's one of the character.
		if head.len() < 2 {
			continue;
		}

		let mut count = head.len();
		// The number of digits in `count`
		let digits = ((count as f64).log10().floor() + 1.0) as usize;

		// We use this index for putting the counting
		// characters in the slice. We guarantee there's
		// at least one free space in the slice to replace
		// with a number.
		let mut from = digits;

		while count > 0 {
			let elem = head.get_mut(from).expect("Out of bounds");
			// This replaces the current element with
			// one of the digits in `count`.
			*elem = char::from_digit(count.rem_euclid(10) as u32, 10)
				.expect("Couldn't convert to base 10");
			from -= 1;
			count /= 10;
		}

		// We'll remove these later
		head[digits + 1..].fill(FILLER);
	}

	// Pass over the input to remove `FILLER`.
	let mut index = chars.len() - 1;
	while index > 0 {
		if chars.get(index) == Some(&FILLER) {
			chars.remove(index);
		}

		index -= 1;
	}

	chars.len() as i32
}

/// Splits a slice up based on recurring elements. This is code for Rust's
/// experimental `GroupByMut` feature. See `core::slice::group_by_mut`.
///
/// `split` assumes that `slice` has blocks of elements that repeat.
fn split_mut<'a, T: PartialEq + 'a>(slice: &'a mut [T]) -> Option<(&'a mut [T], &'a mut [T])> {
	if slice.is_empty() {
		None
	} else {
		let mut len = 1;
		let mut iter = slice.windows(2);
		while let Some([l, r]) = iter.next() {
			if l == r {
				len += 1
			} else {
				break;
			}
		}
		Some(slice.split_at_mut(len))
	}
}
