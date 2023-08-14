use std::fmt::Display;

pub struct Math;

#[allow(dead_code)]
impl Math {
    pub fn checked_add<T>(arg1: T, arg2: T) -> Result<T, String>
    where
        T: num_traits::PrimInt + Display,
    {
        if let Some(res) = arg1.checked_add(&arg2) {
            Ok(res)
        } else {
            Err(format!("Error: Overflow in {} + {}", arg1, arg2))
        }
    }

    pub fn checked_sub<T>(arg1: T, arg2: T) -> Result<T, String>
    where
        T: num_traits::PrimInt + Display,
    {
        if let Some(res) = arg1.checked_sub(&arg2) {
            Ok(res)
        } else {
            Err(format!("Error: Overflow in {} + {}", arg1, arg2))
        }
    }

    pub fn checked_mul<T>(arg1: T, arg2: T) -> Result<T, String>
    where
        T: num_traits::PrimInt + Display,
    {
        if let Some(res) = arg1.checked_mul(&arg2) {
            Ok(res)
        } else {
            Err(format!("Error: Overflow in {} + {}", arg1, arg2))
        }
    }

    pub fn checked_div<T>(arg1: T, arg2: T) -> Result<T, String>
    where
        T: num_traits::PrimInt + Display,
    {
        if let Some(res) = arg1.checked_div(&arg2) {
            Ok(res)
        } else {
            Err(format!("Error: Overflow in {} + {}", arg1, arg2))
        }
    }

    pub fn checked_as_u64<T>(arg: T) -> Result<u64, String>
    where
        T: Display + num_traits::ToPrimitive + Clone,
    {
        let option: Option<u64> = num_traits::NumCast::from(arg.clone());
        if let Some(res) = option {
            Ok(res)
        } else {
            Err(format!("Error: Overflow in {}", arg))
        }
    }
}
