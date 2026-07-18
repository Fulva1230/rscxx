pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn string_concat(left: &str, right: &str) -> String {
    left.to_owned() + right
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn string_concat(left: &str, right: &str) -> String;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
