mod bubble;
mod coctail_shaker;
mod counting;
mod gnome;
mod heap;
mod hoare_quick;
mod in_place_merge;
mod in_place_quick;
mod insertion;
mod intro;
mod merge;
mod quick;
mod selection;
mod tim;
mod weave_merge;

mod common;
mod strand;
use common::*;

use self::{
  bubble::*, coctail_shaker::*, counting::*, gnome::*, heap::*, hoare_quick::*, in_place_merge::*,
  in_place_quick::*, insertion::*, intro::*, merge::*, quick::*, selection::*, strand::*, tim::*,
  weave_merge::*,
};

use super::{array::ArrayWithCounters, Item};
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

pub type Arr<T> = Arc<Mutex<ArrayWithCounters<T>>>;

type SortFunction = fn(&mut Arr<Item>, usize, usize);
type SortsDictionary = HashMap<Sort, Box<SortFunction>>;

macro_rules! sorts_map {
  {$($k: expr => $v: expr),* $(,)?} => {{
    let mut map = HashMap::new();
    $( map.insert($k, Box::new($v as SortFunction)); )*
    map
  }}
}

pub fn run_sort(dictionary: &SortsDictionary, sort: Sort, array: &mut Arr<Item>) {
  dictionary.get(&sort).unwrap()(array, 0, array.lock().unwrap().len() - 1)
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Sort {
  Bubble,
  CoctailShaker,
  Counting,
  Gnome,
  Heap,
  HoareQuick,
  InPlaceQuick,
  InPlaceMerge,
  Intro,
  Insertion,
  Merge,
  Quick,
  Selection,
  Strand,
  Tim,
  WeaveMerge,
}

pub fn get_sorts() -> SortsDictionary {
  sorts_map! {
    Sort::Bubble => bubble_sort,
    Sort::CoctailShaker => coctail_shaker_sort,
    Sort::Counting => counting_sort,
    Sort::Gnome => gnome_sort,
    Sort::Heap => heap_sort,
    Sort::HoareQuick => hoare_quick_sort,
    Sort::InPlaceQuick => in_place_quick_sort,
    Sort::InPlaceMerge => in_place_merge_sort,
    Sort::Intro => intro_sort,
    Sort::Insertion => insertion_sort,
    Sort::Merge => merge_sort,
    Sort::Quick => quick_sort,
    Sort::Selection => selection_sort,
    Sort::Strand => strand_sort,
    Sort::Tim => tim_sort,
    Sort::WeaveMerge => weave_merge_sort,
  }
}
