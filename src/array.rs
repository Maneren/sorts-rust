use std::{
  cell::Cell,
  fmt::{self, Display},
  ops::{Deref, DerefMut, Index, IndexMut},
};

use arrayvec::ArrayVec;

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

#[derive(Debug, Clone)]
pub struct Stats {
  reads: Cell<u64>,
  writes: Cell<u64>,
  swaps: Cell<u64>,
}

impl Display for Stats {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "reads: {}, writes: {}, swaps: {}",
      format_number(self.reads.get() as f32),
      format_number(self.writes.get() as f32),
      format_number(self.swaps.get() as f32)
    )
  }
}

impl Stats {
  pub fn new() -> Self {
    Self {
      reads: Cell::new(0),
      writes: Cell::new(0),
      swaps: Cell::new(0),
    }
  }

  fn read(&self) {
    self.reads.replace(self.reads.get() + 1);
  }

  fn write(&mut self) {
    self.writes.replace(self.writes.get() + 1);
  }

  fn swap(&mut self) {
    self.swaps.replace(self.swaps.get() + 1);
  }
}

pub type Arr<T, const CAP: usize> = ArrayWithCounters<T, CAP>;

pub struct ArrayWithCounters<T, const CAP: usize> {
  pub data: ArrayVec<T, CAP>,
  stats: Stats,
}

impl<T, const CAP: usize> ArrayWithCounters<T, CAP> {
  pub fn new(arr: ArrayVec<T, CAP>) -> Self {
    ArrayWithCounters::<T, CAP> {
      data: arr,
      stats: Stats::new(),
    }
  }

  pub fn swap(&mut self, a: usize, b: usize)
  where
    T: Copy,
  {
    if a == b {
      return;
    }

    self.stats.read();
    self.stats.read();
    self.stats.write();
    self.stats.write();
    self.stats.swap();
    self.data.swap(a, b);
  }

  pub fn poll(&self) -> Stats {
    self.stats.clone()
  }

  pub fn reset(&mut self) {
    self.stats = Stats::new();
  }
}
impl<T, const CAP: usize> Index<usize> for ArrayWithCounters<T, CAP> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    self.stats.read();
    &self.data[index]
  }
}

impl<T, const CAP: usize> IndexMut<usize> for ArrayWithCounters<T, CAP> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.stats.write();
    &mut self.data[index]
  }
}
impl<T, const CAP: usize> Deref for ArrayWithCounters<T, CAP> {
  type Target = ArrayVec<T, CAP>;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}
impl<T, const CAP: usize> DerefMut for ArrayWithCounters<T, CAP> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}
