// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{FaviconDatabase, ITPThirdParty, WebsiteData, WebsiteDataTypes};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitWebsiteDataManager")]
    pub struct WebsiteDataManager(Object<ffi::WebKitWebsiteDataManager, ffi::WebKitWebsiteDataManagerClass>);

    match fn {
        type_ => || ffi::webkit_website_data_manager_get_type(),
    }
}

impl WebsiteDataManager {
    #[doc(alias = "webkit_website_data_manager_fetch")]
    pub fn fetch<P: FnOnce(Result<Vec<WebsiteData>, glib::Error>) + 'static>(
        &self,
        types: WebsiteDataTypes,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn fetch_trampoline<
            P: FnOnce(Result<Vec<WebsiteData>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_website_data_manager_fetch_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = fetch_trampoline::<P>;
        unsafe {
            ffi::webkit_website_data_manager_fetch(
                self.to_glib_none().0,
                types.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn fetch_future(
        &self,
        types: WebsiteDataTypes,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<WebsiteData>, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.fetch(types, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_website_data_manager_get_base_cache_directory")]
    #[doc(alias = "get_base_cache_directory")]
    pub fn base_cache_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_cache_directory(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_website_data_manager_get_base_data_directory")]
    #[doc(alias = "get_base_data_directory")]
    pub fn base_data_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_data_directory(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_website_data_manager_get_favicon_database")]
    #[doc(alias = "get_favicon_database")]
    pub fn favicon_database(&self) -> Option<FaviconDatabase> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_favicon_database(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_website_data_manager_get_favicons_enabled")]
    #[doc(alias = "get_favicons_enabled")]
    pub fn is_favicons_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_get_favicons_enabled(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_website_data_manager_get_itp_summary")]
    #[doc(alias = "get_itp_summary")]
    pub fn itp_summary<P: FnOnce(Result<Vec<ITPThirdParty>, glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn itp_summary_trampoline<
            P: FnOnce(Result<Vec<ITPThirdParty>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_website_data_manager_get_itp_summary_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = itp_summary_trampoline::<P>;
        unsafe {
            ffi::webkit_website_data_manager_get_itp_summary(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn itp_summary_future(
        &self,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<ITPThirdParty>, glib::Error>> + 'static>,
    > {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.itp_summary(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_website_data_manager_is_ephemeral")]
    pub fn is_ephemeral(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_is_ephemeral(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_website_data_manager_set_favicons_enabled")]
    pub fn set_favicons_enabled(&self, enabled: bool) {
        unsafe {
            ffi::webkit_website_data_manager_set_favicons_enabled(
                self.to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }
}

impl fmt::Display for WebsiteDataManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsiteDataManager")
    }
}
