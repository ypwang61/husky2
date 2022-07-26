mod prototype;
mod registrable;
mod registrable_dyn;
mod registrable_safe;

pub use prototype::*;
pub use registrable::*;
pub use registrable_dyn::*;
pub use registrable_safe::*;

use crate::*;
use std::{
    marker::PhantomData,
    panic::{RefUnwindSafe, UnwindSafe},
};

#[repr(C)]
pub struct __Register<'eval> {
    pub(crate) data_kind: __RegisterDataKind,
    pub(crate) data: __RegisterData,
    pub(crate) proto: &'eval __RegisterPrototype,
}

impl<'eval> std::hash::Hash for __Register<'eval> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union __RegisterData {
    pub(crate) as_void: (),
    pub(crate) as_bool: bool,
    pub(crate) as_i32: i32,
    pub(crate) as_i64: i64,
    pub(crate) as_b32: u32,
    pub(crate) as_b64: u64,
    pub(crate) as_f32: f32,
    pub(crate) as_f64: f64,
    pub(crate) as_opt_ptr: Option<*mut ()>,
}
// C standard (N1570, 6.7.2.1 Structure and union specifiers) says:
// 16 The size of a union is sufficient to contain the largest of its members.
// The value of at most one of the members can be stored in a union object at any time.
// A pointer to a union object, suitably converted, points to each of its members
// (or if a member is a bit- field, then to the unit in which it resides),
// and vice versa.
#[test]
fn test_alignment() {
    let a = __RegisterData { as_void: () };
    unsafe {
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_void as *const _ as *const (),
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_bool as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_i32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_i64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_b32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_b64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_f32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_f64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_opt_ptr as *const _ as *const ()
        )
    }
}

unsafe impl<'eval> Send for __Register<'eval> {}
unsafe impl<'eval> Sync for __Register<'eval> {}

impl<'eval> std::fmt::Debug for __Register<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'eval> Clone for __Register<'eval> {
    fn clone(&self) -> Self {
        Self {
            data_kind: self.data_kind,
            data: match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data,
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::EvalRef => self.data,
                __RegisterDataKind::TempRef => self.data,
                __RegisterDataKind::TempMut => panic!(),
                __RegisterDataKind::Moved => panic!(),
                __RegisterDataKind::Undefined => todo!(),
                __RegisterDataKind::Unreturned => panic!(),
            },
            proto: self.proto,
        }
    }
}

impl<'eval> PartialEq for __Register<'eval> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // self.data_kind == other.data_kind && self.opt_data == other.opt_data
    }
}
impl<'eval> Eq for __Register<'eval> {}

pub trait __StaticInfo {
    type __StaticSelf: __StaticInfo<__StaticSelf = Self::__StaticSelf> + __Registrable + 'static;

    fn __static_type_id__() -> std::any::TypeId {
        std::any::TypeId::of::<Self::__StaticSelf>()
    }

    fn __static_type_name() -> std::borrow::Cow<'static, str>;
}

impl<'eval> __Register<'eval> {
    // pub fn proto(&self) -> &'eval __RegisterPrototype {
    //     self.proto
    // }

    pub unsafe fn new_primitive_value<'a, T: __Registrable + 'a>(
        data: __RegisterData,
        proto: &'eval __RegisterPrototype,
    ) -> __Register<'eval>
    where
        T: Copy,
    {
        __Register {
            data_kind: __RegisterDataKind::PrimitiveValue,
            data,
            proto,
        }
    }

    pub fn new_box<T: __Registrable>(
        value: T,
        proto: &'eval __RegisterPrototype,
    ) -> __Register<'eval> {
        let ptr: *mut T = Box::<T>::into_raw(Box::new(value));
        #[cfg(feature = "check")]
        assert_eq!(T::static_type_id, prototype.type_id);
        __Register {
            data_kind: __RegisterDataKind::Box,
            data: __RegisterData {
                as_opt_ptr: Some(ptr as *mut ()),
            },
            proto,
        }
    }

    pub unsafe fn new_eval_ref<T: __Registrable + 'eval>(
        value: &'eval T,
        proto: &'eval __RegisterPrototype,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::EvalRef,
            data: __RegisterData {
                as_opt_ptr: Some(ptr as *mut ()),
            },
            proto,
        }
    }

    pub unsafe fn new_temp_ref<T: __Registrable>(
        value: &T,
        proto: &'eval __RegisterPrototype,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempRef,
            data: __RegisterData {
                as_opt_ptr: Some(ptr as *mut ()),
            },
            proto,
        }
    }

    pub unsafe fn new_temp_mut<T: __Registrable>(
        value: &mut T,
        proto: &'eval __RegisterPrototype,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempMut,
            data: __RegisterData {
                as_opt_ptr: Some(ptr as *mut ()),
            },
            proto,
        }
    }

    pub fn register_move(&mut self) -> __Register<'eval> {
        let moved = __Register {
            data_kind: __RegisterDataKind::Moved,
            data: __RegisterData { as_opt_ptr: None },
            proto: self.proto,
        };
        std::mem::replace(self, moved)
    }

    pub fn new_undefined(proto: &'eval __RegisterPrototype) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Undefined,
            data: __RegisterData { as_opt_ptr: None },
            proto,
        }
    }

    pub fn new_unreturned(proto: &'eval __RegisterPrototype) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Unreturned,
            data: __RegisterData { as_opt_ptr: None },
            proto,
        }
    }

    pub unsafe fn new_undefined_with_message(
        proto: &'eval __RegisterPrototype,
        message: String,
    ) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Undefined,
            data: __RegisterData { as_opt_ptr: None },
            proto,
        }
    }

    pub unsafe fn __copy__(&self) -> Self {
        todo!()
        // Self {
        //     data_kind: self.data_kind,
        //     data: match self.data_kind {
        //         __RegisterDataKind::Data => todo!(),
        //         __RegisterDataKind::Box => todo!(),
        //         __RegisterDataKind::EvalRef => todo!(),
        //         __RegisterDataKind::TempRef => todo!(),
        //         __RegisterDataKind::TempMut => todo!(),
        //         __RegisterDataKind::Moved => todo!(),
        //     },
        // }
    }

    unsafe fn move_into_raw(&mut self) -> *mut dyn __RegistrableDyn {
        self.data_kind = __RegisterDataKind::Moved;
        std::mem::replace(&mut self.data, __RegisterData { as_opt_ptr: None })
            .as_opt_ptr
            .unwrap()
    }

    pub fn downcast_void(&self) -> () {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__VOID_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_void
        }
    }

    pub fn downcast_bool(&self) -> bool {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__BOOL_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_bool
        }
    }

    pub fn downcast_i32(&self) -> i32 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__I32_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_i32
        }
    }

    pub fn downcast_i64(&self) -> i64 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__I64_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_i64
        }
    }

    pub fn downcast_b32(&self) -> u32 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__B32_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_b32
        }
    }

    pub fn downcast_b64(&self) -> u64 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__B64_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_b64
        }
    }

    pub fn downcast_f32(&self) -> f32 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__F32_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_f32
        }
    }

    pub fn downcast_f64(&self) -> f64 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(
                self.proto as *const _,
                &__F64_REGISTER_PROTOTYPE as *const _
            );
            self.data.as_f64
        }
    }

    // pub fn downcast<T>(&mut self) -> T
    // where
    //     T: __Registrable + 'eval,
    // {
    //     match self.data_kind {
    //         __RegisterDataKind::PrimitiveValue => todo!(),
    //         __RegisterDataKind::Box => unsafe { *Box::from_raw(self.move_into_raw() as *mut T) },
    //         __RegisterDataKind::EvalRef => todo!(),
    //         __RegisterDataKind::TempRef => todo!(),
    //         __RegisterDataKind::TempMut => todo!(),
    //         __RegisterDataKind::Moved => todo!(),
    //         __RegisterDataKind::Undefined => todo!(),
    //         __RegisterDataKind::Unreturned => todo!(),
    //     }
    // }

    pub fn downcast_value<T>(&self) -> T
    where
        T: __Registrable + Copy + 'eval,
    {
        unsafe {
            let true_ref: &T = &*((&self.data as *const _) as *const T);
            true_ref.__copy__()
        }
    }

    pub unsafe fn downcast_temp<T>(&mut self) -> T {
        todo!()
    }

    pub unsafe fn downcast_eval_ref<T: 'eval>(&self) -> &'eval T {
        todo!()
    }

    pub unsafe fn downcast_temp_ref<T>(&self) -> &T {
        todo!()
    }

    pub unsafe fn downcast_temp_mut<T>(&mut self) -> &mut T {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum __RegisterDataKind {
    PrimitiveValue,
    Box,
    EvalRef,
    TempRef,
    TempMut,
    Moved,
    Undefined,
    Unreturned,
}

impl<'eval> Drop for __Register<'eval> {
    fn drop(&mut self) {
        match self.data_kind {
            __RegisterDataKind::Box | __RegisterDataKind::Undefined => unsafe {
                // when undefined, opt_data might hold a message
                (*std::mem::replace(&mut self.data, __RegisterData { as_opt_ptr: None })
                    .as_opt_ptr
                    .unwrap())
                .__drop_dyn__()
            },
            _ => (),
        }
    }
}

#[macro_export]
macro_rules! register_new_copyable {
    ($argument: expr, $Type: ty, direct) => {{
        ($argument).to_register()
    }};
}

// impl<'temp, 'eval: 'temp> dyn __RegistrableDyn + 'temp {
//     #[inline]
//     pub fn __downcast_ref<'a, T: __Registrable>(&'a self) -> &'a T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_type_name(),
//                 self.__static_type_name_dyn()
//             )
//         }
//         let ptr: *const dyn __AnyValueDyn = &*self;
//         let ptr: *const T = ptr as *const T;
//         unsafe { &*ptr }
//     }
//     #[inline]
//     pub fn __downcast_copy<'a, T: __Registrable + Copy>(&'a self) -> T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_type_name(),
//                 self.__static_type_name_dyn()
//             )
//         }
//         let ptr: *const dyn __AnyValueDyn = &*self;
//         let ptr: *const T = ptr as *const T;
//         unsafe { *ptr }
//     }

//     #[inline]
//     pub fn __downcast_mut<T: __Registrable>(&mut self) -> &mut T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_type_name(),
//                 self.__static_type_name_dyn()
//             )
//         }
//         let ptr: *mut dyn __AnyValueDyn = &mut *self;
//         let ptr: *mut T = ptr as *mut T;
//         unsafe { &mut *ptr }
//     }
// }
