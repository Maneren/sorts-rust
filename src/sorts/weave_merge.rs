use super::{insertion_sort, Arr};

pub fn weave_merge_sort(array: Arr, start: usize, end: usize) {
  let len = array.len();
  let mut auxiliary = vec![Default::default(); len];

  weave_merge_sort_inner(array, start, end, &mut auxiliary)
}

fn weave_merge_sort_inner(array: Arr, start: usize, end: usize, aux: &mut Vec<usize>) {
  if start == end {
    return;
  }

  let middle = (end - start) / 2 + start;

  weave_merge_sort_inner(array.clone(), start, middle, aux);
  weave_merge_sort_inner(array.clone(), middle + 1, end, aux);

  let length = weak_merge(array.clone(), start, middle, middle + 1, end, aux);

  for (i, &el) in aux[0..length].iter().enumerate() {
    array.set(start + i, el);
  }

  insertion_sort(array, start, end);
}

fn weak_merge(
  array: Arr,
  left_start: usize,
  left_end: usize,
  right_start: usize,
  right_end: usize,
  result: &mut [usize],
) -> usize {
  let mut left_index = left_start;
  let mut right_index = right_start;
  let mut i = 0;

  while left_index <= left_end && right_index <= right_end {
    if i % 2 == 0 {
      result[i] = array.get(left_index);
      left_index += 1;
    } else {
      result[i] = array.get(right_index);
      right_index += 1;
    }
    i += 1;
  }

  while left_index <= left_end {
    result[i] = array.get(left_index);
    left_index += 1;
    i += 1;
  }
  while right_index <= right_end {
    result[i] = array.get(right_index);
    right_index += 1;
    i += 1;
  }

  i
}
