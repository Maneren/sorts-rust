use super::{hoare_partition, Arr};

pub fn hoare_quick_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  let pivot = hoare_partition(array.clone(), start, end);

  if pivot > start + 1 {
    hoare_quick_sort(array.clone(), start, pivot - 1);
  }

  if pivot + 1 < end {
    hoare_quick_sort(array.clone(), pivot + 1, end);
  }
}
