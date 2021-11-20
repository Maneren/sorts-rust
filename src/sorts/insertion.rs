use super::Arr;

pub fn insertion_sort<T, const CAP: usize>(array: &mut Arr<T, CAP>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  for i in start..=end {
    for j in (start + 1..=i).rev() {
      if array[j] < array[j - 1] {
        array.swap(j, j - 1);
      } else {
        break;
      }
    }
  }
}
