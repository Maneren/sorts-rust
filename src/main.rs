#![allow(clippy::pedantic)]
#![feature(is_sorted)]
#![feature(mutex_unlock)]

use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
  time::{Duration, Instant},
};

mod array;
use array::ArrayWithCounters;

mod sorts;
use sorts::{get_sorts, run_sort, Sort};

mod gui;
use config::ITEM_COUNT;
use gui::run_gui;

pub type Item = usize;

mod config {
  use std::time::Duration;

  pub const WINDOW_WIDTH: u32 = 1600;
  pub const WINDOW_HEIGHT: u32 = 800;

  pub const ITEM_COUNT: usize = 8192;

  pub const BASE_TIME: u64 = 1;
  pub const READ_TIME: Duration = Duration::from_nanos(BASE_TIME);
  pub const WRITE_TIME: Duration = Duration::from_nanos(2 * BASE_TIME);
  pub const SWAP_TIME: Duration = Duration::from_nanos(5 * BASE_TIME);
}

static DONE_FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
  let nums: ArrayWithCounters = ArrayWithCounters::new((0..ITEM_COUNT).collect());
  let arc = Arc::new(nums);
  let arc_clone = arc.clone();

  thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    run_sorts(arc_clone);
  });

  run_gui(arc);
}

static ORDER: Ordering = Ordering::Relaxed;

fn run_sorts(nums: Arc<ArrayWithCounters>) {
  let sorts_dictionary = get_sorts();

  let mut rng = rand::thread_rng();

  macro_rules! check_sort {
    ( $sort: expr) => {{
      nums.shuffle(&mut rng);

      let start = Instant::now();
      run_sort(&sorts_dictionary, $sort, nums.clone());
      let time = start.elapsed();

      println!("{:?} - {:?}", $sort, time);
      println!("{}\n", nums.poll());

      for (a, b) in (0..nums.len()).zip(1..nums.len()) {
        if nums.get(a) > nums.get(b) {
          panic!("Incorrect: {a} is not <= {b}");
        }
      }

      nums.reset();
    }};
  }

  /* check_sort!(Sort::Bubble);
  check_sort!(Sort::CoctailShaker);
  check_sort!(Sort::Selection);
  check_sort!(Sort::Gnome);
  check_sort!(Sort::Insertion);
  check_sort!(Sort::Strand); */
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

  DONE_FLAG.store(true, Ordering::Relaxed);
}
