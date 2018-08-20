

enum Drawable {
    Rectangle(u32, u32, u32, u32),
    Circle(u32, u32, u32),
}

trait GameVisualOutput {
    fn display_list() -> &[Drawable];
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
