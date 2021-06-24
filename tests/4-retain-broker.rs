struct Foo {
    pub dict: enchant::Dict,
}

impl Foo {
    fn new() -> Self {
        let mut broker = enchant::Broker::new();
        let dict = broker.request_dict("en_US").unwrap();
        Self { dict }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let _foo = Foo::new();
    }
}
