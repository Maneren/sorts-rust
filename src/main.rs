#![allow(clippy::pedantic)]
#![feature(int_log)]
#![feature(is_sorted)]

use rand::seq::SliceRandom;
use std::time::Instant;

mod array;
use array::ArrayWithCounters;

#[macro_use]
mod sorts;
use sorts::{get_sorts, run_sort, Sort};

type Item = usize;

const ITEM_COUNT: Item = 50000;

fn main() {
  let sorts_dictionary = get_sorts();

  let mut nums = ArrayWithCounters::new((0..ITEM_COUNT).collect());

  let mut rng = rand::thread_rng();

  macro_rules! check_sort {
    ( $sort: expr) => {{
      nums.shuffle(&mut rng);
      nums.reverse();

      let start = Instant::now();
      run_sort(&sorts_dictionary, $sort, &mut nums);
      let time = start.elapsed();

      println!("{:?} - {:?}", $sort, time);
      println!("{}\n", nums.poll());

      if !nums.data.is_sorted() {
        for (a, b) in (0..nums.len()).zip(1..nums.len()) {
          if nums[a] > nums[b] {
            panic!("{}: {:?}", a, &(*nums)[a..=a + 2])
          }
        }
        panic!("Incorrect!!!");
      }

      nums.reset();
    }};
  }

  check_sort!(Sort::Bubble);
  check_sort!(Sort::CoctailShaker);
  check_sort!(Sort::Selection);
  check_sort!(Sort::Gnome);
  check_sort!(Sort::Insertion);
  check_sort!(Sort::Strand);
  check_sort!(Sort::Heap);
  check_sort!(Sort::Quick);
  check_sort!(Sort::InPlaceQuick);
  check_sort!(Sort::HoareQuick);
  check_sort!(Sort::Intro);
  check_sort!(Sort::Merge);
  check_sort!(Sort::Tim);
  check_sort!(Sort::InPlaceMerge);
  check_sort!(Sort::WeaveMerge);
  check_sort!(Sort::Counting);
}
