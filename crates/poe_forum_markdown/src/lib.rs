pub use crate::{
    content::{clean_text, get_content, html_to_markdown},
    details::{get_details, PostDetails},
};

pub mod content;
pub mod details;
mod selectors;
