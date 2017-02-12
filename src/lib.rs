mod token;

#[cfg(test)]
mod tests {
    use token::Type;
    use token::Token;

    #[test]
    fn token_struct_create_type() {
        let _token = Token{ _type: Type::Integer, value: "1".to_string() };
        assert!(_token.get_type() == Type::Integer);
    }

    #[test]
    fn token_object_create_value() {
        let _token = Token{ _type: Type::Integer, value: "1".to_string() };
        assert!(_token.get_value() == "1".to_string());
    }
}
