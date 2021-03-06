use super::{hoare_partition, Arr};

pub fn hoare_quick_sort(array: Arr, start: usize, end: usize) {
  let pivot = hoare_partition(array.clone(), start, end);

  if pivot > start + 1 {
    hoare_quick_sort(array.clone(), start, pivot - 1);
  }

  if pivot + 1 < end {
    hoare_quick_sort(array, pivot + 1, end);
  }
}
