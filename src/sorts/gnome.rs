use super::Arr;

pub fn gnome_sort(array: Arr, start: usize, end: usize) {
  let mut index = start;
  while index < end {
    let (a, b) = (index, index + 1);
    if array.get(a) < array.get(b) {
      index += 1;
    } else {
      array.swap(a, b);

      index = index.saturating_sub(1);
    }
  }
}
