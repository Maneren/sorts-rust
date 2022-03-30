use super::Arr;

pub fn hoare_partition(array: Arr, start: usize, end: usize) -> usize {
  let mut i = start;
  let mut j = end;

  let pivot_index = start + (end - start) / 2;
  let pivot = array.get(pivot_index);

  loop {
    while array.get(i) < pivot {
      i += 1;
    }

    while array.get(j) > pivot {
      j -= 1;
    }

    if i >= j {
      return j;
    };

    array.swap(i, j);
  }
}

pub fn merge(
  array: Arr,
  left_start: usize,
  left_end: usize,
  right_start: usize,
  right_end: usize,
  result: &mut [usize],
) {
  let mut left_index = left_start;
  let mut right_index = right_start;

  let mut i = 0;

  while left_index <= left_end && right_index <= right_end {
    let left_num = array.get(left_index);
    let right_num = array.get(right_index);

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
    result[i] = array.get(left_index);
    left_index += 1;
    i += 1;
  }

  while right_index <= right_end {
    result[i] = array.get(right_index);
    right_index += 1;
    i += 1;
  }

  for (i, &el) in result[0..i].iter().enumerate() {
    array.set(left_start + i, el);
  }
}
