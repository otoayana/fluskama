//! # fluskama
//!
//! an [askama](https://github.com/djc/askama) wrapper for the [fluffer](https://codeberg.org/catboomer/fluffer)
//! gemini server framework. it eases serving gemini pages written in
//! askama by creating a wrapper for askama's template type.
//!
//! ## wrapping a template
//! as previously mentioned, fluskama works as a wrapper for any askama templates. in order to wrap a template,
//! we can call `FluffTemplate::from()`
//!
//! ```rust
//! # use fluskama::FluffTemplate;
//! # use askama::Template;
//! #
//! # #[derive(Template)]
//! # #[template(path = "page.gmi", escape = "txt")]
//! # struct Page {
//! #     name: String,
//! #     age: u8,
//! # }
//! #
//! # async fn page() -> FluffTemplate<Page> {
//! #     let template = Page {
//! #         name: String::from("John Doe"),
//! #         age: 21
//! #     };
//! #
//! FluffTemplate::from(template)
//! # }
//! ```
use askama::Template;
use fluffer::{async_trait, GemBytes};

/// Wrapper for any type implementing askama::Template, which adds
/// fluffer::GemBytes as a trait. It can be used as a companion to
/// write gemtext templates, which can later be served using fluffer.
/// It is advised to use a "txt" escaper within the source template.
#[derive(Debug)]
pub struct FluffTemplate<T: Template> {
    template: T,
}

impl<T: Template> FluffTemplate<T> {
    /// Wraps any struct implementing the Template trait into a
    /// FluffTemplate.
    pub fn from(template: T) -> Self {
        FluffTemplate { template }
    }
}

#[async_trait]
impl<T: Template + std::marker::Send> GemBytes for FluffTemplate<T> {
    async fn gem_bytes(self) -> Vec<u8> {
        let template = match self.template.render() {
            Ok(template) => format!("20 text/gemini\r\n{}", template),
            Err(error) => format!("50 text/gemini\r\n{}", error),
        };

        template.into_bytes()
    }
}
