


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod foo {
    #[export_name = "my_demo_function"]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
    #[export_name = "my_demo_function_alias"]
    fn my_demo_function_alias_impl(a: u32) -> u32 {
        my_demo_function(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
