use std::{
  cell::{Cell, Ref, RefCell},
  fmt::{self, Display},
  ops::{Deref, DerefMut, Index, IndexMut, Range, RangeInclusive},
};

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

  fn read(&self, n: u64) {
    self.reads.replace(self.reads.get() + n);
  }

  fn write(&self, n: u64) {
    self.writes.replace(self.writes.get() + n);
  }

  fn swap(&self, n: u64) {
    self.swaps.replace(self.swaps.get() + n);
  }
}

pub struct ArrayWithCounters<T> {
  pub data: Vec<T>,
  stats: RefCell<Stats>,
}

impl<T> ArrayWithCounters<T> {
  pub fn new(vec: Vec<T>) -> Self {
    ArrayWithCounters::<T> {
      data: vec,
      stats: RefCell::new(Stats::new()),
    }
  }

  pub fn swap(&mut self, a: usize, b: usize)
  where
    T: Copy,
  {
    if a == b {
      return;
    }

    self.stats.borrow().read(2);
    self.stats.borrow().write(2);
    self.stats.borrow().swap(1);
    self.data.swap(a, b);
  }

  pub fn poll(&self) -> RefCell<Stats> {
    self.stats.clone()
  }

  pub fn reset(&mut self) {
    self.stats = Stats::new();
  }
}
impl<T> Index<usize> for ArrayWithCounters<T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    self.stats.read(1);
    &self.data[index]
  }
}

impl<T> Index<Range<usize>> for ArrayWithCounters<T> {
  type Output = [T];

  fn index(&self, index: Range<usize>) -> &Self::Output {
    self.stats.read((index.end - index.start) as u64);
    &self.data[index]
  }
}

impl<T> Index<RangeInclusive<usize>> for ArrayWithCounters<T> {
  type Output = [T];

  fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
    self.stats.read((index.end() + 1 - index.start()) as u64);
    &self.data[index]
  }
}

impl<T> IndexMut<usize> for ArrayWithCounters<T> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.stats.write(1);
    &mut self.data[index]
  }
}
impl<T> Deref for ArrayWithCounters<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}
impl<T> DerefMut for ArrayWithCounters<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}
