#![allow(clippy::pedantic)]
#![feature(int_log)]

use arrayvec::ArrayVec;
use rand::seq::SliceRandom;
use std::time::Instant;

mod array;
use array::ArrayWithCounters;

#[macro_use]
mod sorts;
use sorts::{get_sorts, run_sort, Sort};

const ITEM_COUNT: usize = 2000000;

fn main() {
  let sorts_dictionary = get_sorts();
  let mut nums =
    ArrayWithCounters::new(ArrayVec::<u32, ITEM_COUNT>::from_iter(0..ITEM_COUNT as u32));

  let mut rng = rand::thread_rng();

  let sorted = nums.clone();

  macro_rules! check_sort {
    ( $sort: expr) => {{
      nums.shuffle(&mut rng);
      // nums.reverse();

      let start = Instant::now();
      run_sort(&sorts_dictionary, $sort, &mut nums);
      let time = start.elapsed();

      println!("{:?}", $sort);
      println!("{}", nums.poll());

      if sorted == nums.data {
        println!("Correct")
      } else {
        println!("Incorrect")
      }

      println!("Elapsed: {:?}", time);

      println!();

      nums.reset();
    }};
  }

  /*  check_sort!(Sort::Bubble);
  check_sort!(Sort::Selection);
  check_sort!(Sort::Insertion);
  check_sort!(Sort::CoctailShaker); */
  check_sort!(Sort::Heap);
  check_sort!(Sort::InPlaceQuick);
  check_sort!(Sort::HoareQuick);
  // check_sort!(Sort::Intro); TODO: fix this
  check_sort!(Sort::Merge);
  check_sort!(Sort::Quick);
  // check_sort!(Sort::WeaveMerge);
  check_sort!(Sort::Tim);
}
