fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    pub fn it_shall_display_i_love_netwo_in_a_frame() {
        assert_eq!(display_in_a_frame("I love netwo"), r#"
            *********
            * I     *
            * Love  *
            * Netwo *
            *********");
    }
}
