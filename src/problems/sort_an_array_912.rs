#[test]
fn run_problem() {
	// GeeksForGeeks sanity test: https://www.geeksforgeeks.org/heap-sort/
	// assert_eq!(sort_array(vec![1, 3, 5, 4, 6, 13, 10, 9, 8, 15, 17]), [1, 3, 4, 5, 6, 8, 9, 10, 13, 15, 17]);

	// LeetCode example 1
	assert_eq!(sort_array(vec![5, 2, 3, 1]), [1, 2, 3, 5]);
}

fn heapify(arr: &mut [i32], limit: usize, root: usize) {
	let mut large = root;
	let (lnode, rnode) = (2 * large + 1, 2 * large + 2);

	if lnode < limit && arr.get(lnode) > arr.get(large) {
		large = lnode;
	}

	if rnode < limit && arr.get(rnode) > arr.get(large) {
		large = rnode;
	}

	if root != large {
		arr.swap(root, large);
		heapify(arr, limit, large);
	}
}

fn sort(arr: &mut [i32]) {
	// Turn the array into a binary heap
	for idx in (0..arr.len() / 2).rev() {
		heapify(arr, arr.len(), idx);
	}

	// Perform the sort
	for idx in (0..arr.len()).rev() {
		// We swap the root and the current node
		arr.swap(0, idx);
		heapify(arr, idx, 0);
	}
}

fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
	sort(&mut nums);
	nums
}
