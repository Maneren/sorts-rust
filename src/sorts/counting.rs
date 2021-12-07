use super::Arr;

pub fn counting_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy + Default + Into<usize>,
{
  let n = end - start + 1;
  // The output character array that will have sorted arr
  let mut output = vec![Default::default(); n];
  let mut count = vec![0; n];

  // store count of each character
  for i in 0..n {
    let index = (*array[start + i]).into();
    count[index] += 1
  }

  // Change count[i] so that count[i] now contains actual
  // position of this character in output array
  for i in 1..n {
    count[i] += count[i - 1]
  }

  // Build the output character array
  // To make it stable we are operating in reverse order.
  for i in (0..=n - 1).rev() {
    let el = *array[i];
    output[count[el.into()] - 1] = el;
    count[el.into()] -= 1;
  }

  // Copy the output array to arr, so that arr now
  // contains sorted characters
  for (i, &item) in output.iter().enumerate().take(n) {
    *array.index_mut(start + i) = item;
  }
}
