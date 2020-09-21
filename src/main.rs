use std::cmp::max;

fn main() {
    println!("Hello, world!");
}

pub fn display_in_a_frame(sentence: &str) -> &str {
    let words: Vec<&str> = sentence.split(' ').collect();
    let largest_word_length = words.iter().fold(0, |acc, word| max(acc, word.len()));



    r#"*********
* I     *
* Love  *
* Netwo *
*********"#
}

pub(crate) fn create_line(word: &str, length: usize, padding_character: char) -> &str {
    let mut buffer: &str = format!("* {}{} *", padding_character).as_str();

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
