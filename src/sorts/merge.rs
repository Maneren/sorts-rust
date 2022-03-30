use super::{merge, Arr};

pub fn merge_sort(array: Arr, start: usize, end: usize) {
  let mut auxiliary = vec![Default::default(); array.len()];

  merge_sort_inner(array, start, end, &mut auxiliary)
}

fn merge_sort_inner(array: Arr, start: usize, end: usize, aux: &mut Vec<usize>) {
  if start >= end {
    return;
  }

  let middle = (end - start) / 2 + start;

  merge_sort_inner(array.clone(), start, middle, aux);
  merge_sort_inner(array.clone(), middle + 1, end, aux);

  merge(array, start, middle, middle + 1, end, aux);
}
