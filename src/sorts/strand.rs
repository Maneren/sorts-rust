use core::fmt;

use super::Arr;

pub fn strand_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy + Default + fmt::Debug,
{
  let len = array.len();
  let mut auxiliary = vec![Default::default(); len];

  strand_sort_inner(array, start, end, end - start + 1, &mut auxiliary)
}

fn strand_sort_inner<T>(
  array: &mut Arr<T>,
  start: usize,
  end: usize,
  input_length: usize,
  aux: &mut Vec<T>,
) where
  T: Ord + Copy + fmt::Debug,
{
  if input_length == 0 {
    return;
  }

  println!("> {}..{}, {}", start, end, input_length);

  let mut sublist = vec![array[start]];
  let output = Vec::from(&array[start + input_length..=end]);

  let input_length = {
    let mut new_input_length = 0;
    for i in 1..input_length {
      let value = array[start + i];

      if sublist.last().unwrap() < &value {
        sublist.push(value);
      } else {
        array[start + new_input_length + 1] = value;
        new_input_length += 1;
      }
    }

    new_input_length
  };

  let output_length = strand_merge(output, sublist, aux);
  let output = Vec::from(&aux[..output_length]);

  output.iter().enumerate().for_each(|(i, &item)| {
    let index = input_length + i;
    array[start + index] = item;
  });

  strand_sort_inner(array, start, end, input_length, aux);
}

fn strand_merge<T>(left: Vec<T>, right: Vec<T>, aux: &mut Vec<T>) -> usize
where
  T: Ord + Copy,
{
  let mut left_index = 0;
  let mut right_index = 0;

  let mut i = 0;

  while left_index < left.len() && right_index < right.len() {
    let left_num = left[left_index];
    let right_num = right[right_index];

    if left_num < right_num {
      aux[i] = left_num;
      left_index += 1;
    } else {
      aux[i] = right_num;
      right_index += 1;
    }
    i += 1
  }
  while left_index < left.len() {
    aux[i] = left[left_index];
    left_index += 1;
    i += 1
  }
  while right_index < right.len() {
    aux[i] = right[right_index];
    right_index += 1;
    i += 1
  }

  i
}
