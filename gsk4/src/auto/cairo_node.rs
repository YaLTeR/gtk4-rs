// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct CairoNode(Object<ffi::GskCairoNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_cairo_node_get_type(),
    }
}

impl CairoNode {
    #[doc(alias = "gsk_cairo_node_new")]
    pub fn new(bounds: &graphene::Rect) -> CairoNode {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gsk_cairo_node_new(bounds.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_cairo_node_get_draw_context")]
    pub fn get_draw_context(&self) -> Option<cairo::Context> {
        unsafe { from_glib_full(ffi::gsk_cairo_node_get_draw_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_cairo_node_get_surface")]
    pub fn get_surface(&self) -> Option<cairo::Surface> {
        unsafe { from_glib_none(ffi::gsk_cairo_node_get_surface(self.to_glib_none().0)) }
    }
}

impl fmt::Display for CairoNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CairoNode")
    }
}
