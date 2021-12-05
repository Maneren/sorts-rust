use super::{insertion_sort, Arr};

// https://www.geeksforgeeks.org/in-place-merge-sort/

pub fn in_place_merge_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  if start >= end {
    return;
  }

  let middle = (end - start) / 2 + start;

  in_place_merge_sort(array.clone(), start, middle);
  in_place_merge_sort(array.clone(), middle + 1, end);

  in_place_merge(array, start, middle, middle + 1, end);
}

pub fn in_place_merge<T>(
  array: Arr<T>,
  left_start: usize,
  left_end: usize,
  right_start: usize,
  right_end: usize,
) where
  T: Ord + Copy,
{
  let left_length = left_end - left_start + 1;
  let right_length = right_end - right_start + 1;

  // Return right now if we're done
  if left_length == 0 || right_length == 0 || *array.index(left_end) <= *array.index(right_start) {
    return;
  }

  if right_length <= 8 {
    insertion_sort(array, left_start, right_end);
    return;
  }

  // Find the pivot points.  Basically this is just
  // finding the point in 'a' where we can swap in the
  // first part of 'b' such that after the swap the last
  // element in 'a' will be less than or equal to the
  // least element in 'b'
  let mut left_pivot = left_start;
  let mut right_pivot = right_start;

  let mut i = left_start;
  while i <= left_end && right_pivot <= right_end {
    if *array.index(left_pivot) > *array.index(right_pivot) {
      right_pivot += 1;
    } else {
      left_pivot += 1;
    }

    i += 1
  }

  left_pivot += right_start - i;

  {
    // Swap first part of b with last part of a
    let mut left_pointer = left_pivot;
    let mut right_pointer = right_start;

    while left_pointer < right_start {
      array.swap(left_pointer, right_pointer);
      left_pointer += 1;
      right_pointer += 1;
    }
  }

  // Now merge the two sub-array pairings
  if left_pivot > left_start {
    in_place_merge(
      array.clone(),
      left_start,
      left_pivot - 1,
      left_pivot,
      left_end,
    );
  }
  if right_pivot > right_start {
    in_place_merge(array, right_start, right_pivot - 1, right_pivot, right_end);
  }
}
