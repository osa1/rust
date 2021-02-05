// check-pass
// run-rustfix
// compile-flags: -W invalid-ptr-to-int-cast

fn main() {
    // Motivation for this lint: user meant u16::MAX
    let _ = u16::max as u32; //~ casting pointer to `u32` [invalid_ptr_to_int_cast]

    // Pointer to int
    let a: usize = 123;
    let ptr: *const usize = &a as *const usize;

    let _ = ptr as u8; //~ casting pointer to `u8` [invalid_ptr_to_int_cast]
    let _ = ptr as u16; //~ casting pointer to `u16` [invalid_ptr_to_int_cast]
    let _ = ptr as u32; //~ casting pointer to `u32` [invalid_ptr_to_int_cast]
    let _ = ptr as u64;
    let _ = ptr as u128; //~ casting pointer to `u128` [invalid_ptr_to_int_cast]
    let _ = ptr as i8; //~ casting pointer to `i8` [invalid_ptr_to_int_cast]
    let _ = ptr as i16; //~ casting pointer to `i16` [invalid_ptr_to_int_cast]
    let _ = ptr as i32; //~ casting pointer to `i32` [invalid_ptr_to_int_cast]
    let _ = ptr as i64; //~ casting pointer to `i64` [invalid_ptr_to_int_cast]
    let _ = ptr as i128; //~ casting pointer to `i128` [invalid_ptr_to_int_cast]
    let _ = ptr as usize;

    // Function to int
    fn test() {}

    let _ = test as u8; //~ casting pointer to `u8` [invalid_ptr_to_int_cast]
    let _ = test as u16; //~ casting pointer to `u16` [invalid_ptr_to_int_cast]
    let _ = test as u32; //~ casting pointer to `u32` [invalid_ptr_to_int_cast]
    let _ = test as u64;
    let _ = test as u128; //~ casting pointer to `u128` [invalid_ptr_to_int_cast]
    let _ = test as i8; //~ casting pointer to `i8` [invalid_ptr_to_int_cast]
    let _ = test as i16; //~ casting pointer to `i16` [invalid_ptr_to_int_cast]
    let _ = test as i32; //~ casting pointer to `i32` [invalid_ptr_to_int_cast]
    let _ = test as i64; //~ casting pointer to `i64` [invalid_ptr_to_int_cast]
    let _ = test as i128; //~ casting pointer to `i128` [invalid_ptr_to_int_cast]
    let _ = test as usize;

    // Make sure we handle delayed coercion cast checking
    let _ = {({ test }) as u16}; //~ casting pointer to `u16` [invalid_ptr_to_int_cast]
}
