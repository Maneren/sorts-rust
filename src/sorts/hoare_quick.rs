use super::{common::hoare_partition, Arr};

pub fn hoare_quick_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  if start >= end {
    return;
  };

  let pivot = hoare_partition(array, start, end);

  if pivot != 0 {
    hoare_quick_sort(array, start, pivot - 1);
  }

  if pivot != end {
    hoare_quick_sort(array, pivot + 1, end);
  }
}
