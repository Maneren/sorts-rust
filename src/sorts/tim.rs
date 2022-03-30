use super::{insertion_sort, merge, Arr};

pub fn tim_sort(array: Arr, start: usize, end: usize) {
  let mut auxiliary = vec![Default::default(); array.len()];

  tim_sort_inner(array, start, end, &mut auxiliary)
}

fn tim_sort_inner(array: Arr, start: usize, end: usize, aux: &mut Vec<usize>) {
  if start >= end {
    return;
  }

  if end - start <= 16 {
    return insertion_sort(array, start, end);
  }

  let middle = (end - start) / 2 + start;

  tim_sort_inner(array.clone(), start, middle, aux);
  tim_sort_inner(array.clone(), middle + 1, end, aux);

  merge(array, start, middle, middle + 1, end, aux);
}
