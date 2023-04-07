// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::GeolocationPosition;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitGeolocationManager")]
    pub struct GeolocationManager(Object<ffi::WebKitGeolocationManager, ffi::WebKitGeolocationManagerClass>);

    match fn {
        type_ => || ffi::webkit_geolocation_manager_get_type(),
    }
}

impl GeolocationManager {
    #[doc(alias = "webkit_geolocation_manager_failed")]
    pub fn failed(&self, error_message: &str) {
        unsafe {
            ffi::webkit_geolocation_manager_failed(
                self.to_glib_none().0,
                error_message.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_geolocation_manager_get_enable_high_accuracy")]
    #[doc(alias = "get_enable_high_accuracy")]
    pub fn enables_high_accuracy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_geolocation_manager_get_enable_high_accuracy(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_geolocation_manager_update_position")]
    pub fn update_position(&self, position: &mut GeolocationPosition) {
        unsafe {
            ffi::webkit_geolocation_manager_update_position(
                self.to_glib_none().0,
                position.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "start")]
    pub fn connect_start<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn start_trampoline<F: Fn(&GeolocationManager) -> bool + 'static>(
            this: *mut ffi::WebKitGeolocationManager,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    start_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stop")]
    pub fn connect_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_trampoline<F: Fn(&GeolocationManager) + 'static>(
            this: *mut ffi::WebKitGeolocationManager,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "enable-high-accuracy")]
    pub fn connect_enable_high_accuracy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_high_accuracy_trampoline<
            F: Fn(&GeolocationManager) + 'static,
        >(
            this: *mut ffi::WebKitGeolocationManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-high-accuracy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_high_accuracy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GeolocationManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GeolocationManager")
    }
}