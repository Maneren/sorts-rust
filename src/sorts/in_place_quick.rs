use std::fmt;

use super::Arr;

pub fn in_place_quick_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy + fmt::Debug,
{
  if start >= end {
    return;
  };
  let pivot = partition(array, start, end);

  if pivot != 0 {
    in_place_quick_sort(array, start, pivot - 1);
  }

  if pivot != end {
    in_place_quick_sort(array, pivot + 1, end);
  }
}

fn partition<T>(array: &mut Arr<T>, start: usize, end: usize) -> usize
where
  T: Ord + Copy,
{
  let mut pivot_index = start;

  for i in start..end {
    if array[i] < array[end] {
      array.swap(i, pivot_index);
      pivot_index += 1;
    }
  }

  array.swap(end, pivot_index);

  pivot_index
}
