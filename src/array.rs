use std::{
  cell::{Cell, UnsafeCell},
  fmt::{self, Debug, Display},
  marker::PhantomData,
  ops::{Deref, DerefMut, Index, IndexMut},
  sync::Mutex,
  thread,
  time::Duration,
};

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

  pub fn reset(&self) {
    self.reads.replace(0);
    self.writes.replace(0);
    self.swaps.replace(0);
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

#[derive(Debug)]
pub enum ArrayGuard<T: ?Sized + Index<usize> + IndexMut<usize>> {
  Read((*mut T, usize)),
  Write((*mut T, usize)),
}

impl<'a, T: ?Sized + Index<usize> + IndexMut<usize>> Deref for ArrayGuard<T> {
  type Target = <T as Index<usize>>::Output;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Read((pointer, index)) => unsafe { &(**pointer)[*index] },
      Self::Write((pointer, index)) => unsafe { &mut (**pointer)[*index] },
    }
  }
}
impl<T: ?Sized + Index<usize> + IndexMut<usize>> DerefMut for ArrayGuard<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      Self::Read(_) => panic!("Read-only"),
      Self::Write((pointer, index)) => unsafe { &mut (**pointer)[*index] },
    }
  }
}

pub struct ArrayWithCounters<'a, T: 'a> {
  pub data: Mutex<UnsafeCell<Vec<T>>>,
  stats: Mutex<Stats>,
  _marker: &'a PhantomData<T>,
}

unsafe impl<'a, T: 'a> Sync for ArrayWithCounters<'a, T> {}

impl<'a, T: 'a> ArrayWithCounters<'a, T> {
  pub fn new(vec: Vec<T>) -> Self {
    ArrayWithCounters::<T> {
      data: Mutex::new(UnsafeCell::new(vec)),
      stats: Mutex::new(Stats::new()),
      _marker: &PhantomData,
    }
  }

  pub fn swap(&self, a: usize, b: usize)
  where
    T: Copy,
  {
    if a == b {
      return;
    }
    let stats = self.stats.lock().unwrap();
    stats.read(2);
    stats.write(2);
    stats.swap(1);

    thread::sleep(Duration::from_micros(SWAP_TIME));

    let pointer = self.data.lock().unwrap().get();

    unsafe { (*pointer).swap(a, b) };
  }

  pub fn poll(&self) -> Stats {
    self.stats.lock().unwrap().clone()
  }

  pub fn reset(&self) {
    self.stats.lock().unwrap().reset();
  }

  pub fn index(&self, index: usize) -> ArrayGuard<Vec<T>> {
    self.stats.lock().unwrap().read(1);

    thread::sleep(Duration::from_micros(READ_TIME));

    let pointer = self.data.lock().unwrap().get();

    ArrayGuard::Read((pointer, index))
  }

  pub fn index_mut(&self, index: usize) -> ArrayGuard<Vec<T>> {
    self.stats.lock().unwrap().write(1);

    thread::sleep(Duration::from_micros(WRITE_TIME));

    let pointer = self.data.lock().unwrap().get();

    ArrayGuard::Write((pointer, index))
  }

  #[allow(clippy::mut_from_ref)]
  pub fn deref_mut(&self) -> &mut Vec<T> {
    unsafe { &mut *self.data.lock().unwrap().get() }
  }
}

impl<T> Deref for ArrayWithCounters<'_, T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.data.lock().unwrap().get() }
  }
}
