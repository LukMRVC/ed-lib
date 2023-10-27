use std::fmt::{write, Debug};

pub trait Label {
    type LabelType;
    fn get_label(&self) -> &Self::LabelType;
    fn new(label: Self::LabelType) -> Self
    where
        Self: Sized;
    fn set_label(&mut self, new_label: Self::LabelType);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct StringLabel(String);

impl std::fmt::Display for StringLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.0)
    }
}

impl Label for StringLabel {
    type LabelType = String;
    fn get_label(&self) -> &Self::LabelType {
        &self.0
    }

    fn new(label: Self::LabelType) -> Self {
        StringLabel(label)
    }

    fn set_label(&mut self, new_label: Self::LabelType) {
        self.0 = new_label;
    }
}

impl From<&[u8]> for StringLabel {
    fn from(value: &[u8]) -> Self {
        StringLabel(String::from_utf8(value.to_vec()).unwrap())
    }
}

impl Debug for dyn Label<LabelType = String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrent_display_impl() {
        let lbl = StringLabel::new("haha".to_owned());
        assert_eq!(lbl.to_string(), "haha");
    }
}
