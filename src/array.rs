use std::{
  fmt::{self, Debug, Display},
  sync::{
    atomic::{AtomicU64, AtomicUsize},
    Mutex,
  },
  thread,
};

use rand::prelude::{SliceRandom, ThreadRng};
use speedy2d::color::Color;

use crate::ORDER;

use super::config::{READ_TIME, SWAP_TIME, WRITE_TIME};

#[allow(
  clippy::cast_precision_loss,
  clippy::cast_possible_truncation,
  clippy::cast_sign_loss
)]
fn format_number(input: f32) -> String {
  let sizes = ['-', 'k', 'M', 'G', 'T'];
  let base = 1000.0;

  if input == 0.0 {
    return "0".to_string();
  }

  let i = input.log(base).floor();
  let number = input / base.powi(i as i32);

  let string = format!("{:.2}", number)
    .trim_end_matches('0')
    .trim_end_matches('.')
    .to_owned();

  if i >= 1.0 {
    format!("{}{}", string, sizes[i as usize])
  } else {
    string
  }
}

#[derive(Debug)]
pub struct Stats {
  reads: AtomicU64,
  writes: AtomicU64,
  swaps: AtomicU64,
}
impl Stats {
  pub fn new() -> Self {
    Self {
      reads: AtomicU64::new(0),
      writes: AtomicU64::new(0),
      swaps: AtomicU64::new(0),
    }
  }

  pub fn reset(&self) {
    self.reads.store(0, ORDER);
    self.writes.store(0, ORDER);
    self.swaps.store(0, ORDER);
  }

  fn read(&self, n: u64) {
    self.reads.fetch_add(n, ORDER);
  }

  fn write(&self, n: u64) {
    self.writes.fetch_add(n, ORDER);
  }

  fn swap(&self, n: u64) {
    self.swaps.fetch_add(n, ORDER);
  }
}
impl Display for Stats {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "reads: {}, writes: {}, swaps: {}",
      format_number(self.reads.load(ORDER) as f32),
      format_number(self.reads.load(ORDER) as f32),
      format_number(self.reads.load(ORDER) as f32)
    )
  }
}
impl Clone for Stats {
  fn clone(&self) -> Self {
    Self {
      reads: AtomicU64::new(self.reads.load(ORDER)),
      writes: AtomicU64::new(self.writes.load(ORDER)),
      swaps: AtomicU64::new(self.swaps.load(ORDER)),
    }
  }
}

enum HighlightType {
  Read,
  Write,
  Swap,
}

#[derive(Copy, Clone, Debug)]
pub struct Highlight(pub usize, pub Color);

impl Highlight {
  fn new(index: usize, type_: HighlightType) -> Self {
    let color = match type_ {
      HighlightType::Read => Color::GREEN,
      HighlightType::Write => Color::RED,
      HighlightType::Swap => Color::BLUE,
    };
    Highlight(index, color)
  }

  pub fn swap(index: usize) -> Self {
    Self::new(index, HighlightType::Swap)
  }

  pub fn read(index: usize) -> Self {
    Self::new(index, HighlightType::Read)
  }

  pub fn write(index: usize) -> Self {
    Self::new(index, HighlightType::Write)
  }
}

#[derive(Debug)]
pub struct ArrayWithCounters {
  pub data: Mutex<Vec<AtomicUsize>>,
  stats: Stats,
  pub highlighted: Mutex<Vec<Highlight>>,
}

impl ArrayWithCounters {
  pub fn new(vec: Vec<usize>) -> Self {
    let vec = vec.into_iter().map(AtomicUsize::new).collect();
    ArrayWithCounters {
      data: Mutex::new(vec),
      stats: Stats::new(),
      highlighted: Mutex::new(Vec::with_capacity(2)),
    }
  }

  pub fn swap(&self, a: usize, b: usize) {
    if a == b {
      return;
    }

    self.stats.read(2);
    self.stats.write(2);
    self.stats.swap(1);

    let mut highlighted = self.highlighted.lock().unwrap();
    highlighted.clear();
    highlighted.push(Highlight::swap(a));
    highlighted.push(Highlight::swap(b));
    Mutex::unlock(highlighted);

    thread::sleep(SWAP_TIME);

    self.data.lock().unwrap().swap(a, b);
  }

  pub fn poll(&self) -> &Stats {
    &self.stats
  }

  pub fn reset(&self) {
    self.stats.reset();
  }

  pub fn set(&self, index: usize, value: usize) {
    self.stats.write(1);

    let mut highlighted = self.highlighted.lock().unwrap();
    highlighted.clear();
    highlighted.push(Highlight::write(index));
    Mutex::unlock(highlighted);

    thread::sleep(WRITE_TIME);

    self.data.lock().unwrap()[index].store(value, ORDER);
  }

  pub fn get(&self, index: usize) -> usize {
    self.stats.read(1);

    let mut highlighted = self.highlighted.lock().unwrap();
    highlighted.clear();
    highlighted.push(Highlight::read(index));
    Mutex::unlock(highlighted);

    thread::sleep(READ_TIME);

    let data = &*self.data.lock().unwrap();

    data[index].load(ORDER)
  }

  pub fn to_usize_vec(&self) -> Vec<usize> {
    let lock = self.data.lock().unwrap();
    let vec = lock.iter().map(|x| x.load(ORDER)).collect();

    vec
  }

  pub fn shuffle(&self, thread_rng: &mut ThreadRng) {
    let lock = &mut *self.data.lock().unwrap();

    lock.shuffle(thread_rng);
  }

  pub fn len(&self) -> usize {
    self.data.lock().unwrap().len()
  }

  pub fn highlights(&self) -> Vec<Highlight> {
    (*self.highlighted.lock().unwrap()).clone()
  }
}
