// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{
    ContextMenu, Frame, ScriptWorld, URIRequest, URIResponse, UserMessage, WebEditor,
    WebFormManager, WebHitTestResult,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitWebPage")]
    pub struct WebPage(Object<ffi::WebKitWebPage, ffi::WebKitWebPageClass>);

    match fn {
        type_ => || ffi::webkit_web_page_get_type(),
    }
}

impl WebPage {
    #[doc(alias = "webkit_web_page_get_editor")]
    #[doc(alias = "get_editor")]
    pub fn editor(&self) -> Option<WebEditor> {
        unsafe { from_glib_none(ffi::webkit_web_page_get_editor(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_web_page_get_form_manager")]
    #[doc(alias = "get_form_manager")]
    pub fn form_manager(&self, world: Option<&ScriptWorld>) -> Option<WebFormManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_form_manager(
                self.to_glib_none().0,
                world.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_web_page_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> u64 {
        unsafe { ffi::webkit_web_page_get_id(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_web_page_get_main_frame")]
    #[doc(alias = "get_main_frame")]
    pub fn main_frame(&self) -> Option<Frame> {
        unsafe { from_glib_none(ffi::webkit_web_page_get_main_frame(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_web_page_get_uri")]
    #[doc(alias = "get_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_web_page_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_web_page_send_message_to_view")]
    pub fn send_message_to_view<P: FnOnce(Result<UserMessage, glib::Error>) + 'static>(
        &self,
        message: &UserMessage,
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
        unsafe extern "C" fn send_message_to_view_trampoline<
            P: FnOnce(Result<UserMessage, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_web_page_send_message_to_view_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = send_message_to_view_trampoline::<P>;
        unsafe {
            ffi::webkit_web_page_send_message_to_view(
                self.to_glib_none().0,
                message.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn send_message_to_view_future(
        &self,
        message: &UserMessage,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserMessage, glib::Error>> + 'static>>
    {
        let message = message.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.send_message_to_view(&message, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "context-menu")]
    pub fn connect_context_menu<F: Fn(&Self, &ContextMenu, &WebHitTestResult) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn context_menu_trampoline<
            F: Fn(&WebPage, &ContextMenu, &WebHitTestResult) -> bool + 'static,
        >(
            this: *mut ffi::WebKitWebPage,
            context_menu: *mut ffi::WebKitContextMenu,
            hit_test_result: *mut ffi::WebKitWebHitTestResult,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(context_menu),
                &from_glib_borrow(hit_test_result),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"context-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    context_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "document-loaded")]
    pub fn connect_document_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn document_loaded_trampoline<F: Fn(&WebPage) + 'static>(
            this: *mut ffi::WebKitWebPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"document-loaded\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    document_loaded_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "send-request")]
    pub fn connect_send_request<F: Fn(&Self, &URIRequest, &URIResponse) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_request_trampoline<
            F: Fn(&WebPage, &URIRequest, &URIResponse) -> bool + 'static,
        >(
            this: *mut ffi::WebKitWebPage,
            request: *mut ffi::WebKitURIRequest,
            redirected_response: *mut ffi::WebKitURIResponse,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(request),
                &from_glib_borrow(redirected_response),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-request\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    send_request_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "user-message-received")]
    pub fn connect_user_message_received<F: Fn(&Self, &UserMessage) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn user_message_received_trampoline<
            F: Fn(&WebPage, &UserMessage) -> bool + 'static,
        >(
            this: *mut ffi::WebKitWebPage,
            message: *mut ffi::WebKitUserMessage,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(message)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"user-message-received\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    user_message_received_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "uri")]
    pub fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<F: Fn(&WebPage) + 'static>(
            this: *mut ffi::WebKitWebPage,
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
                b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_uri_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WebPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebPage")
    }
}