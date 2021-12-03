#![allow(clippy::pedantic)]
#![feature(int_log)]
#![feature(is_sorted)]

use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
  let mut nums = ArrayWithCounters::new((0..ITEM_COUNT).collect());
  let mut mutex = Arc::new(RwLock::new(nums));
  let mut mutex_clone = mutex.clone();

  let handle = thread::spawn(move || {
    let window = Window::new_centered("Sorts Animation", (800, 800)).unwrap();
    window.run_loop(SortsWindowHandler {
      start_time: Instant::now(),
      mutex: mutex_clone,
    })
  });

  run_sorts(nums);

  handle.join().unwrap();
}

struct SortsWindowHandler {
  start_time: Instant,
}

impl WindowHandler for SortsWindowHandler {
  fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
    graphics.clear_screen(Color::WHITE);

    let elapsed_secs = self.start_time.elapsed().as_secs_f32();

    let center = Vector2::new(400.0, 400.0);
    let offset = 200.0;

    let position = center + Vector2::new(elapsed_secs.cos() * offset, elapsed_secs.sin() * offset);

    graphics.draw_circle(position, 75.0, Color::from_rgb(0.8, 0.9, 1.0));

    // Request that we draw another frame once this one has finished
    helper.request_redraw();
  }
}

mod array;
use array::ArrayWithCounters;

#[macro_use]
mod sorts;
use sorts::{get_sorts, run_sort, Sort};

type Item = usize;

const ITEM_COUNT: Item = 50000;

fn run_sorts(nums: Arc<RwLock<ArrayWithCounters<usize>>>) {
  let sorts_dictionary = get_sorts();

  let mut rng = rand::thread_rng();

  macro_rules! check_sort {
    ( $sort: expr) => {{
      {
        let nums_guard = nums.write().unwrap();
        nums_guard.shuffle(&mut rng);
        nums_guard.reverse();
      }

      let start = Instant::now();
      run_sort(&sorts_dictionary, $sort, &mut nums);
      let time = start.elapsed();

      let nums = nums.read().unwrap();

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
