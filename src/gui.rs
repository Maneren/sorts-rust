use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};

use speedy2d::{
  color::Color,
  dimen::Vector2,
  shape::Rectangle,
  window::{WindowHandler, WindowHelper},
  Graphics2D, Window,
};

use super::{
  array::Highlight,
  config::{ITEM_COUNT, WINDOW_SIZE},
  ArrayWithCounters,
};

pub fn run_gui(mutex_clone: Arc<ArrayWithCounters>, done: Arc<AtomicBool>) {
  let x_coords = (0..ITEM_COUNT)
    .map(|i| {
      (
        (WINDOW_SIZE as f32) / (ITEM_COUNT as f32) * (i as f32),
        (WINDOW_SIZE as f32) / (ITEM_COUNT as f32) * ((i as f32) + 1_f32),
      )
    })
    .collect::<Vec<_>>();

  let window = Window::new_centered("Sorts Animation", (WINDOW_SIZE, WINDOW_SIZE)).unwrap();

  window.run_loop(SortsWindowHandler {
    array: mutex_clone,
    done,
    x_coords,
  });
}

struct SortsWindowHandler {
  array: Arc<ArrayWithCounters>,
  done: Arc<AtomicBool>,
  x_coords: Vec<(f32, f32)>,
}

impl WindowHandler for SortsWindowHandler {
  fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
    graphics.clear_screen(Color::BLACK);

    let count = ITEM_COUNT as f32;
    let size = WINDOW_SIZE as f32;

    let data = (*self.array).to_usize_vec();

    for (i, item) in data.into_iter().enumerate() {
      let item = item as f32 + 1.0;

      let (x1, x2) = self.x_coords[i];

      let top_left = Vector2::new(x1, size * (1.0 - (item / count)));
      let bottom_right = Vector2::new(x2, size);

      graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), Color::WHITE)
    }

    let highlights = self.array.highlights();

    for Highlight(i, color) in highlights {
      let (x1, x2) = self.x_coords[i];
      let item = self.array.get(i) as f32 + 1.0;

      let top_left = Vector2::new(x1, size * (1.0 - (item / count)));
      let bottom_right = Vector2::new(x2, size);

      graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color)
    }

    if self.done.load(Ordering::Relaxed) {
      helper.terminate_loop();
    }

    helper.request_redraw();
  }
}
