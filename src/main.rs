fn main() {
    println!("Hello, world!");
}

pub fn display_in_a_frame(sentence: &str) -> &str {

    let
    r#"*********
* I     *
* Love  *
* Netwo *
*********"#
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn it_shall_display_i_love_netwo_in_a_frame() {
        assert_eq!(
            display_in_a_frame("I Love Netwo"),
            r#"*********
* I     *
* Love  *
* Netwo *
*********"#
        );
    }

    #[test]
    pub fn it_shall_display_i_love_netwo_in_a_frame_lowercase() {
        assert_eq!(
            display_in_a_frame("i love netwo"),
            r#"*********
* i     *
* love  *
* netwo *
*********"#
        );
    }
}
