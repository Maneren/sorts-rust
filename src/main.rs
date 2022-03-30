#![allow(clippy::pedantic)]
#![feature(int_log)]
#![feature(is_sorted)]
#![feature(mutex_unlock)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

mod array;
use array::ArrayWithCounters;

mod sorts;
use sorts::{get_sorts, run_sort, Sort};

mod gui;
use gui::run_gui;

use config::ITEM_COUNT;

pub type Item = usize;

mod config {
  use std::time::Duration;

  pub const WINDOW_SIZE: u32 = 900;

  pub const ITEM_COUNT: usize = 4096;

  pub const BASE_TIME: u64 = 100;
  pub const READ_TIME: Duration = Duration::from_micros(BASE_TIME);
  pub const WRITE_TIME: Duration = Duration::from_micros(2 * BASE_TIME);
  pub const SWAP_TIME: Duration = Duration::from_micros(2 * BASE_TIME);
}

fn main() {
  let nums: ArrayWithCounters = ArrayWithCounters::new((0..ITEM_COUNT).collect());
  let mutex = Arc::new(nums);
  let mutex_clone = mutex.clone();

  let done_flag = Arc::new(AtomicBool::new(false));
  let done_flag_clone = done_flag.clone();

  thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    run_sorts(mutex, done_flag_clone);
  });

  run_gui(mutex_clone, done_flag);
}

static ORDER: Ordering = Ordering::Relaxed;

fn run_sorts(nums: Arc<ArrayWithCounters>, done_flag: Arc<AtomicBool>) {
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

      let correct = true;
      for (a, b) in (0..nums.len()).zip(1..nums.len()) {
        if nums.get(a) > nums.get(b) {
          panic!("{}", a);
        }
      }
      if !correct {
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

  done_flag.store(true, Ordering::Relaxed);
}
