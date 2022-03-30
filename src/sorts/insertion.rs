use super::Arr;

pub fn insertion_sort(array: Arr, start: usize, end: usize) {
  for i in start + 1..=end {
    let mut previous = array.get(i);
    for j in (start + 1..=i).rev() {
      let current = array.get(j - 1);

      if current > previous {
        array.swap(j, j - 1);
        previous = array.get(j - 1);
      } else {
        break;
      }
    }
  }
}
