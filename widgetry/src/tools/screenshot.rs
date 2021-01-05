use abstutil::Timer;

use crate::runner::State;
use crate::{Prerender, SharedAppState};

/// Take a screenshot of the entire canvas, tiling it based on the window's width and height.
pub(crate) fn screenshot_everything<A: SharedAppState>(
    state: &mut State<A>,
    dir_path: &str,
    prerender: &Prerender,
    zoom: f64,
) -> anyhow::Result<()> {
    let mut timer = Timer::new("capturing screen");
    let num_tiles_x = (state.canvas.map_dims.0 * zoom / state.canvas.window_width).ceil() as usize;
    let num_tiles_y = (state.canvas.map_dims.1 * zoom / state.canvas.window_height).ceil() as usize;
    let orig_zoom = state.canvas.cam_zoom;
    let orig_x = state.canvas.cam_x;
    let orig_y = state.canvas.cam_y;

    timer.start_iter("capturing images", num_tiles_x * num_tiles_y);
    state.canvas.cam_zoom = zoom;
    std::fs::create_dir_all(dir_path)?;

    for tile_y in 0..num_tiles_y {
        for tile_x in 0..num_tiles_x {
            timer.next();
            state.canvas.cam_x = (tile_x as f64) * state.canvas.window_width;
            state.canvas.cam_y = (tile_y as f64) * state.canvas.window_height;

            let suffix = state.draw(prerender, true).unwrap_or_else(String::new);
            let filename = format!(
                "{}/{:02}x{:02}{}.png",
                dir_path,
                tile_x + 1,
                tile_y + 1,
                suffix
            );
            prerender.inner.screencap(&state.canvas, filename)?;
        }
    }

    state.canvas.cam_zoom = orig_zoom;
    state.canvas.cam_x = orig_x;
    state.canvas.cam_y = orig_y;
    Ok(())
}
