//! # RealSense Bindings for Rust
//!
//! The project provides high-level bindings (crate `realsense_rust`) to librealsense2 library as well as low-level FFI
//! (crate `realsense_sys`) interface.
//!
//! **Default bindings are for librealsense version: 2.44.0**
//!
//! This project is hosted on both [Github](https://github.com/Tangram-Vision/realsense-rust) and
//! [Gitlab](https://gitlab.com/tangram-vision-oss/realsense-rust/). While we're happy to receive pull / merge requests
//! on either platform, we focus most of our work on Gitlab, so please submit an issue there if you've found something
//! we need to improve or have a question regarding how things work.
//!
//! ## Getting Started
//!
//! Make sure the current librealsense version above is installed on your system. Visit the [RealSense official
//! repository](https://github.com/IntelRealSense/librealsense) to download and install this on the host machine.
//!
//! Once that's done, add this crate to your project's `Cargo.toml`.
//!
//! ## Examples and Usage
//!
//! Check out the examples folder for helpful snippets of code, as well as minimal configurations that fit some of the
//! most popular RealSense devices. For more explanation, see the crate documentation.
//!
//! ### Features
//!
//! Use these by running `cargo run --features <name of feature>`
//!
//! - **buildtime-bindgen**: Generate Rust bindings during build time.
//! - **device-test**: Enable tests that requires connections to RealSense devices.
//!
//! ## Regenerating the API Bindings
//!
//! *Non-Linux users*: The current bindings are formatted for Linux. Users on systems other than Linux must run with the
//! `buildtime-bindgen` feature to reformat the bindings. See the README in realsense-sys for more.
//!
//! *Backwards compatibility*: If you're using an older librealsense version, you may enable the `buildtime-bindgen`
//! feature to re-generate the bindings. We make no claims of backwards compatibility; good luck.
//!
//! ## Special Considerations
//!
//! - **USB Current Draw**: Many RealSense devices draw more current than a standard USB cable can provide. For example,
//!   standard USB can run 0.9 amps, while the RealSense 435i draws 2 amps. Using a USB cable that doesn't have the
//!   right current capability will interfere with the USB connection on the host, and the device will seem to
//!   disconnect. A device power cycle doesn't always remedy this, either. In many cases, the host USB hub itself will
//!   need a reset. Make sure any USB cables used are able to draw at least 2 amps. Read more on the issue
//!   [here](https://support.intelrealsense.com/hc/en-us/community/posts/360033595714-D435-USB-connection-issues).
//!
//! - **USB Bandwidth**: When a device is connected, librealsense will measure the transmission speed of data across its
//!   USB connection. USB3 speeds can handle all streams running simultaneously. USB2 speeds _cannot_; trying to set a
//!   streaming configuration that is too much for USB2 will result in a failed streaming config, and will cause the
//!   program to fail. Luckily, this information can be looked up and compensated for during runtime. See the
//!   device-specific demo examples for ways to achieve this.
//!
//! - **Supported but Ignored Stream Options**: There are a few Sensor options that are registered as "supported" by the
//!   sensor, but are actually just set to their default values on runtime. These options are listed and tested in
//!   `check_supported_but_ignored_sensor_options()` device tests. Currently,
//!   [GlobalTimeEnabled](kind::Rs2Option::GlobalTimeEnabled) on the L500 is the only setting known to suffer from this.
//!   However, the test has been written in a way that makes it easy to test more Options for this same behavior.
//!
//! ## Realsense-sys: A low-level API
//!
//! The realsense-sys crate provides C bindings generated from librealsense headers. See the [realsense-sys
//! crate](https://crates.io/crates/realsense-sys) documentation for more information.
//!
//! ## Design Philosophy
//!
//! There's a lot of thought that went into making this library Rust-safe. Check out the
//! [architecture](docs::architecture) doc for our thoughts on Rust safety, error handling, and more for this API.

pub mod base;
pub mod config;
pub mod context;
pub mod device;
pub mod device_hub;
pub mod docs;
mod error;
pub mod frame;
pub mod kind;
pub mod pipeline;
pub mod sensor;
pub mod stream_profile;

// pub mod frame_queue;
// pub mod processing_block;
// pub mod processing_block_kind;
// pub mod processing_block_list;

/// The module collects common used traits from this crate.
pub mod prelude {
    pub use crate::frame::{FrameCategory, FrameEx};
}

// pub use frame_queue::FrameQueue;
// pub use processing_block::{
//     Align, AnyProcessingBlock, Colorizer, DecimationFilter, DisparityFilter, HoleFillingFilter,
//     HuffmanDepthDecompress, PointCloud, ProcessingBlock, RatesPrinter, SpatialFilter, Syncer,
//     TemporalFilter, ThresholdFilter, UnitsTransform, YuyDecoder, ZeroOrderFilter,
// };
// pub use processing_block_list::{ProcessingBlockList, ProcessingBlockListIntoIter};
