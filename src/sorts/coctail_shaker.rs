use super::Arr;

pub fn coctail_shaker_sort<T, const CAP: usize>(array: &mut Arr<T, CAP>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  for i in start..=end {
    let mut swapped = false;
    for j in (i + 1..=end).rev() {
      let a = j;
      let b = j - 1;
      if array[a] < array[b] {
        array.swap(a, b);
        swapped = true;
      }
    }
    for j in i..end {
      let a = j;
      let b = j + 1;
      if array[a] > array[b] {
        array.swap(a, b);
        swapped = true;
      }
    }
    if !swapped {
      break;
    };
  }
}
