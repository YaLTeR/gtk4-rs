// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::AxisFlags;
use glib::translate::*;

#[repr(C)]
pub struct TimeCoord(ffi::GdkTimeCoord);

impl TimeCoord {
    pub fn new(time: u32, axes: [f64; 12], flags: AxisFlags) -> TimeCoord {
        assert_initialized_main_thread!();
        TimeCoord(ffi::GdkTimeCoord {
            time,
            axes,
            flags: flags.to_glib(),
        })
    }

    pub fn get_time(&self) -> u32 {
        self.0.time
    }

    pub fn get_axes(&self) -> &[f64; 12] {
        &self.0.axes
    }

    pub fn get_flags(&self) -> AxisFlags {
        unsafe { from_glib(self.0.flags) }
    }
}

#[doc(hidden)]
impl GlibPtrDefault for TimeCoord {
    type GlibType = *mut ffi::GdkTimeCoord;
}
#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkTimeCoord) -> Self {
        TimeCoord(*ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_full(ptr: *mut ffi::GdkTimeCoord) -> Self {
        let res = from_glib_none(ptr);
        glib::ffi::g_free(ptr as *mut _);
        res
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<ffi::GdkTimeCoord, *mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::GdkTimeCoord, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GdkTimeCoord, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::GdkTimeCoord, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}
