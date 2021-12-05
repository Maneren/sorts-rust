use super::Arr;

pub fn selection_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  for i in start..=end {
    let mut smallest = i;
    for j in i..=end {
      if *array.index(j) < *array.index(smallest) {
        smallest = j;
      }
    }
    array.swap(i, smallest);
  }
}
