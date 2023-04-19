fn main() {
    println!("Hello, {}!", "JoJo");
}

fn truthy() -> bool {
    return true;
}

#[cfg(test)]
mod test {
    use super::truthy;

    #[test]
    fn test_something() {
        assert_eq!(truthy(), true)
    }
}
