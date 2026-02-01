mod canvas;
mod raster;

// inside modules to use
pub use canvas::Canvas;

// running each module's tests
pub use canvas::test as test_canvas;
pub use raster::test as test_raster;