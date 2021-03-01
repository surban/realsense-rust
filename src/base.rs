//! Common types and functions.

use std::ffi::OsStr;

use crate::{
    common::*,
    error::{Error, Result},
};

#[cfg(feature = "with-image")]
pub use rs2_image::*;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(sys::RS2_DEFAULT_TIMEOUT as u64);

/// Fallible conversion to `Cow<'a, CStr>`.
///
/// It is used for FFI interfaces that requires C string pointers.
pub trait TryIntoCowCStr<'a> {
    fn try_into_cow_cstr(self) -> Result<Cow<'a, CStr>>;
}

impl<'a> TryIntoCowCStr<'a> for CString {
    fn try_into_cow_cstr(self) -> Result<Cow<'a, CStr>> {
        Ok(self.into())
    }
}

impl<'a> TryIntoCowCStr<'a> for &'a CStr {
    fn try_into_cow_cstr(self) -> Result<Cow<'a, CStr>> {
        Ok(self.into())
    }
}

impl<'a> TryIntoCowCStr<'a> for String {
    fn try_into_cow_cstr(self) -> Result<Cow<'a, CStr>> {
        let bytes: Option<Vec<_>> = self
            .into_bytes()
            .into_iter()
            .map(|byte| NonZeroU8::new(byte))
            .collect();
        let bytes = bytes.ok_or_else(|| {
            Error::ToCStrConversion(
                "cannot convert to CString: the string cannot contain null bytes",
            )
        })?;
        let cstring = CString::from(bytes);
        Ok(cstring.into())
    }
}

impl<'a> TryIntoCowCStr<'a> for &str {
    fn try_into_cow_cstr(self) -> Result<Cow<'a, CStr>> {
        let bytes: Option<Vec<_>> = self.bytes().map(|byte| NonZeroU8::new(byte)).collect();
        let bytes = bytes.ok_or_else(|| {
            Error::ToCStrConversion(
                "cannot convert to CString: the string cannot contain null bytes",
            )
        })?;
        let cstring = CString::from(bytes);
        Ok(cstring.into())
    }
}

/// Converts an `OsStr` to a CString.
pub fn os_str_to_cstring(s: &OsStr) -> CString {
    #[cfg(unix)]
    {
        CString::new(s.as_bytes()).unwrap()
    }

    #[cfg(windows)]
    {
        CString::new(s.to_str().unwrap().as_bytes()).unwrap()
    }
}

/// The intrinsic parameters for motion devices.
pub struct MotionIntrinsics(pub sys::rs2_motion_device_intrinsic);

impl Deref for MotionIntrinsics {
    type Target = sys::rs2_motion_device_intrinsic;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MotionIntrinsics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<sys::rs2_motion_device_intrinsic> for MotionIntrinsics {
    fn as_ref(&self) -> &sys::rs2_motion_device_intrinsic {
        &self.0
    }
}

impl AsMut<sys::rs2_motion_device_intrinsic> for MotionIntrinsics {
    fn as_mut(&mut self) -> &mut sys::rs2_motion_device_intrinsic {
        &mut self.0
    }
}

unsafe impl Send for MotionIntrinsics {}
unsafe impl Sync for MotionIntrinsics {}

/// The intrinsic parameters of stream.
pub struct Intrinsics(pub sys::rs2_intrinsics);

impl Deref for Intrinsics {
    type Target = sys::rs2_intrinsics;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Intrinsics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<sys::rs2_intrinsics> for Intrinsics {
    fn as_ref(&self) -> &sys::rs2_intrinsics {
        &self.0
    }
}

impl AsMut<sys::rs2_intrinsics> for Intrinsics {
    fn as_mut(&mut self) -> &mut sys::rs2_intrinsics {
        &mut self.0
    }
}

unsafe impl Send for Intrinsics {}
unsafe impl Sync for Intrinsics {}

/// The extrinsic parameters of stream.
pub struct Extrinsics(pub sys::rs2_extrinsics);

#[cfg(feature = "with-nalgebra")]
impl Extrinsics {
    pub fn to_isometry(&self) -> Isometry3<f32> {
        let rotation = {
            let matrix = MatrixMN::<f32, U3, U3>::from_iterator(self.0.rotation.iter().copied());
            UnitQuaternion::from_matrix(&matrix)
        };
        let translation = {
            let [x, y, z] = self.0.translation;
            Translation3::new(x, y, z)
        };
        Isometry3::from_parts(translation, rotation)
    }
}

impl Deref for Extrinsics {
    type Target = sys::rs2_extrinsics;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Extrinsics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<sys::rs2_extrinsics> for Extrinsics {
    fn as_ref(&self) -> &sys::rs2_extrinsics {
        &self.0
    }
}

impl AsMut<sys::rs2_extrinsics> for Extrinsics {
    fn as_mut(&mut self) -> &mut sys::rs2_extrinsics {
        &mut self.0
    }
}

unsafe impl Send for Extrinsics {}
unsafe impl Sync for Extrinsics {}

/// Represents a pose detected by sensor.
#[derive(Debug)]
pub struct PoseData(pub sys::rs2_pose);

impl PoseData {
    pub fn tracker_confidence(&self) -> u32 {
        self.0.tracker_confidence as u32
    }

    pub fn mapper_confidence(&self) -> u32 {
        self.0.mapper_confidence as u32
    }
}

#[cfg(feature = "with-nalgebra")]
impl PoseData {
    pub fn translation(&self) -> Translation3<f32> {
        let sys::rs2_vector { x, y, z } = self.0.translation;
        Translation3::new(x, y, z)
    }

    pub fn velocity(&self) -> Vector3<f32> {
        let sys::rs2_vector { x, y, z } = self.0.velocity;
        Vector3::new(x, y, z)
    }

    pub fn acceleration(&self) -> Vector3<f32> {
        let sys::rs2_vector { x, y, z } = self.0.acceleration;
        Vector3::new(x, y, z)
    }

    pub fn rotation(&self) -> UnitQuaternion<f32> {
        let sys::rs2_quaternion { x, y, z, w } = self.0.rotation;
        Unit::new_unchecked(Quaternion::new(w, x, z, y))
    }

    pub fn angular_velocity(&self) -> Vector3<f32> {
        let sys::rs2_vector { x, y, z } = self.0.angular_velocity;
        Vector3::new(x, y, z)
    }

    pub fn angular_acceleration(&self) -> Vector3<f32> {
        let sys::rs2_vector { x, y, z } = self.0.angular_acceleration;
        Vector3::new(x, y, z)
    }
}

impl Deref for PoseData {
    type Target = sys::rs2_pose;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PoseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<sys::rs2_pose> for PoseData {
    fn as_ref(&self) -> &sys::rs2_pose {
        &self.0
    }
}

impl AsMut<sys::rs2_pose> for PoseData {
    fn as_mut(&mut self) -> &mut sys::rs2_pose {
        &mut self.0
    }
}

unsafe impl Send for PoseData {}
unsafe impl Sync for PoseData {}

/// Contains width and height of a frame.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}
/// Represents the specification of a stream.
#[derive(Debug)]
pub struct StreamProfileData {
    pub stream: StreamKind,
    pub format: Format,
    pub index: usize,
    pub unique_id: i32,
    pub framerate: i32,
}

#[cfg(feature = "with-image")]
mod rs2_image {
    use super::*;

    /// Image type returned by sensor.
    ///
    /// This is a wrapper of various [ImageBuffer](image::ImageBuffer) variants.
    /// It pixel data is stored in slice for better performance.
    #[derive(Debug, Clone)]
    pub enum Rs2Image<'a> {
        Bgr8(ImageBuffer<Bgr<u8>, &'a [u8]>),
        Bgra8(ImageBuffer<Bgra<u8>, &'a [u8]>),
        Rgb8(ImageBuffer<Rgb<u8>, &'a [u8]>),
        Rgba8(ImageBuffer<Rgba<u8>, &'a [u8]>),
        Luma16(ImageBuffer<Luma<u16>, &'a [u16]>),
    }

    /// Creates an owned image by coping underlying buffer.
    impl<'a> Rs2Image<'a> {
        pub fn to_owned(&self) -> DynamicImage {
            self.into()
        }
    }

    impl<'a> From<&Rs2Image<'a>> for DynamicImage {
        fn from(from: &Rs2Image<'a>) -> DynamicImage {
            match from {
                Rs2Image::Bgr8(image) => DynamicImage::ImageBgr8(image.convert()),
                Rs2Image::Bgra8(image) => DynamicImage::ImageBgra8(image.convert()),
                Rs2Image::Rgb8(image) => DynamicImage::ImageRgb8(image.convert()),
                Rs2Image::Rgba8(image) => DynamicImage::ImageRgba8(image.convert()),
                Rs2Image::Luma16(image) => DynamicImage::ImageLuma16(image.convert()),
            }
        }
    }

    impl<'a> From<Rs2Image<'a>> for DynamicImage {
        fn from(from: Rs2Image<'a>) -> DynamicImage {
            (&from).into()
        }
    }
}
