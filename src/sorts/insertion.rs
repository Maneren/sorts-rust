use super::Arr;

pub fn insertion_sort<T>(array: Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  for i in start + 1..=end {
    let mut previous = *array[i];
    for j in (start + 1..=i).rev() {
      let current = *array[j - 1];

      if current > previous {
        array.swap(j, j - 1);
        previous = *array[j - 1];
      } else {
        break;
      }
    }
  }
}
