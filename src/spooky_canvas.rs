mod canvas;
mod raster;

// inside modules to use
pub use canvas::Canvas;

// running each module's tests
pub use canvas::main_test as test_canvas;
pub use raster::main_test as test_raster;