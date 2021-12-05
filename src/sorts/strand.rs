use core::fmt;

use super::Arr;

pub fn strand_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy + Default + fmt::Debug,
{
  let len = array.len();
  let mut auxiliary = vec![Default::default(); len];

  strand_sort_inner(array, start, end, end - start + 1, &mut auxiliary)
}

fn strand_sort_inner<T>(
  array: Arr<T>,
  start: usize,
  end: usize,
  input_length: usize,
  new_output: &mut Vec<T>,
) where
  T: Ord + Copy + Default + fmt::Debug,
{
  let mut read_pointer = start + input_length;
  let mut new_output_pointer = 0;

  while read_pointer <= end && *array.index(read_pointer) < *array.index(0) {
    new_output[new_output_pointer] = *array.index(read_pointer);
    new_output_pointer += 1;
    read_pointer += 1;
  }

  new_output[new_output_pointer] = *array.index(0);
  let mut sublist_last_pointer = new_output_pointer;
  new_output_pointer += 1;

  let input_length = {
    let mut new_input_length = 0;
    for i in 1..input_length {
      let value = *array.index(start + i);

      if new_output[sublist_last_pointer] < value {
        while read_pointer <= end && *array.index(read_pointer) < value {
          new_output[new_output_pointer] = *array.index(read_pointer);
          new_output_pointer += 1;
          read_pointer += 1;
        }

        new_output[new_output_pointer] = value;
        sublist_last_pointer = new_output_pointer;
        new_output_pointer += 1;
      } else {
        *array.index_mut(start + new_input_length) = value;
        new_input_length += 1;
      }
    }

    new_input_length
  };

  for (i, &item) in new_output.iter().enumerate().take(new_output_pointer) {
    *array.index_mut(start + input_length + i) = item;
  }

  if input_length > 0 {
    strand_sort_inner(array, start, end, input_length, new_output)
  };
}
