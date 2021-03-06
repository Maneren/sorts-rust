use super::Arr;

pub fn heap_sort(array: Arr, start: usize, end: usize) {
  heapify(array.clone(), start, end);

  for i in (1..end - start + 1).rev() {
    array.swap(start, start + i);
    sift_down(array.clone(), start, start + i - 1, 0);
  }
}

fn get_left_child_index(parent: usize) -> usize {
  2 * parent + 1
}

fn get_right_child_index(parent: usize) -> usize {
  2 * parent + 2
}

fn sift_down(array: Arr, start: usize, end: usize, index: usize) {
  let current_i = start + index;
  let current = array.get(current_i);

  let right_i = start + get_right_child_index(index);
  let left_i = start + get_left_child_index(index);

  let right = if right_i <= end {
    Some(array.get(right_i))
  } else {
    None
  };
  let left = if left_i <= end {
    Some(array.get(left_i))
  } else {
    None
  };

  match (left, right) {
    (Some(left), Some(right)) => {
      if left < current && right < current {
        return; // tree is correct, no need for change
      };

      let larger_i = if left > right { left_i } else { right_i };
      array.swap(current_i, larger_i);
      sift_down(array, start, end, larger_i - start);
    }
    (Some(left), None) => {
      if current < left {
        array.swap(current_i, left_i);
        sift_down(array, start, end, left_i - start);
      }
    }
    (None, Some(right)) => {
      if current < right {
        array.swap(current_i, right_i);
        sift_down(array, start, end, right_i - start);
      }
    }
    _ => {}
  }
}

fn heapify(array: Arr, start: usize, end: usize) {
  for i in (0..=((end - start) / 2)).rev() {
    sift_down(array.clone(), start, end, i);
  }
}
