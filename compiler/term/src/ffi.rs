use core::convert::{TryFrom, TryInto};

use crate::encoding::*;
use crate::tag::Tag;

use crate::encoding::Encoding32 as E32;
use crate::encoding::Encoding64 as E64;
use crate::encoding::Encoding64Nanboxed as E64N;

mod tablegen {
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/term_encoding.rs"));

    impl<T> TryInto<Tag<T>> for TermKind
    where
        T: Word,
    {
        type Error = ();

        fn try_into(self) -> Result<Tag<T>, Self::Error> {
            match self {
                TermKind::None => Ok(Tag::None),
                TermKind::Atom | TermKind::Boolean => Ok(Tag::Atom),
                TermKind::Fixnum => Ok(Tag::SmallInteger),
                TermKind::BigInt => Ok(Tag::BigInteger),
                TermKind::Float => Ok(Tag::Float),
                TermKind::Nil => Ok(Tag::Nil),
                TermKind::Cons => Ok(Tag::List),
                TermKind::Tuple => Ok(Tag::Tuple),
                TermKind::Map => Ok(Tag::Map),
                TermKind::Closure => Ok(Tag::Closure),
                TermKind::HeapBin => Ok(Tag::HeapBinary),
                TermKind::ProcBin => Ok(Tag::ProcBin),
                TermKind::Box => Ok(Tag::Box),
                TermKind::Term
                | TermKind::Pid
                | TermKind::Reference
                | TermKind::List
                | TermKind::Number
                | TermKind::Integer
                | TermKind::Binary => Err(()),
            }
        }
    }
}

pub use self::tablegen::*;

#[repr(C)]
pub struct EncodingInfo {
    pub pointer_size: u32,
    pub supports_nanboxing: bool,
}

macro_rules! unwrap_term_kind {
    ($kind:expr) => {{
        let kind: Result<TermKind, _> = $kind.try_into();
        match kind {
            Ok(k) => k,
            Err(_) => {
                panic!("use of invalid term kind value: {}", $kind);
            }
        }
    }};
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_type"]
#[cfg(target_pointer_width = "32")]
pub extern "C" fn is_type(ty: u32, value: usize) -> bool {
    do_is_type::<E32>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_type"]
#[cfg(all(target_pointer_width = "64", target_arch = "x86_64"))]
pub extern "C" fn is_type(ty: u32, value: usize) -> bool {
    do_is_type::<E64N>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_type"]
#[cfg(all(target_pointer_width = "64", not(target_arch = "x86_64")))]
pub extern "C" fn is_type(ty: u32, value: usize) -> bool {
    do_is_type::<E64>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_boxed_type"]
#[cfg(target_pointer_width = "32")]
pub extern "C" fn is_boxed_type(ty: u32, value: usize) -> bool {
    do_is_boxed_type::<E32>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_boxed_type"]
#[cfg(all(target_pointer_width = "64", target_arch = "x86_64"))]
pub extern "C" fn is_boxed_type(ty: u32, value: usize) -> bool {
    do_is_boxed_type::<E64N>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_boxed_type"]
#[cfg(all(target_pointer_width = "64", not(target_arch = "x86_64")))]
pub extern "C" fn is_boxed_type(ty: u32, value: usize) -> bool {
    do_is_boxed_type::<E64>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_tuple"]
#[cfg(target_pointer_width = "32")]
pub extern "C" fn is_tuple_type(arity: usize, value: usize) -> bool {
    do_is_tuple::<E32>(arity, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_tuple"]
#[cfg(all(target_pointer_width = "64", target_arch = "x86_64"))]
pub extern "C" fn is_tuple_type(arity: usize, value: usize) -> bool {
    do_is_tuple::<E64N>(arity, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_tuple"]
#[cfg(all(target_pointer_width = "64", not(target_arch = "x86_64")))]
pub extern "C" fn is_tuple_type(arity: usize, value: usize) -> bool {
    do_is_tuple::<E64>(arity, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_function"]
#[cfg(target_pointer_width = "32")]
pub extern "C" fn is_function_type(arity: usize, value: usize) -> bool {
    do_is_function::<E32>(arity, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_function"]
#[cfg(all(target_pointer_width = "64", target_arch = "x86_64"))]
pub extern "C" fn is_function_type(arity: usize, value: usize) -> bool {
    do_is_function::<E64N>(arity, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_is_function"]
#[cfg(all(target_pointer_width = "64", not(target_arch = "x86_64")))]
pub extern "C" fn is_function_type(arity: usize, value: usize) -> bool {
    do_is_function::<E64>(arity, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_encode_immediate"]
#[cfg(target_pointer_width = "32")]
pub extern "C" fn encode_immediate(ty: u32, value: usize) -> usize {
    do_encode_immediate::<E32>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_encode_immediate"]
#[cfg(all(target_pointer_width = "64", target_arch = "x86_64"))]
pub extern "C" fn encode_immediate(ty: u32, value: usize) -> usize {
    do_encode_immediate::<E64N>(ty, value)
}

#[unwind(allowed)]
#[export_name = "__lumen_builtin_encode_immediate"]
#[cfg(all(target_pointer_width = "64", not(target_arch = "x86_64")))]
pub extern "C" fn encode_immediate(ty: u32, value: usize) -> usize {
    do_encode_immediate::<E64>(ty, value)
}

/// This is a less efficient, but more general type checking function,
/// primarily meant for consumption during compile-time
#[unwind(allowed)]
#[export_name = "lumen_is_type"]
pub extern "C" fn generic_is_type(encoding: *const EncodingInfo, ty: u32, value: u64) -> bool {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => do_is_type::<E32>(ty, value as usize),
        64 if encoding.supports_nanboxing => do_is_type::<E64N>(ty, value as usize),
        64 => do_is_type::<E64>(ty, value as usize),
        _ => unreachable!(),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_encode_immediate"]
pub extern "C" fn generic_encode_immediate(
    encoding: *const EncodingInfo,
    ty: u32,
    value: u64,
) -> u64 {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => do_encode_immediate::<E32>(ty, value as usize) as u64,
        64 if encoding.supports_nanboxing => do_encode_immediate::<E64N>(ty, value as usize) as u64,
        64 => do_encode_immediate::<E64>(ty, value as usize) as u64,
        ps => unreachable!("invalid pointer size {:?}", ps),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_encode_header"]
pub extern "C" fn encode_header(encoding: *const EncodingInfo, ty: u32, arity: u64) -> u64 {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => do_encode_header::<E32>(ty, arity as usize) as u64,
        64 if encoding.supports_nanboxing => do_encode_header::<E64N>(ty, arity as usize) as u64,
        64 => do_encode_header::<E64>(ty, arity as usize) as u64,
        _ => unreachable!(),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_list_tag"]
pub extern "C" fn list_tag(encoding: *const EncodingInfo) -> u64 {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => Encoding32::TAG_LIST as u64,
        64 if encoding.supports_nanboxing => Encoding64Nanboxed::TAG_LIST,
        64 => Encoding64::TAG_LIST,
        _ => unreachable!(),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_box_tag"]
pub extern "C" fn box_tag(encoding: *const EncodingInfo) -> u64 {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => Encoding32::TAG_BOXED as u64,
        64 if encoding.supports_nanboxing => Encoding64Nanboxed::TAG_BOXED,
        64 => Encoding64::TAG_BOXED,
        _ => unreachable!(),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_literal_tag"]
pub extern "C" fn literal_tag(encoding: *const EncodingInfo) -> u64 {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => Encoding32::TAG_LITERAL as u64,
        64 if encoding.supports_nanboxing => Encoding64Nanboxed::TAG_LITERAL,
        64 => Encoding64::TAG_LITERAL,
        _ => unreachable!(),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_immediate_mask"]
pub extern "C" fn immediate_mask(encoding: *const EncodingInfo) -> MaskInfo {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => Encoding32::immediate_mask_info(),
        64 if encoding.supports_nanboxing => Encoding64Nanboxed::immediate_mask_info(),
        64 => Encoding64::immediate_mask_info(),
        _ => unreachable!(),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_list_mask"]
pub extern "C" fn list_mask(encoding: *const EncodingInfo) -> u64 {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => Encoding32::MASK_PRIMARY as u64,
        64 if encoding.supports_nanboxing => Encoding64Nanboxed::TAG_MASK,
        64 => Encoding64::MASK_PRIMARY,
        _ => unreachable!(),
    }
}

#[unwind(allowed)]
#[export_name = "lumen_header_mask"]
pub extern "C" fn header_mask(encoding: *const EncodingInfo) -> MaskInfo {
    let encoding = unsafe { &*encoding };
    match encoding.pointer_size {
        32 => Encoding32::header_mask_info(),
        64 if encoding.supports_nanboxing => Encoding64Nanboxed::header_mask_info(),
        64 => Encoding64::header_mask_info(),
        _ => unreachable!(),
    }
}

#[inline]
fn do_is_type<T>(ty: u32, value: usize) -> bool
where
    T: Encoding,
    <T as Encoding>::Type: Word,
    <<T as Encoding>::Type as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let kind = unwrap_term_kind!(ty);
    let tag = T::type_of(value.try_into().unwrap());
    // This is necessary to check some types which may be either boxed
    // or immediate, but if a type kind is known to be boxed, one should
    // use is_boxed_type/2 instead
    let is_boxed = tag.is_box();
    let tag = if is_boxed {
        let value = unsafe { *(value as *const usize) };
        T::type_of(value.try_into().unwrap())
    } else {
        tag
    };
    match kind {
        // Because these term types represent polymorphic
        // types which may be either boxed or immediate, we
        // have to perform some more precise checks to avoid
        // misclassifying the input value
        TermKind::Term => is_boxed || tag.is_term(),
        TermKind::List => tag.is_list(),
        TermKind::Number if is_boxed => tag.is_boxed_number(),
        TermKind::Number => tag.is_number(),
        TermKind::Integer if is_boxed => tag.is_big_integer(),
        TermKind::Integer => tag.is_integer(),
        TermKind::Binary => tag.is_binary(),
        TermKind::Pid if is_boxed => tag.is_external_pid(),
        TermKind::Pid => tag.is_pid(),
        TermKind::Reference if is_boxed => tag.is_external_reference(),
        TermKind::Reference => tag.is_reference(),
        TermKind::Boolean => !is_boxed && T::is_boolean(value.try_into().unwrap()),
        _ if is_boxed && tag.is_boxable() => {
            let expected: Result<Tag<T::Type>, _> = kind.try_into();
            match expected {
                Ok(t) => tag == t,
                Err(_) => unreachable!(),
            }
        }
        _ if !is_boxed => {
            let expected: Result<Tag<T::Type>, _> = kind.try_into();
            match expected {
                Ok(t) => tag == t,
                Err(_) => unreachable!(),
            }
        }
        _ => false,
    }
}

#[inline]
fn do_is_boxed_type<T>(ty: u32, value: usize) -> bool
where
    T: Encoding,
    <T as Encoding>::Type: Word,
    <<T as Encoding>::Type as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let kind = unwrap_term_kind!(ty);
    let tag = T::type_of(value.try_into().unwrap());
    if Tag::Box != tag {
        return false;
    }
    let value = unsafe { *(value as *const usize) };
    let tag = T::type_of(value.try_into().unwrap());
    match kind {
        TermKind::Term => tag.is_term(),
        TermKind::List => tag.is_list(),
        TermKind::Number => tag.is_number(),
        TermKind::Integer => tag.is_integer(),
        TermKind::Binary => tag.is_binary(),
        TermKind::Pid => tag.is_pid(),
        TermKind::Reference => tag.is_reference(),
        _ => {
            let expected: Result<Tag<T::Type>, _> = kind.try_into();
            match expected {
                Ok(t) => tag == t,
                Err(_) => unreachable!(),
            }
        }
    }
}

#[inline]
fn do_is_tuple<T>(arity: usize, value: usize) -> bool
where
    T: Encoding,
    <T as Encoding>::Type: Word,
    <<T as Encoding>::Type as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let tag = T::type_of(value.try_into().unwrap());
    if Tag::Box != tag {
        return false;
    }
    let value = unsafe { *(value as *const usize) };
    let value = value.try_into().unwrap();
    if T::is_tuple(value) {
        let actual_arity = T::Type::as_usize(&T::decode_header_value(value));
        return arity == actual_arity;
    }
    false
}

#[inline]
fn do_is_function<T>(arity: usize, value: usize) -> bool
where
    T: Encoding,
    <T as Encoding>::Type: Word,
    <<T as Encoding>::Type as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let tag = T::type_of(value.try_into().unwrap());
    if Tag::Box != tag {
        return false;
    }
    let ptr = value as *const usize;
    let value = unsafe { *ptr };
    let value = value.try_into().unwrap();
    if T::is_function(value) {
        // HACK(pauls): This is dependent on the layout of Closure,
        // which we don't have access to in this crate. It is unlikely
        // to change in a way that breaks this, but should that happen,
        // this will need to be changed accordingly
        //
        // Closure {
        //   header: usize / Header<Closure>,
        //   module: usize / Atom
        //   arity: u32,
        //   ...
        // }
        let arity_ptr = unsafe { ptr.offset(3) as *const u32 };
        let actual_arity = unsafe { (*arity_ptr) as usize };
        return arity == actual_arity;
    }
    false
}

#[inline]
fn do_encode_immediate<T>(ty: u32, value: usize) -> usize
where
    T: Encoding,
    <T as Encoding>::Type: Word,
    <<T as Encoding>::Type as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let kind = unwrap_term_kind!(ty);
    let tag: Result<Tag<T::Type>, _> = kind.try_into();
    match tag {
        Ok(t) => {
            let result = T::encode_immediate_with_tag(value.try_into().unwrap(), t);
            result.as_usize()
        }
        Err(_) => {
            panic!(
                "invalid term kind {:?} given to lumen_encode_immediate",
                kind
            );
        }
    }
}

#[inline]
fn do_encode_header<T>(ty: u32, value: usize) -> usize
where
    T: Encoding,
    <T as Encoding>::Type: Word,
    <<T as Encoding>::Type as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let kind = unwrap_term_kind!(ty);
    let tag: Result<Tag<T::Type>, _> = kind.try_into();
    match tag {
        Ok(t) => {
            let result = T::encode_header_with_tag(value.try_into().unwrap(), t);
            result.as_usize()
        }
        Err(_) => {
            panic!("invalid term kind {:?} given to lumen_encode_header", kind);
        }
    }
}
