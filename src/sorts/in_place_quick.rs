use super::Arr;

pub fn in_place_quick_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  let pivot = partition(array.clone(), start, end);

  if pivot > start + 1 {
    in_place_quick_sort(array.clone(), start, pivot - 1);
  }

  if pivot + 1 < end {
    in_place_quick_sort(array, pivot + 1, end);
  }
}

fn partition<T>(array: Arr<T>, start: usize, end: usize) -> usize
where
  T: Ord + Copy,
{
  let mut pivot_index = start;

  for i in start..end {
    if *array.index(i) < *array.index(end) {
      array.swap(i, pivot_index);
      pivot_index += 1;
    }
  }

  array.swap(end, pivot_index);

  pivot_index
}
