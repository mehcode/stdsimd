#[cfg(test)]
use stdsimd_test::assert_instr;

/// Return the bit at index `b` of 32-bit integer `a`.
#[inline]
#[cfg_attr(test, assert_instr(btl))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittest_u32(a: &u32, b: u32) -> u8 {
    let r: u8;
    asm!("btl $2, $1\nsetc ${0:b}"
         : "=r"(r)
         : "m"(a), "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

/// Return the bit at index `b` of 32-bit integer `a`,
/// and set that bit to one.
#[inline]
#[cfg_attr(test, assert_instr(btsl))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittestandset_u32(a: &mut u32, b: u32) -> u8 {
    let r: u8;
    asm!("btsl $2, $0\nsetc ${1:b}"
         : "=*m"(a), "=r"(r)
         : "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

/// Return the bit at index `b` of 32-bit integer `a`,
/// and set that bit to zero.
#[inline]
#[cfg_attr(test, assert_instr(btrl))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittestandreset_u32(a: &mut u32, b: u32) -> u8 {
    let r: u8;
    asm!("btrl $2, $0\nsetc ${1:b}"
         : "=*m"(a), "=r"(r)
         : "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

/// Return the bit at index `b` of 32-bit integer `a`,
/// and set that bit to its complement.
#[inline]
#[cfg_attr(test, assert_instr(btcl))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittestandcomplement_u32(a: &mut u32, b: u32) -> u8 {
    let r: u8;
    asm!("btcl $2, $0\nsetc ${1:b}"
         : "=*m"(a), "=r"(r)
         : "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

#[cfg(test)]
mod tests {
    use crate::core_arch::x86::*;

    #[test]
    fn test_bittest_u32() {
        unsafe {
            let a = 0b0101_0000u32;
            assert_eq!(_bittest_u32(&a, 4), 1);
            assert_eq!(_bittest_u32(&a, 5), 0);
        }
    }

    #[test]
    fn test_bittestandset_u32() {
        unsafe {
            let mut a = 0b0101_0000u32;
            assert_eq!(_bittestandset_u32(&mut a, 4), 1);
            assert_eq!(_bittestandset_u32(&mut a, 4), 1);
            assert_eq!(_bittestandset_u32(&mut a, 5), 0);
            assert_eq!(_bittestandset_u32(&mut a, 5), 1);
        }
    }

    #[test]
    fn test_bittestandreset_u32() {
        unsafe {
            let mut a = 0b0101_0000u32;
            assert_eq!(_bittestandreset_u32(&mut a, 4), 1);
            assert_eq!(_bittestandreset_u32(&mut a, 4), 0);
            assert_eq!(_bittestandreset_u32(&mut a, 5), 0);
            assert_eq!(_bittestandreset_u32(&mut a, 5), 0);
        }
    }

    #[test]
    fn test_bittestandcomplement_u32() {
        unsafe {
            let mut a = 0b0101_0000u32;
            assert_eq!(_bittestandcomplement_u32(&mut a, 4), 1);
            assert_eq!(_bittestandcomplement_u32(&mut a, 4), 0);
            assert_eq!(_bittestandcomplement_u32(&mut a, 4), 1);
            assert_eq!(_bittestandcomplement_u32(&mut a, 5), 0);
            assert_eq!(_bittestandcomplement_u32(&mut a, 5), 1);
        }
    }
}
