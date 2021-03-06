use crate::SpinButton;
use glib::object::Cast;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::IsA;
use libc::{c_double, c_int};
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait SpinButtonExtManual: 'static {
    fn connect_input<F>(&self, input_func: F) -> SignalHandlerId
    where
        F: Fn(&Self) -> Option<Result<f64, ()>> + 'static;
}

impl<T: IsA<SpinButton>> SpinButtonExtManual for T {
    fn connect_input<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self) -> Option<Result<f64, ()>> + 'static,
    {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"input\0".as_ptr() as *mut _,
                Some(transmute(input_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn input_trampoline<T, F: Fn(&T) -> Option<Result<f64, ()>> + 'static>(
    this: *mut ffi::GtkSpinButton,
    new_value: *mut c_double,
    f: &F,
) -> c_int
where
    T: IsA<SpinButton>,
{
    match f(&SpinButton::from_glib_borrow(this).unsafe_cast_ref()) {
        Some(Ok(v)) => {
            *new_value = v;
            glib::ffi::GTRUE
        }
        Some(Err(_)) => ffi::GTK_INPUT_ERROR,
        None => glib::ffi::GFALSE,
    }
}
