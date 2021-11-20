use super::Arr;

pub fn selection_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  for i in start..=end {
    let mut smallest = i;
    for j in i..=end {
      if array[j] < array[smallest] {
        smallest = j;
      }
    }
    array.swap(i, smallest);
  }
}
