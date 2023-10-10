use std::sync::Arc;

use speedy2d::{
  color::Color,
  dimen::Vector2,
  shape::Rectangle,
  window::{WindowHandler, WindowHelper},
  Graphics2D, Window,
};

use super::{
  array::Highlight,
  config::{ITEM_COUNT, WINDOW_WIDTH},
  ArrayWithCounters,
};
use crate::{config::WINDOW_HEIGHT, DONE_FLAG, ORDER};

pub fn run_gui(mutex_clone: Arc<ArrayWithCounters>) {
  let x_coords = calc_x_coords(WINDOW_WIDTH);

  let window = Window::new_centered("Sorts Animation", (WINDOW_WIDTH, WINDOW_HEIGHT)).unwrap();

  window.run_loop(SortsWindowHandler {
    array: mutex_clone,
    height: WINDOW_WIDTH as f32,
    x_coords,
  });
}

fn calc_x_coords(width: u32) -> Vec<(f32, f32)> {
  let step_size = width as f32 / ITEM_COUNT as f32;

  (0..ITEM_COUNT)
    .map(|i| {
      let x = step_size * (i as f32);
      (x, x + step_size)
    })
    .collect()
}

struct SortsWindowHandler {
  array: Arc<ArrayWithCounters>,
  height: f32,
  x_coords: Vec<(f32, f32)>,
}

impl WindowHandler for SortsWindowHandler {
  fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
    graphics.clear_screen(Color::BLACK);

    let count = ITEM_COUNT as f32;

    let data = (*self.array).to_usize_vec();

    for (item, &(x1, x2)) in data.into_iter().zip(self.x_coords.iter()) {
      let item = item as f32 + 1.0;

      let top_left = Vector2::new(x1, self.height * (1.0 - (item / count)));
      let bottom_right = Vector2::new(x2, self.height);

      graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), Color::WHITE)
    }

    let highlights = self.array.highlights();

    for Highlight(i, color) in highlights {
      let (x1, x2) = self.x_coords[i];
      let item = self.array.get(i) as f32 + 1.0;

      let top_left = Vector2::new(x1, self.height * (1.0 - (item / count)));
      let bottom_right = Vector2::new(x2, self.height);

      graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color)
    }

    if DONE_FLAG.load(ORDER) {
      helper.terminate_loop();
    }

    helper.request_redraw();
  }

  fn on_resize(&mut self, _helper: &mut WindowHelper, new_size: Vector2<u32>) {
    let Vector2 { x, y } = new_size;

    self.height = y as f32;
    self.x_coords = calc_x_coords(x);
  }
}
