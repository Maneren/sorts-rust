use super::Arr;

pub fn insertion_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  for i in start..=end {
    for j in (start + 1..=i).rev() {
      if *array.index(j) < *array.index(j - 1) {
        array.swap(j, j - 1);
      } else {
        break;
      }
    }
  }
}
