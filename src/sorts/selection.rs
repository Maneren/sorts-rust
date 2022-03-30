use super::Arr;

pub fn selection_sort(array: Arr, start: usize, end: usize) {
  for i in start..=end {
    let mut smallest_i = i;
    let mut smallest_val = array.get(smallest_i);

    for j in i..=end {
      let current = array.get(j);

      if current < smallest_val {
        smallest_i = j;
        smallest_val = current;
      }
    }
    array.swap(i, smallest_i);
  }
}
