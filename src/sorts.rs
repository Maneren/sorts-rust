mod common;

use std::collections::HashMap;

use super::array::{Arr, ArrayWithCounters};
use super::ITEM_COUNT;

type Item = u32;
type SortFunction<const CAP: usize> = fn(&mut ArrayWithCounters<Item, CAP>, usize, usize);
type SortsDictionary = HashMap<Sort, Box<SortFunction<ITEM_COUNT>>>;

macro_rules! sorts_map {
  {$($k: expr => $v: expr),* $(,)?} => {{
    let mut map = HashMap::new();
    $( map.insert($k, Box::new($v as SortFunction<ITEM_COUNT>)); )*
    map
  }}
}

pub fn run_sort(
  dictionary: &SortsDictionary,
  sort: Sort,
  array: &mut ArrayWithCounters<Item, ITEM_COUNT>,
) {
  dictionary.get(&sort).unwrap()(array, 0, array.len() - 1)
}

mod bubble;
mod coctail_shaker;
mod heap;
mod hoare_quick;
mod in_place_quick;
mod insertion;
mod intro;
mod merge;
mod quick;
mod selection;
mod tim;
mod weave_merge;

pub use self::{
  bubble::bubble_sort, coctail_shaker::coctail_shaker_sort, heap::heap_sort,
  hoare_quick::hoare_quick_sort, in_place_quick::in_place_quick_sort, insertion::insertion_sort,
  intro::intro_sort, merge::merge_sort, quick::quick_sort, selection::selection_sort,
  tim::tim_sort, weave_merge::weave_merge_sort,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Sort {
  Bubble,
  CoctailShaker,
  Heap,
  HoareQuick,
  InPlaceQuick,
  Intro,
  Insertion,
  Merge,
  Quick,
  Selection,
  Tim,
  WeaveMerge,
}

pub fn get_sorts() -> SortsDictionary {
  sorts_map! {
    Sort::Bubble => bubble_sort,
    Sort::CoctailShaker => coctail_shaker_sort,
    Sort::Heap => heap_sort,
    Sort::HoareQuick => hoare_quick_sort,
    Sort::InPlaceQuick => in_place_quick_sort,
    Sort::Intro => intro_sort,
    Sort::Insertion => insertion_sort,
    Sort::Merge => merge_sort,
    Sort::Quick => quick_sort,
    Sort::Selection => selection_sort,
    Sort::Tim => tim_sort,
    Sort::WeaveMerge => weave_merge_sort,
  }
}
