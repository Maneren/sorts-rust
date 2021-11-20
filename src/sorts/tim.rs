use super::{common::merge, insertion_sort, Arr};

pub fn tim_sort<T, const CAP: usize>(array: &mut Arr<T, CAP>, start: usize, end: usize)
where
  T: Ord + Copy + Default,
{
  let mut auxiliary = Vec::with_capacity(CAP);

  auxiliary.resize_with(CAP, Default::default);

  merge_sort_inner(array, start, end, &mut auxiliary)
}

pub fn merge_sort_inner<T, const CAP: usize>(
  array: &mut Arr<T, CAP>,
  start: usize,
  end: usize,
  aux: &mut Vec<T>,
) where
  T: Ord + Copy,
{
  if start >= end {
    return;
  }

  if end - start < 8 {
    return insertion_sort(array, start, end);
  }

  let middle = (end - start) / 2 + start;

  merge_sort_inner(array, start, middle, aux);
  merge_sort_inner(array, middle + 1, end, aux);

  merge(array, start, middle, middle + 1, end, aux);
}
