use rand::Rng;

use super::Arr;

pub fn quick_sort(array: Arr, start: usize, end: usize) {
  let mut rng = rand::thread_rng();

  let len = array.len();
  let mut auxiliaries = (vec![Default::default(); len], vec![Default::default(); len]);

  quick_sort_inner(array, start, end, &mut auxiliaries, &mut rng)
}

fn quick_sort_inner(
  array: Arr,
  start: usize,
  end: usize,
  auxes: &mut (Vec<usize>, Vec<usize>),
  rng: &mut impl Rng,
) {
  let pivot = rng.gen_range(0..(end - start + 1)) + start;
  let pivot_value = array.get(pivot);

  let right = &mut auxes.0;
  let left = &mut auxes.1;

  let mut left_index = 0;
  let mut right_index = 0;

  for i in start..=end {
    let item = array.get(i);

    if item < pivot_value {
      left[left_index] = item;
      left_index += 1;
    } else {
      right[right_index] = item;
      right_index += 1;
    }
  }

  left[0..left_index]
    .iter()
    .enumerate()
    .for_each(|(i, &item)| array.set(start + i, item));

  let right_offset = start + left_index;

  right[0..right_index]
    .iter()
    .enumerate()
    .for_each(|(i, &item)| array.set(right_offset + i, item));

  if right_offset > start + 1 {
    quick_sort_inner(array.clone(), start, right_offset - 1, auxes, rng);
  }

  if right_offset < end {
    quick_sort_inner(array, right_offset, end, auxes, rng);
  }
}
