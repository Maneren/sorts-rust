use super::Arr;

pub fn in_place_quick_sort(array: Arr, start: usize, end: usize) {
  let pivot = partition(array.clone(), start, end);

  if pivot > start + 1 {
    in_place_quick_sort(array.clone(), start, pivot - 1);
  }

  if pivot + 1 < end {
    in_place_quick_sort(array, pivot + 1, end);
  }
}

fn partition(array: Arr, start: usize, end: usize) -> usize {
  let mut pivot_index = start;
  let end_value = array.get(end);

  for i in start..end {
    if array.get(i) < end_value {
      array.swap(i, pivot_index);
      pivot_index += 1;
    }
  }

  array.swap(end, pivot_index);

  pivot_index
}
