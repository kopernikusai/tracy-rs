#![allow(unused)]

//#![feature(const_cstr_unchecked)]

use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop};

#[cfg(feature = "enable")]
mod macros;

#[cfg(not(feature = "enable"))]
#[path = "macros_disabled.rs"]
mod macros;

pub use macros::*;

pub type ColorType = u32;
mod colors;

pub mod sys {
    use libc::{c_char, c_int, c_void, size_t};

    #[repr(C)]
    pub struct source_location_data {
        pub name: *const c_char,
        pub function: *const c_char,
        pub file: *const c_char,
        pub line: u32,
        pub color: u32,
    }

    #[repr(C)]
    pub struct zone_context {
        pub id: u32,
        pub active: c_int,
    }

    #[link(name = "tracy")]
    extern "C" {
        pub fn ___tracy_emit_zone_begin(
            srcloc: *const source_location_data,
            active: c_int,
        ) -> zone_context;
        pub fn ___tracy_emit_zone_begin_callstack(
            srcloc: *const source_location_data,
            depth: c_int,
            active: c_int,
        ) -> zone_context;
        pub fn ___tracy_emit_zone_end(ctx: zone_context);
        pub fn ___tracy_emit_zone_text(ctx: zone_context, txt: *const c_char, size: size_t);
        pub fn ___tracy_emit_zone_name(ctx: zone_context, txt: *const c_char, size: size_t);

        pub fn ___tracy_emit_frame_mark(name: *const c_char);
        pub fn ___tracy_emit_frame_mark_start(name: *const c_char);
        pub fn ___tracy_emit_frame_mark_end(name: *const c_char);
        pub fn ___tracy_emit_frame_image(
            image: *const c_void,
            w: u16,
            h: u16,
            offset: u8,
            flip: c_int,
        );
        pub fn ___tracy_emit_plot(name: *const c_char, val: f64);
    }
}

//FIXME: figure out if 'a is fine enough, or if we need 'static
pub struct SourceLocation<'a> {
    pub function: &'a CStr,
    pub file: &'a CStr,
    pub line: u32,
}

pub struct ZoneContext<'a> {
    context: ManuallyDrop<sys::zone_context>,
    marker: PhantomData<SourceLocationData<'a>>,
}

pub type CallstackDepth = libc::c_int;

impl<'a> ZoneContext<'a> {
    #[inline]
    pub fn new(loc: &SourceLocationData<'a>, active: bool) -> Self {
        Self {
            context: ManuallyDrop::new(unsafe {
                sys::___tracy_emit_zone_begin(&loc.data as *const _, if active { 1 } else { 0 })
            }),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_callstack(
        loc: &SourceLocationData<'a>,
        depth: CallstackDepth,
        active: bool,
    ) -> Self {
        Self {
            context: ManuallyDrop::new(unsafe {
                sys::___tracy_emit_zone_begin_callstack(
                    &loc.data as *const _,
                    depth,
                    if active { 1 } else { 0 },
                )
            }),
            marker: PhantomData,
        }
    }
}

impl Drop for ZoneContext<'_> {
    #[inline]
    fn drop(&mut self) {
        //we could avoid this unsafe by manually copying (or deriving Close, though that doesn't
        //seem useful outside this specific use-case), but this is obviously safe also.
        unsafe { sys::___tracy_emit_zone_end(ManuallyDrop::take(&mut self.context)) };
    }
}

pub struct SourceLocationData<'a> {
    data: sys::source_location_data,
    marker: PhantomData<&'a CStr>,
}

impl<'a> SourceLocationData<'a> {
    #[inline]
    pub const fn with_name_and_color(
        loc: &SourceLocation<'a>,
        name: &'a CStr,
        color: ColorType,
    ) -> Self {
        Self {
            data: sys::source_location_data {
                name: name.as_ptr(),
                function: loc.function.as_ptr(),
                file: loc.file.as_ptr(),
                line: loc.line,
                color,
            },
            marker: PhantomData,
        }
    }

    #[inline]
    pub const fn with_color(loc: &SourceLocation<'a>, color: ColorType) -> Self {
        Self {
            data: sys::source_location_data {
                name: std::ptr::null(),
                function: loc.function.as_ptr(),
                file: loc.file.as_ptr(),
                line: loc.line,
                color,
            },
            marker: PhantomData,
        }
    }

    #[inline]
    pub const fn with_name(loc: &SourceLocation<'a>, name: &'a CStr) -> Self {
        Self::with_name_and_color(loc, name, 0)
    }

    #[inline]
    pub const fn without_name_or_color(loc: &SourceLocation<'a>) -> Self {
        Self::with_color(loc, 0)
    }
}

#[test]
fn it_works() {
    println!("Generating some test data to verify this basically works...");
    {
        zone_ncs!("it_works", 0, 1, true);

        for i in 0..10 {
            println!("step");
            for i in 0..20_000 {
                let val = (i as f64).sin();
                plot!("counter", val);
                println!("tick: {}", val);
            }
            frame_mark!();
        }
    }

    assert!(true);
}
