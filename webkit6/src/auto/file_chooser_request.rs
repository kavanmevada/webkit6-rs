// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitFileChooserRequest")]
    pub struct FileChooserRequest(Object<ffi::WebKitFileChooserRequest, ffi::WebKitFileChooserRequestClass>);

    match fn {
        type_ => || ffi::webkit_file_chooser_request_get_type(),
    }
}

impl FileChooserRequest {
    #[doc(alias = "webkit_file_chooser_request_cancel")]
    pub fn cancel(&self) {
        unsafe {
            ffi::webkit_file_chooser_request_cancel(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_file_chooser_request_get_mime_types")]
    #[doc(alias = "get_mime_types")]
    pub fn mime_types(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_mime_types(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_file_chooser_request_get_mime_types_filter")]
    #[doc(alias = "get_mime_types_filter")]
    pub fn mime_types_filter(&self) -> Option<gtk::FileFilter> {
        unsafe {
            from_glib_none(ffi::webkit_file_chooser_request_get_mime_types_filter(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_file_chooser_request_get_select_multiple")]
    #[doc(alias = "get_select_multiple")]
    pub fn selects_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_file_chooser_request_get_select_multiple(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_file_chooser_request_get_selected_files")]
    #[doc(alias = "get_selected_files")]
    pub fn selected_files(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::webkit_file_chooser_request_get_selected_files(self.to_glib_none().0),
            )
        }
    }

    #[doc(alias = "webkit_file_chooser_request_select_files")]
    pub fn select_files(&self, files: &[&str]) {
        unsafe {
            ffi::webkit_file_chooser_request_select_files(
                self.to_glib_none().0,
                files.to_glib_none().0,
            );
        }
    }

    pub fn filter(&self) -> Option<gtk::FileFilter> {
        glib::ObjectExt::property(self, "filter")
    }

    #[doc(alias = "filter")]
    pub fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<F: Fn(&FileChooserRequest) + 'static>(
            this: *mut ffi::WebKitFileChooserRequest,
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
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mime-types")]
    pub fn connect_mime_types_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mime_types_trampoline<F: Fn(&FileChooserRequest) + 'static>(
            this: *mut ffi::WebKitFileChooserRequest,
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
                b"notify::mime-types\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mime_types_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "select-multiple")]
    pub fn connect_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_multiple_trampoline<
            F: Fn(&FileChooserRequest) + 'static,
        >(
            this: *mut ffi::WebKitFileChooserRequest,
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
                b"notify::select-multiple\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_select_multiple_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected-files")]
    pub fn connect_selected_files_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_files_trampoline<
            F: Fn(&FileChooserRequest) + 'static,
        >(
            this: *mut ffi::WebKitFileChooserRequest,
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
                b"notify::selected-files\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_files_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileChooserRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileChooserRequest")
    }
}