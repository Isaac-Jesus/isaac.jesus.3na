pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len {
        unsafe{
            product *= *ptr.offset(i as isize);
        }
    }
    product
}


// o codigo não rodava pois havia um erro no indice que comecava pelo 1 e nao por 0

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}

fn main() {
    println!("Hello, world!");
}