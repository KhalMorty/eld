use crate::input::Input;
use crate::stats::Stats;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use eld::dioxus::Chart;
use eld::Segment;

mod input;
mod stats;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const MAIN_CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    let eld_data = use_signal::<Vec<Segment>>(Vec::new);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            class: "container",
            h1 { "Electronic Logging Device (ELD)" }
            Input { eld_data: eld_data.clone() }
            Chart {
                data: eld_data.clone(),
                width: 1000,
                height: 350,
                background_color: "#F0F0F0",
                grid_color: "#BBBBBB",
                font: "16px Arial",
                label_color: "#222222",
                on_duty_color: "#FFD700",
            }
            Stats { eld_data: eld_data.clone() }
        }
    }
}
