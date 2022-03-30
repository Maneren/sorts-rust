use super::{heap_sort, hoare_partition, insertion_sort, Arr};

pub fn intro_sort(array: Arr, start: usize, end: usize) {
  intro_sort_inner(array, start, end, 2 * (end - start + 1).log(2) as usize)
}

fn intro_sort_inner(array: Arr, start: usize, end: usize, depth_limit: usize) {
  if end - start <= 16 {
    return insertion_sort(array, start, end);
  }

  if depth_limit == 0 {
    return heap_sort(array, start, end);
  }

  let pivot = hoare_partition(array.clone(), start, end);

  if pivot > start + 1 {
    intro_sort_inner(array.clone(), start, pivot - 1, depth_limit - 1);
  }

  if pivot + 1 < end {
    intro_sort_inner(array, pivot + 1, end, depth_limit - 1);
  }
}
