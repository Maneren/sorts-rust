use super::Arr;

pub fn hoare_partition<T>(array: &mut Arr<T>, start: usize, end: usize) -> usize
where
  T: Ord + Copy,
{
  let mut i = start;
  let mut j = end;

  let pivot = array[((start + end) / 2)];

  loop {
    while array[i] < pivot {
      i += 1;
    }

    while array[j] > pivot {
      j -= 1;
    }

    if i >= j {
      return j;
    };

    array.swap(i, j);
  }
}

pub fn merge<T>(
  array: &mut Arr<T>,
  left_start: usize,
  left_end: usize,
  right_start: usize,
  right_end: usize,
  result: &mut Vec<T>,
) where
  T: Ord + Copy,
{
  let mut left_index = left_start;
  let mut right_index = right_start;

  let mut i = 0;

  while left_index <= left_end && right_index <= right_end {
    let left_num = array[left_index];
    let right_num = array[right_index];

    if left_num <= right_num {
      result[i] = left_num;
      left_index += 1;
    } else {
      result[i] = right_num;
      right_index += 1;
    }
    i += 1;
  }

  while left_index <= left_end {
    result[i] = array[left_index];
    left_index += 1;
    i += 1;
  }

  while right_index <= right_end {
    result[i] = array[right_index];
    right_index += 1;
    i += 1;
  }

  for (i, &el) in result[0..i].iter().enumerate() {
    array[left_start + i] = el;
  }
}
