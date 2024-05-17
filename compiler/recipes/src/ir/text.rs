use super::{Expression, RichTextInner, RichTextPart};

impl RichTextPart {
  pub fn r#static(content: String) -> Self {
    Self {
      rich_text_inner: Some(RichTextInner::Static(content)),
    }
  }

  pub fn dynamic(content: Expression) -> Self {
    Self {
      rich_text_inner: Some(RichTextInner::Dynamic(content)),
    }
  }
}
