// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod docs {
        pub mod string_processor {
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod processor {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_reverse_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::reverse(_rt::string_lift(bytes0));
                    let ptr2 = (&raw mut _RET_AREA.0).cast::<u8>();
                    let vec3 = (result1.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(::core::mem::size_of::<*const u8>()).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_reverse<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0
                        .add(::core::mem::size_of::<*const u8>())
                        .cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    fn reverse(input: _rt::String) -> _rt::String;
                }
                #[doc(hidden)]
                macro_rules! __export_docs_string_processor_processor_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "docs:string-processor/processor#reverse")] unsafe extern "C" fn
                        export_reverse(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
                        unsafe { $($path_to_types)*:: _export_reverse_cabi::<$ty > (arg0,
                        arg1) } } #[unsafe (export_name =
                        "cabi_post_docs:string-processor/processor#reverse")] unsafe
                        extern "C" fn _post_return_reverse(arg0 : * mut u8,) { unsafe {
                        $($path_to_types)*:: __post_return_reverse::<$ty > (arg0) } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_docs_string_processor_processor_cabi;
                #[cfg_attr(target_pointer_width = "64", repr(align(8)))]
                #[cfg_attr(target_pointer_width = "32", repr(align(4)))]
                struct _RetArea(
                    [::core::mem::MaybeUninit<
                        u8,
                    >; 2 * ::core::mem::size_of::<*const u8>()],
                );
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 2
                        * ::core::mem::size_of::<*const u8>()],
                );
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #![allow(dead_code, clippy::all)]
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_string_processor_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::docs::string_processor::processor::__export_docs_string_processor_processor_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::docs::string_processor::processor);
    };
}
#[doc(inline)]
pub(crate) use __export_string_processor_impl as export;
#[cfg(target_arch = "wasm32")]
#[unsafe(
    link_section = "component-type:wit-bindgen:0.41.0:docs:string-processor:string-processor:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 246] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07p\x01A\x02\x01A\x02\x01\
B\x02\x01@\x01\x05inputs\0s\x04\0\x07reverse\x01\0\x04\0\x1fdocs:string-processo\
r/processor\x05\0\x04\0&docs:string-processor/string-processor\x04\0\x0b\x16\x01\
\0\x10string-processor\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-co\
mponent\x070.227.1\x10wit-bindgen-rust\x060.41.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
