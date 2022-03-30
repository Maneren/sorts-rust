use super::Arr;

pub fn coctail_shaker_sort(array: Arr, start: usize, end: usize) {
  for i in start..=end {
    let mut swapped = false;
    for j in (i + 1..=end).rev() {
      let a = j;
      let b = j - 1;
      if array.get(a) < array.get(b) {
        array.swap(a, b);
        swapped = true;
      }
    }
    for j in i..end {
      let a = j;
      let b = j + 1;
      if array.get(a) > array.get(b) {
        array.swap(a, b);
        swapped = true;
      }
    }
    if !swapped {
      break;
    };
  }
}
