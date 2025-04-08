use regex::Regex;
use std::any::type_name;

/// Get the type name without the module prefixes.
#[must_use]
pub fn short_type_name<T>() -> String {
    shorten_type_names(type_name::<T>())
}

/// Remove module prefixes of anything that matches that pattern of a type name.
#[must_use]
pub fn shorten_type_names(input: &str) -> String {
    let regex = Regex::new(r"(&?)([a-zA-Z_:]+::)?([a-zA-Z_]+)").expect("Regex should be valid");
    regex.replace_all(input, r"$1$3").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_type_name_test() {
        assert_eq!(short_type_name::<String>(), "String");
        assert_eq!(short_type_name::<&str>(), "&str");
        assert_eq!(short_type_name::<Option<String>>(), "Option<String>");
        assert_eq!(
            short_type_name::<Result<&str, String>>(),
            "Result<&str, String>"
        );
    }
}
