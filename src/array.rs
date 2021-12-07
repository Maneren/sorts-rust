use std::{
  cell::{Cell, UnsafeCell},
  fmt::{self, Debug, Display},
  marker::PhantomData,
  ops::{Deref, DerefMut, Index, IndexMut},
  sync::Mutex,
  thread,
};

use speedy2d::color::Color;

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

#[derive(Debug, Copy)]
pub enum ArrayGuard<T: ?Sized + Index<usize> + IndexMut<usize>> {
  Read((*mut T, usize)),
  Write((*mut T, usize)),
}

impl<T: ?Sized + Index<usize> + IndexMut<usize>> Clone for ArrayGuard<T> {
  fn clone(&self) -> Self {
    match self {
      Self::Read((pointer, index)) => Self::Read((*pointer, *index)),
      Self::Write((pointer, index)) => Self::Write((*pointer, *index)),
    }
  }
}
impl<T: ?Sized + Index<usize> + IndexMut<usize>> Deref for ArrayGuard<T> {
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
unsafe impl<T: ?Sized + Index<usize> + IndexMut<usize>> Sync for ArrayGuard<T> {}
unsafe impl<T: ?Sized + Index<usize> + IndexMut<usize>> Send for ArrayGuard<T> {}

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

pub struct ArrayWithCounters<'a, T: 'a> {
  pub data: Mutex<UnsafeCell<Vec<T>>>,
  guard: UnsafeCell<ArrayGuard<Vec<T>>>,
  stats: Stats,
  pub highlighted: Mutex<UnsafeCell<Vec<Highlight>>>,
  phantom: PhantomData<&'a T>,
}

unsafe impl<'a, T> Sync for ArrayWithCounters<'a, T> {}

impl<'a, T> ArrayWithCounters<'a, T> {
  pub fn new(vec: Vec<T>) -> Self {
    let data = Mutex::new(UnsafeCell::new(vec));
    let guard = UnsafeCell::new(ArrayGuard::Read((data.lock().unwrap().get(), 0)));
    ArrayWithCounters {
      data,
      guard,
      stats: Stats::new(),
      highlighted: Mutex::new(UnsafeCell::new(Vec::with_capacity(2))),
      phantom: PhantomData,
    }
  }

  pub fn swap(&self, a: usize, b: usize)
  where
    T: Copy,
  {
    if a == b {
      return;
    }

    self.stats.read(2);
    self.stats.write(2);
    self.stats.swap(1);

    let mut highlighted = self.highlighted.lock().unwrap();
    highlighted.get_mut().clear();
    highlighted.get_mut().push(Highlight::swap(a));
    highlighted.get_mut().push(Highlight::swap(b));
    Mutex::unlock(highlighted);

    thread::sleep(SWAP_TIME);

    self.data.lock().unwrap().get_mut().swap(a, b);
  }

  pub fn poll(&self) -> Stats {
    self.stats.clone()
  }

  pub fn reset(&self) {
    self.stats.reset();
  }

  pub fn index_mut(&self, index: usize) -> ArrayGuard<Vec<T>> {
    self.stats.write(1);

    let mut highlighted = self.highlighted.lock().unwrap();
    highlighted.get_mut().clear();
    highlighted.get_mut().push(Highlight::write(index));
    Mutex::unlock(highlighted);

    thread::sleep(WRITE_TIME);

    let pointer = self.data.lock().unwrap().get();
    ArrayGuard::Write((pointer, index))
  }

  pub fn highlights(&self) -> &Vec<Highlight> {
    unsafe { &*self.highlighted.lock().unwrap().get() }
  }

  #[allow(clippy::mut_from_ref)]
  pub fn deref_mut(&self) -> &mut Vec<T> {
    unsafe { &mut *self.data.lock().unwrap().get() }
  }
}
impl<'a, T> Index<usize> for ArrayWithCounters<'a, T> {
  type Output = ArrayGuard<Vec<T>>;

  fn index(&self, index: usize) -> &Self::Output {
    self.stats.read(1);

    let mut highlighted = self.highlighted.lock().unwrap();
    highlighted.get_mut().clear();
    highlighted.get_mut().push(Highlight::read(index));
    Mutex::unlock(highlighted);

    thread::sleep(READ_TIME);

    unsafe {
      *self.guard.get() = ArrayGuard::Read((self.data.lock().unwrap().get(), index));
    }

    unsafe { &(*self.guard.get()) }
  }
}

impl<'a, T> IndexMut<usize> for ArrayWithCounters<'a, T> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.stats.read(1);

    let mut highlighted = self.highlighted.lock().unwrap();
    highlighted.get_mut().clear();
    highlighted.get_mut().push(Highlight::write(index));
    Mutex::unlock(highlighted);

    thread::sleep(WRITE_TIME);

    unsafe {
      *self.guard.get() = ArrayGuard::Write((self.data.lock().unwrap().get(), index));
    }

    unsafe { &mut (*self.guard.get()) }
  }
}

impl<'a, T> Deref for ArrayWithCounters<'a, T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.data.lock().unwrap().get() }
  }
}
