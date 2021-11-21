use super::Arr;

pub fn gnome_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
  T: Ord + Copy,
{
  let mut index = start;
  while index < end {
    let (a, b) = (index, index + 1);
    if array[a] < array[b] {
      index += 1;
    } else {
      array.swap(a, b);

      if index != 0 {
        index -= 1
      }
    }
  }
}
