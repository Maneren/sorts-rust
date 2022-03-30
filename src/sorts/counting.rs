use super::Arr;

pub fn counting_sort(array: Arr, start: usize, end: usize) {
  let n = end - start + 1;
  // The output character array that will have sorted arr
  let mut output = vec![Default::default(); n];
  let mut count = vec![0; n];

  // store count of each character
  for i in 0..n {
    let index = array.get(start + i);
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
    let el = array.get(i);
    output[count[el] - 1] = el;
    count[el] -= 1;
  }

  // Copy the output array to arr, so that arr now
  // contains sorted characters
  for (i, &item) in output.iter().enumerate().take(n) {
    array.set(start + i, item);
  }
}
