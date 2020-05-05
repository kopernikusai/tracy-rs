// #[macro_export]
// macro_rules! cstr {
//     ( $( $str: expr ),* ) => (unsafe{ ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($( $str ),* , "\0").as_bytes()) })
// }

#[macro_export]
macro_rules! loc {
    () => {{
        //use std::ffi::CStr;
        $crate::SourceLocation {
            //https://github.com/z33ky/tracy-rs/issues/1
            function: concat!(module_path!(), "::fn(?)\0"),
            file: concat!(file!(), "\0"),
            line: line!(),
        }
    }};
}

#[macro_export]
macro_rules! src_loc_data {
    () => {
        $crate::SourceLocationData::without_name_or_color(&$crate::loc!())
    };

    (name=$name: expr) => {
        $crate::SourceLocationData::with_name(&$crate::loc!(), concat!($name, "\0"))
    };

    (name=$name: expr, color=$color: expr) => {
        $crate::SourceLocationData::with_name_and_color(
            &$crate::loc!(),
            concat!($name, "\0"),
            $color,
        )
    };

    (color=$color: expr) => {
        $crate::SourceLocationData::with_color(&$crate::loc!(), $color)
    };
}

#[macro_export]
macro_rules! zone {
    ($active: expr) => {
        let _zone = {
            const _loc_data: $crate::SourceLocationData = $crate::src_loc_data!();
            $crate::ZoneContext::new(&_loc_data, $active)
        };
    };
}

#[macro_export]
macro_rules! zone_n {
    ($name: expr, $active: expr) => {
        let _zone = {
            const _loc_data: $crate::SourceLocationData =
                $crate::src_loc_data!(name = $name);
            $crate::ZoneContext::new(&_loc_data, $active)
        };
    };
}

#[macro_export]
macro_rules! zone_c {
    ($color: expr, $active: expr) => {
        let _zone = {
            const _loc_data = $crate::src_loc_data!(color = $color);
            $crate::ZoneContext::new(&_loc_data, $active)
        };
    };
}

#[macro_export]
macro_rules! zone_nc {
    ($name: expr, $color: expr, $active: expr) => {
        let _zone = {
            const _loc_data: $crate::SourceLocationData =
                $crate::src_loc_data!(name = $name, color = $color);
            $crate::ZoneContext::new(&_loc_data, $active)
        };
    };
}

#[macro_export]
macro_rules! zone_s {
    ($depth: expr, $active: expr) => {
        let _zone = {
            const _loc_data: $crate::SourceLocationData = $crate::src_loc_data!();
            $crate::ZoneContext::with_callstack(&_loc_data, $depth, $active)
        };
    };
}

#[macro_export]
macro_rules! zone_ns {
    ($name: expr, $depth: expr, $active: expr) => {
        let _zone = {
            const _loc_data: $crate::SourceLocationData =
                $crate::src_loc_data!(name = $name);
            $crate::ZoneContext::with_callstack(&_loc_data, $depth, $active)
        };
    };
}

#[macro_export]
macro_rules! zone_cs {
    ($color: expr, $depth: expr, $active: expr) => {
        let _zone = {
            const _loc_data: $crate::SourceLocationData =
                $crate::src_loc_data!(color = $color);
            $crate::ZoneContext::with_callstack(&_loc_data, $depth, $active)
        };
    };
}

#[macro_export]
macro_rules! zone_ncs {
    ($name: expr, $color: expr, $depth: expr, $active: expr) => {
        let _zone = {
            let _loc_data: $crate::SourceLocationData =
                $crate::src_loc_data!(name = $name, color = $color);
            $crate::ZoneContext::with_callstack(&_loc_data, $depth, $active)
        };
    };
}

#[macro_export]
macro_rules! frame_mark {
    () => {
        unsafe { $crate::sys::___tracy_emit_frame_mark(::core::ptr::null()) };
    };

    ($name: expr) => {
        unsafe {
            $crate::sys::___tracy_emit_frame_mark(
                concat!($name, "\0").as_ptr() as *const $crate::sys::c_char
            )
        };
    };
}

#[macro_export]
macro_rules! frame_mark_start {
    ($name: expr) => {
        unsafe {
            $crate::sys::___tracy_emit_frame_mark_start(
                concat!($name, "\0").as_ptr() as *const $crate::sys::c_char,
            )
        };
    };
}

#[macro_export]
macro_rules! frame_mark_end {
    ($name: expr) => {
        unsafe {
            $crate::sys::___tracy_emit_frame_mark_end(
                concat!($name, "\0").as_ptr() as *const $crate::sys::c_char
            )
        };
    };
}

//TODO: bounds-checked variant
#[macro_export]
macro_rules! frame_mark_image_unchecked {
    ($image: expr, $width: expr, $height: expr, $offset: expr, $flip: expr) => {
        $crate::sys::___tracy_emit_frame_mark_image(
            $image as *const ::libc::c_void,
            $width,
            $height,
            $offset,
            if $flip { 1 } else { 0 },
        );
    };
}

#[macro_export]
macro_rules! plot {
    ($name: expr, $value: expr) => {
        unsafe {
            $crate::sys::___tracy_emit_plot(
                concat!($name, "\0").as_ptr() as *const $crate::sys::c_char,
                $value,
            )
        }
    };
}
