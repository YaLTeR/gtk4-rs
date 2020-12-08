// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ColorStop;
use crate::RenderNode;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::glib_wrapper! {
    pub struct LinearGradientNode(Object<ffi::GskLinearGradientNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_linear_gradient_node_get_type(),
    }
}

impl LinearGradientNode {
    #[doc(alias = "gsk_linear_gradient_node_get_color_stops")]
    pub fn get_color_stops(&self) -> Vec<ColorStop> {
        unsafe {
            let mut n_stops = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_linear_gradient_node_get_color_stops(
                    self.to_glib_none().0,
                    n_stops.as_mut_ptr(),
                ),
                n_stops.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_end")]
    pub fn get_end(&self) -> Option<graphene::Point> {
        unsafe { from_glib_none(ffi::gsk_linear_gradient_node_get_end(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_n_color_stops")]
    pub fn get_n_color_stops(&self) -> usize {
        unsafe { ffi::gsk_linear_gradient_node_get_n_color_stops(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_start")]
    pub fn get_start(&self) -> Option<graphene::Point> {
        unsafe {
            from_glib_none(ffi::gsk_linear_gradient_node_get_start(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for LinearGradientNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LinearGradientNode")
    }
}
