use super::Arr;

pub fn strand_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy + Default,
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
  T: Ord + Copy,
{
  if input_length == 0 {
    return;
  }

  let mut sublist = vec![array[start]];
  let mut input = Vec::from(&array[start + 1..start + input_length]);
  let output = Vec::from(&array[start + input_length..=end]);

  {
    let mut i = 0;
    while i < input.len() {
      if sublist.last().unwrap() < &input[i] {
        let value = input.remove(i);
        sublist.push(value);
      }
      i += 1;
    }
  }

  let output_length = strand_merge(output, sublist, aux);
  let output = Vec::from(&aux[..output_length]);

  for i in 0..input.len() {
    array[start + i] = input[i]
  }

  output.iter().enumerate().for_each(|(i, &item)| {
    let index = input.len() + i;
    array[start + index] = item;
  });

  strand_sort_inner(array, start, end, input.len(), aux);
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
