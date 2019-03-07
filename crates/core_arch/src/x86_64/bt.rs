#[cfg(test)]
use stdsimd_test::assert_instr;

/// Return the bit at index `b` of 64-bit integer `a`.
#[inline]
#[cfg_attr(test, assert_instr(btq))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittest_u64(a: &u64, b: u64) -> u8 {
    let r: u8;
    asm!("btq $2, $1\nsetc ${0:b}"
         : "=r"(r)
         : "m"(a), "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

/// Return the bit at index `b` of 64-bit integer `a`,
/// and set that bit to one.
#[inline]
#[cfg_attr(test, assert_instr(btsq))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittestandset_u64(a: &mut u64, b: u64) -> u8 {
    let r: u8;
    asm!("btsq $2, $0\nsetc ${1:b}"
         : "=*m"(a), "=r"(r)
         : "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

/// Return the bit at index `b` of 64-bit integer `a`,
/// and set that bit to zero.
#[inline]
#[cfg_attr(test, assert_instr(btrq))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittestandreset_u64(a: &mut u64, b: u64) -> u8 {
    let r: u8;
    asm!("btrq $2, $0\nsetc ${1:b}"
         : "=*m"(a), "=r"(r)
         : "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

/// Return the bit at index `b` of 64-bit integer `a`,
/// and set that bit to its complement.
#[inline]
#[cfg_attr(test, assert_instr(btcq))]
#[unstable(feature = "bittest", issue = "0")]
pub unsafe fn _bittestandcomplement_u64(a: &mut u64, b: u64) -> u8 {
    let r: u8;
    asm!("btcq $2, $0\nsetc ${1:b}"
         : "=*m"(a), "=r"(r)
         : "lr"(b)
         : "cc", "flags", "fpsr");
    r
}

#[cfg(test)]
mod tests {
    use crate::core_arch::x86_64::*;

    #[test]
    fn test_bittest_u64() {
        unsafe {
            let a = 0b0101_0000u64;
            assert_eq!(_bittest_u64(&a, 4), 1);
            assert_eq!(_bittest_u64(&a, 5), 0);
        }
    }

    #[test]
    fn test_bittestandset_u64() {
        unsafe {
            let mut a = 0b0101_0000u64;
            assert_eq!(_bittestandset_u64(&mut a, 4), 1);
            assert_eq!(_bittestandset_u64(&mut a, 4), 1);
            assert_eq!(_bittestandset_u64(&mut a, 5), 0);
            assert_eq!(_bittestandset_u64(&mut a, 5), 1);
        }
    }

    #[test]
    fn test_bittestandreset_u64() {
        unsafe {
            let mut a = 0b0101_0000u64;
            assert_eq!(_bittestandreset_u64(&mut a, 4), 1);
            assert_eq!(_bittestandreset_u64(&mut a, 4), 0);
            assert_eq!(_bittestandreset_u64(&mut a, 5), 0);
            assert_eq!(_bittestandreset_u64(&mut a, 5), 0);
        }
    }

    #[test]
    fn test_bittestandcomplement_u64() {
        unsafe {
            let mut a = 0b0101_0000u64;
            assert_eq!(_bittestandcomplement_u64(&mut a, 4), 1);
            assert_eq!(_bittestandcomplement_u64(&mut a, 4), 0);
            assert_eq!(_bittestandcomplement_u64(&mut a, 4), 1);
            assert_eq!(_bittestandcomplement_u64(&mut a, 5), 0);
            assert_eq!(_bittestandcomplement_u64(&mut a, 5), 1);
        }
    }
}
