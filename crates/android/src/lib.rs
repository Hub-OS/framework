#![allow(unused_doc_comments)]

mod android_jvm;

pub mod content;
pub mod graphics;
pub mod net;
pub mod text;
pub mod util;
pub mod view;

pub use ::android_activity as activity;
pub use android_jvm::*;
pub use jni;
