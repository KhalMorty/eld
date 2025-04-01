#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/eld/refs/heads/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/eld/blob/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod chart;

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

#[cfg(feature = "lep")]
pub mod leptos;

pub use chart::{clear_chart, draw_chart, DutyStatus, Segment};
