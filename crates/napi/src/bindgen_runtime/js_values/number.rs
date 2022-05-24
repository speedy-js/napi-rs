use super::{check_status, sys, Result};
use crate::type_of;

macro_rules! impl_number_conversions {
  ( $( ($name:literal, $t:ty as $st:ty, $get:ident, $create:ident) ,)* ) => {
    $(
      impl $crate::bindgen_prelude::TypeName for $t {
        #[inline(always)]
        fn type_name() -> &'static str {
          $name
        }

        fn value_type() -> crate::ValueType {
          crate::ValueType::Number
        }
      }

      impl $crate::bindgen_prelude::ValidateNapiValue for $t {
        #[inline(always)]
        fn type_of() -> Vec<$crate::ValueType> {
          vec![$crate::ValueType::Number]
        }
      }

      impl $crate::bindgen_prelude::ToNapiValue for $t {
        #[inline(always)]
        unsafe fn to_napi_value(env: $crate::sys::napi_env, val: $t) -> Result<$crate::sys::napi_value> {
          let mut ptr = std::ptr::null_mut();
          let val: $st = val.into();

          check_status!(
            unsafe { sys::$create(env, val, &mut ptr) },
            "Failed to convert rust type `{}` into napi value",
            $name,
          )?;

          Ok(ptr)
        }
      }

      impl $crate::bindgen_prelude::FromNapiValue for $t {
        #[inline(always)]
        unsafe fn from_napi_value(env: $crate::sys::napi_env, napi_val: $crate::sys::napi_value) -> Result<Self> {
          let mut ret = 0 as $st;

          check_status!(
            unsafe { sys::$get(env, napi_val, &mut ret) },
            "Failed to convert napi value {:?} into rust type `{}`",
            type_of!(env, napi_val),
            $name,
          )?;

          Ok(ret.try_into().expect(concat!("Failed to convert ", stringify!($st), " to ", stringify!($t))))
        }
      }
    )*
  };
}

impl_number_conversions!(
  ("u8", u8 as u32, napi_get_value_uint32, napi_create_uint32),
  ("i8", i8 as i32, napi_get_value_int32, napi_create_int32),
  ("u16", u16 as u32, napi_get_value_uint32, napi_create_uint32),
  ("i16", i16 as i32, napi_get_value_int32, napi_create_int32),
  ("u32", u32 as u32, napi_get_value_uint32, napi_create_uint32),
  ("i32", i32 as i32, napi_get_value_int32, napi_create_int32),
  ("i64", i64 as i64, napi_get_value_int64, napi_create_int64),
  ("f64", f64 as f64, napi_get_value_double, napi_create_double),
);
