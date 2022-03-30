use super::Arr;

pub fn bubble_sort(array: Arr, start: usize, end: usize) {
  for i in ((start + 1)..=end).rev() {
    let mut swapped = false;
    for j in start..i {
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
