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
  new_output: &mut Vec<T>,
) where
  T: Ord + Copy + fmt::Debug,
{
  let mut read_pointer = start + input_length;
  let mut new_output_pointer = 0;

  while read_pointer <= end && array[read_pointer] < array[0] {
    new_output[new_output_pointer] = array[read_pointer];
    new_output_pointer += 1;
    read_pointer += 1;
  }

  new_output[new_output_pointer] = array[0];
  let mut sublist_last_pointer = new_output_pointer;
  new_output_pointer += 1;

  let input_length = {
    let mut new_input_length = 0;
    for i in 1..input_length {
      let value = array[start + i];

      if new_output[sublist_last_pointer] < value {
        while read_pointer <= end && array[read_pointer] < value {
          new_output[new_output_pointer] = array[read_pointer];
          new_output_pointer += 1;
          read_pointer += 1;
        }

        new_output[new_output_pointer] = value;
        sublist_last_pointer = new_output_pointer;
        new_output_pointer += 1;
      } else {
        array[start + new_input_length] = value;
        new_input_length += 1;
      }
    }

    new_input_length
  };

  for i in 0..new_output_pointer {
    array[start + input_length + i] = new_output[i];
  }

  if input_length > 0 {
    strand_sort_inner(array, start, end, input_length, new_output)
  };
}
