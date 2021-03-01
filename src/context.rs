//! Defines the sensor context.

use crate::{
    base::os_str_to_cstring,
    common::*,
    device_hub::DeviceHub,
    device_list::DeviceList,
    error::{ErrorChecker, Result},
};

#[derive(Debug)]
pub struct Context {
    pub(crate) ptr: NonNull<sys::rs2_context>,
}

impl Context {
    /// Create an instance.
    pub fn new() -> Result<Self> {
        let ptr = {
            let mut checker = ErrorChecker::new();
            let context = unsafe {
                sys::rs2_create_context(sys::RS2_API_VERSION as i32, checker.inner_mut_ptr())
            };
            checker.check()?;
            context
        };

        let context = Self {
            ptr: NonNull::new(ptr).unwrap(),
        };

        Ok(context)
    }

    /// Create an [DeviceHub](DeviceHub) instance.
    pub fn create_device_hub(&self) -> Result<DeviceHub> {
        let ptr = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = sys::rs2_create_device_hub(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            ptr
        };

        let hub = DeviceHub {
            ptr: NonNull::new(ptr).unwrap(),
        };
        Ok(hub)
    }

    /// Discover available devices.
    pub fn query_devices(&self, product_mask: Option<c_int>) -> Result<DeviceList> {
        let list = match product_mask {
            Some(mask) => unsafe {
                let mut checker = ErrorChecker::new();
                let list =
                    sys::rs2_query_devices_ex(self.ptr.as_ptr(), mask, checker.inner_mut_ptr());
                checker.check()?;
                DeviceList::from_raw(list)
            },
            None => unsafe {
                let mut checker = ErrorChecker::new();
                let list = sys::rs2_query_devices(self.ptr.as_ptr(), checker.inner_mut_ptr());
                checker.check()?;
                DeviceList::from_raw(list)
            },
        };

        Ok(list)
    }

    /// Add device file to context.
    pub fn add_device<P>(&mut self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let cstring = os_str_to_cstring(file.as_ref().as_os_str());
        unsafe {
            let mut checker = ErrorChecker::new();
            sys::rs2_context_add_device(
                self.ptr.as_ptr(),
                cstring.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
        }
        Ok(())
    }

    // /// Remove device file from context. (unimplemented)
    // pub fn remove_device<P>(&mut self, file: P) -> Result<()>
    // where
    //     P: AsRef<Path>,
    // {
    //     todo!();
    // }

    pub fn into_raw(self) -> *mut sys::rs2_context {
        let ptr = self.ptr;
        mem::forget(self);
        ptr.as_ptr()
    }

    pub unsafe fn from_raw(ptr: *mut sys::rs2_context) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub(crate) unsafe fn unsafe_clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::rs2_delete_context(self.ptr.as_ptr()) }
    }
}

unsafe impl Send for Context {}
