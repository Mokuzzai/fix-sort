







#[deprecated(since = "0.1.0", note = "do not use outside library")]
pub trait FixDirty<T: Ord> {
	fn fix_dirty(&self, slice: &mut [T]);
}

#[allow(deprecated)]
impl<T: Ord> FixDirty<T> for usize {
	fn fix_dirty(&self, slice: &mut [T]) {
		let this = *self;

		assert!(this < slice.len());

		let (start, rest) = slice.split_at_mut(this);

		if let Some((this, rest)) = rest.split_first_mut() {
			let index = start.partition_point(|item| item < this);
		} else {
			last(start)
		}
	}
}

#[allow(deprecated)]
pub fn fix_dirty<T: Ord>(slice: &mut [T], items: impl FixDirty<T>){
	items.fix_dirty(slice)
}

pub fn last<T: Ord>(slice: &mut [T]) {
	// 'a
	if slice.len() < 2 {
		return
	}

	let Some((last, start)) = slice.split_last() else { unreachable!() };

	let index = start.partition_point(|item| item < last);

	let ptr = slice.as_mut_ptr();
	let len = slice.len();

	if index == len - 1 {
		return
	}

	let last = unsafe {
		// SAFETY: this is returns the last element
		// NOTE: the slice is know not to be empty: 'a
		std::ptr::read(ptr.add(len - 1))
	};

	// 0 1 2 3 4 5 6

	unsafe {
		// len = 2
		// index = 0
		//             0               1                   1
		// copy first item to replace second item
		std::ptr::copy(ptr.add(index), ptr.add(index + 1), len - index - 1);
		// copy second item to become first item
		std::ptr::write(ptr.add(index), last);
	}
}
