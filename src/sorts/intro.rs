use super::{common::hoare_partition, heap::heap_sort, insertion::insertion_sort, Arr};

pub fn intro_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  intro_sort_inner(array, start, end, 2 * (end - start + 1).log(2) as usize)
}

fn intro_sort_inner<T>(array: &mut Arr<T>, start: usize, end: usize, depth_limit: usize)
where
  T: Ord + Copy,
{
  if (end + 1 - start) <= 16 {
    return insertion_sort(array, start, end);
  }

  if depth_limit == 0 {
    return heap_sort(array, start, end);
  }

  let pivot = hoare_partition(array, start, end);

  intro_sort_inner(array, start, pivot - 1, depth_limit - 1);
  intro_sort_inner(array, pivot + 1, end, depth_limit - 1);
}
