use super::{insertion_sort, Arr};

pub fn weave_merge_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy + Default,
{
  let len = array.len();
  let mut auxiliary = vec![Default::default(); len];

  weave_merge_sort_inner(array, start, end, &mut auxiliary)
}

fn weave_merge_sort_inner<T>(array: &mut Arr<T>, start: usize, end: usize, aux: &mut Vec<T>)
where
  T: Ord + Copy,
{
  if start == end {
    return;
  }

  let middle = (end - start) / 2 + start;

  weave_merge_sort_inner(array, start, middle, aux);
  weave_merge_sort_inner(array, middle + 1, end, aux);

  let length = weak_merge(array, start, middle, middle + 1, end, aux);

  for (i, &el) in aux[0..length].iter().enumerate() {
    array[start + i] = el;
  }

  insertion_sort(array, start, end);
}

fn weak_merge<T>(
  array: &mut Arr<T>,
  left_start: usize,
  left_end: usize,
  right_start: usize,
  right_end: usize,
  result: &mut Vec<T>,
) -> usize
where
  T: Ord + Copy,
{
  let mut left_index = left_start;
  let mut right_index = right_start;
  let mut i = 0;

  while left_index <= left_end && right_index <= right_end {
    if i % 2 == 0 {
      result[i] = array[left_index];
      left_index += 1;
    } else {
      result[i] = array[right_index];
      right_index += 1;
    }
    i += 1;
  }

  while left_index <= left_end {
    result[i] = array[left_index];
    left_index += 1;
    i += 1;
  }
  while right_index <= right_end {
    result[i] = array[right_index];
    right_index += 1;
    i += 1;
  }

  i
}
