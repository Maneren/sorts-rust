#![allow(clippy::pedantic)]
#![feature(int_log)]
#![feature(is_sorted)]
#![feature(type_alias_impl_trait)]

use rand::seq::SliceRandom;
use speedy2d::shape::Rectangle;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use config::ITEM_COUNT;

fn main() {
  let nums = ArrayWithCounters::new((0..ITEM_COUNT).collect());
  let mutex = Arc::new(nums);
  let mutex_clone = mutex.clone();

  thread::spawn(move || {
    thread::sleep(Duration::from_secs(1));
    run_sorts(mutex);
  });

  let window = Window::new_centered("Sorts Animation", (800, 800)).unwrap();
  window.run_loop(SortsWindowHandler { array: mutex_clone });
}

struct SortsWindowHandler<'a, T> {
  array: Arc<ArrayWithCounters<'a, T>>,
}

impl<'a> WindowHandler for SortsWindowHandler<'a, usize> {
  fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
    graphics.clear_screen(Color::BLACK);

    for (i, &item) in self.array.iter().enumerate() {
      let item = item as f32 + 1_f32;
      let i = i as f32;
      let count = ITEM_COUNT as f32;

      let size = 800_f32;
      let top_left = Vector2::new(size / count * i, size - (item / count * size));
      let bottom_right = Vector2::new(size / count * (i + 1_f32), size);

      graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), Color::WHITE)
    }

    helper.request_redraw();
  }
}

mod array;
use array::ArrayWithCounters;

#[macro_use]
mod sorts;
use sorts::{get_sorts, run_sort, Sort};

type Item = usize;

mod config {
  pub const ITEM_COUNT: usize = 8192;

  pub const BASE_TIME: u64 = 10;
  pub const READ_TIME: u64 = BASE_TIME;
  pub const WRITE_TIME: u64 = 2 * BASE_TIME;
  pub const SWAP_TIME: u64 = 4 * BASE_TIME;
}

fn run_sorts(nums: Arc<ArrayWithCounters<usize>>) {
  let sorts_dictionary = get_sorts();

  let mut rng = rand::thread_rng();

  println!("{:?}", nums.index_mut(10));

  macro_rules! check_sort {
    ( $sort: expr) => {{
      {
        nums.deref_mut().shuffle(&mut rng);
        nums.deref_mut().reverse();
      }

      let start = Instant::now();
      run_sort(&sorts_dictionary, $sort, nums.clone());
      let time = start.elapsed();

      println!("{:?} - {:?}", $sort, time);
      println!("{}\n", nums.poll());

      if !nums.is_sorted() {
        for (a, b) in (0..nums.len()).zip(1..nums.len()) {
          if nums[a] > nums[b] {
            panic!("{}", a);
          }
        }
        panic!("Incorrect!!!");
      }

      nums.reset();
    }};
  }

  // check_sort!(Sort::Bubble);
  // check_sort!(Sort::CoctailShaker);
  // check_sort!(Sort::Selection);
  // check_sort!(Sort::Gnome);
  // check_sort!(Sort::Insertion);
  check_sort!(Sort::Strand);
  check_sort!(Sort::Heap);
  check_sort!(Sort::Quick);
  check_sort!(Sort::InPlaceQuick);
  check_sort!(Sort::HoareQuick);
  check_sort!(Sort::Intro);
  check_sort!(Sort::Merge);
  check_sort!(Sort::Tim);
  // check_sort!(Sort::InPlaceMerge);
  // check_sort!(Sort::WeaveMerge);
  check_sort!(Sort::Counting);
}
