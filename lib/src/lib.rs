pub mod label;
pub mod node;
pub mod parsing;

#[cfg(test)]
mod tests {
    use crate::label::Label;

    use super::*;

    #[test]
    fn it_works() {
        let lbl = label::StringLabel::new("haha".to_owned());
    }
}
