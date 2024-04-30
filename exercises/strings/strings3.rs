// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn trim_me( mut input: String) -> String {
    // TODO: Remove whitespace from both ends of a string!
          let s= String::from(input.trim());
          s
}

fn compose_me( mut input: String) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.push_str(" world!");
    input
    
   

   
}

fn replace_me( input: String) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    (input.replace("cars","balloons")).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me(String::from("Hello!     ")), "Hello!".to_string());
        assert_eq!(trim_me(String::from("  What's up!")), "What's up!".to_string());
        assert_eq!(trim_me(String::from("   Hola!  ")), "Hola!".to_string());
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me( String::from("Hello")), "Hello world!".to_string());
        assert_eq!(compose_me( String::from("Goodbye")), "Goodbye world!".to_string());
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me(String::from("I think cars are cool")), String::from("I think balloons are cool"));
        assert_eq!(replace_me(String::from("I love to look at cars")), String::from("I love to look at balloons"));
    }
}
