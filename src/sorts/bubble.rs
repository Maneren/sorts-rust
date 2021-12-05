use super::Arr;

pub fn bubble_sort<T>(array: &mut Arr<T>, start: usize, end: usize)
where
    T: Ord + Copy,
{
    for i in ((start + 1)..=end).rev() {
        let mut swapped = false;
        for j in start..i {
            let a = j;
            let b = j + 1;
            if array.read().unwrap()[a] > array.read().unwrap()[b] {
                array.write().unwrap().swap(a, b);
                swapped = true;
            }
        }
        if !swapped {
            break;
        };
    }
}
