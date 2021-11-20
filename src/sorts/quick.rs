use super::Arr;

use rand::Rng;

pub fn quick_sort<T, const CAP: usize>(array: &mut Arr<T, CAP>, start: usize, end: usize)
where
  T: Ord + Copy + Default,
{
  let mut rng = rand::thread_rng();

  let mut auxiliaries = (Vec::with_capacity(CAP), Vec::with_capacity(CAP));

  auxiliaries.0.resize_with(CAP, Default::default);
  auxiliaries.1.resize_with(CAP, Default::default);

  quick_sort_inner(array, start, end, &mut auxiliaries, &mut rng)
}

fn quick_sort_inner<T, const CAP: usize>(
  array: &mut Arr<T, CAP>,
  start: usize,
  end: usize,
  auxes: &mut (Vec<T>, Vec<T>),
  rng: &mut impl Rng,
) where
  T: Ord + Copy,
{
  if end <= start {
    return;
  };

  let right_offset;

  let pivot = rng.gen_range(0..(end - start + 1)) + start;
  let pivot_value = array[pivot];

  let right = &mut auxes.0;
  let left = &mut auxes.1;

  let mut left_index = 0;
  let mut right_index = 0;

  for i in start..=end {
    let item = array[i];

    if item < pivot_value {
      left[left_index] = item;
      left_index += 1;
    } else {
      right[right_index] = item;
      right_index += 1;
    }
  }

  for (i, item) in left[0..left_index].iter().enumerate() {
    array[start + i] = *item;
  }

  right_offset = start + left_index;

  for (i, item) in right[0..right_index].iter().enumerate() {
    array[right_offset + i] = *item;
  }

  if right_offset != 0 {
    quick_sort_inner(array, start, right_offset - 1, auxes, rng);
  }

  if right_offset != end {
    quick_sort_inner(array, right_offset, end, auxes, rng);
  }
}
