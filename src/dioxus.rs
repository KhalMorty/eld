#![doc = include_str!("../DIOXUS.md")]

use crate::chart::ChartProps;
use crate::chart::Segment;
use crate::draw_chart;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct Properties {
    /// The dataset for the chart.
    ///
    /// A signal that holds a vector of `Segment` values, representing the data points
    /// that will be visualized on the chart. This is a reactive signal, meaning it
    /// will automatically trigger updates when modified.
    pub data: Signal<Vec<Segment>>,

    /// The width of the canvas in pixels.
    ///
    /// Defines the width of the chart's canvas. Defaults to `900px` if not provided.
    #[props(default = 900)]
    pub width: u32,

    /// The height of the canvas in pixels.
    ///
    /// Specifies the height of the chart's canvas. Defaults to `300px` if not provided.
    #[props(default = 300)]
    pub height: u32,

    /// The background color of the chart.
    ///
    /// Specifies the color used for the chart's background. Accepts any valid CSS color format.
    /// Defaults to `"#FFFFFF"` (white) if not provided.
    #[props(default = "#FFFFFF")]
    pub background_color: &'static str,

    /// The color of the grid lines on the chart.
    ///
    /// Defines the color used for the grid lines. Accepts any valid CSS color format.
    /// Defaults to `"#CCCCCC"` (light gray) if not provided.
    #[props(default = "#CCCCCC")]
    pub grid_color: &'static str,

    /// The font style used for labels.
    ///
    /// Defines the font used for axis labels and other text elements in the chart.
    /// The format follows CSS font specifications (e.g., `"bold 14px Arial"`).
    /// Defaults to `"bold 14px Arial"` if not provided.
    #[props(default = "bold 14px Arial")]
    pub font: &'static str,

    /// The color of the axis labels.
    ///
    /// Specifies the color used for the text labels on the chart's axes.
    /// Defaults to `"#444444"` (dark gray) if not provided.
    #[props(default = "#444444")]
    pub label_color: &'static str,

    /// The color used to represent "Off Duty" status.
    ///
    /// This color is used in the chart to visualize periods when a user is off duty.
    /// Defaults to `"#8E8E8E"` (gray) if not provided.
    #[props(default = "#8E8E8E")]
    pub off_duty_color: &'static str,

    /// The color used to represent "Sleeper" status.
    ///
    /// This color is used in the chart to indicate periods when a user is in a sleeper berth.
    /// Defaults to `black` if not provided.
    #[props(default = "black")]
    pub sleeper_color: &'static str,

    /// The color used to represent "Driving" status.
    ///
    /// This color is used in the chart to highlight periods when a user is actively driving.
    /// Defaults to `green` if not provided.
    #[props(default = "green")]
    pub driving_color: &'static str,

    /// The color used to represent "On Duty" status.
    ///
    /// This color is used in the chart to indicate periods when a user is on duty but not driving.
    /// Defaults to `orange` if not provided.
    #[props(default = "orange")]
    pub on_duty_color: &'static str,
}

/// Chart Component
///
/// A Dioxus component that renders a **duty status chart** inside a `<canvas>` element.
/// This chart visually represents different duty statuses (Off Duty, Sleeper, Driving, On Duty)
/// based on the provided data.
///
/// # Properties
/// The `Chart` component accepts a set of customizable properties through the `Properties` struct:
///
/// - **data** *(Signal<Vec<Segment>>)* - The log data containing time segments for different duty statuses.
/// - **width** *(u32)* - The width of the chart in pixels.
/// - **height** *(u32)* - The height of the chart in pixels.
/// - **background_color** *(String)* - The background color of the chart.
/// - **grid_color** *(String)* - The color of the grid lines.
/// - **font** *(String)* - The font used for labels.
/// - **label_color** *(String)* - The color of the labels.
/// - **off_duty_color** *(String)* - The color representing **Off Duty** time.
/// - **sleeper_color** *(String)* - The color representing **Sleeper Berth** time.
/// - **driving_color** *(String)* - The color representing **Driving** time.
/// - **on_duty_color** *(String)* - The color representing **On Duty** time.
///
/// # Examples
///
/// ## Basic Usage
/// This example renders a `Chart` component with sample data:
///
/// ```rust
/// use dioxus::prelude::*;
/// use eld::{Segment, DutyStatus};
/// use eld::dioxus::Chart;
///
/// #[component]
/// fn App() -> Element {
///     let eld_data = use_signal(|| vec![
///         Segment { start_hour: 0.0, end_hour: 6.0, status: DutyStatus::OffDuty, location: "".to_string(), note: "".to_string() },
///         Segment { start_hour: 6.0, end_hour: 12.0, status: DutyStatus::Sleeper, location: "".to_string(), note: "".to_string() },
///         Segment { start_hour: 12.0, end_hour: 18.0, status: DutyStatus::Driving, location: "".to_string(), note: "".to_string() },
///         Segment { start_hour: 18.0, end_hour: 24.0, status: DutyStatus::OnDuty, location: "".to_string(), note: "".to_string() },
///     ]);
///
///     rsx! {
///         Chart {
///             data: eld_data,
///             width: 800,
///             height: 400,
///             background_color: "#ffffff",
///             grid_color: "#cccccc",
///             font: "12px Arial",
///             label_color: "#333333",
///             off_duty_color: "#dddddd",
///             sleeper_color: "#000000",
///             driving_color: "#28a745",
///             on_duty_color: "#ff9800",
///         }
///     }
/// }
/// ```
///
/// # Behavior
/// - When `data` changes, the `use_effect` hook **redraws the chart**.
/// - The chart is **scrollable horizontally** for better visibility on smaller screens.
/// - Uses the `draw_chart` function to render the chart inside the `<canvas>` element.
///
/// # Notes
/// - The `<canvas>` must have a unique `id` (`eld-canvas`) for proper rendering.
/// - The `draw_chart` function must be implemented separately and handle the drawing logic.
#[component]
pub fn Chart(props: Properties) -> Element {
    let data = props.data.clone();
    let hook_props = props.clone();

    use_effect(move || {
        if let Err(err) = draw_chart(
            &data(),
            &ChartProps {
                width: hook_props.width,
                height: hook_props.height,
                background_color: hook_props.background_color,
                grid_color: hook_props.grid_color,
                font: hook_props.font,
                label_color: hook_props.label_color,
                off_duty_color: hook_props.off_duty_color,
                sleeper_color: hook_props.sleeper_color,
                driving_color: hook_props.driving_color,
                on_duty_color: hook_props.on_duty_color,
            },
        ) {
            log::error!("Failed to draw chart: {}", err);
        }
    });

    rsx! {
        div {
            id: "eld-container",
            style: "position: relative; max-width: 100%; overflow-x: auto;",
            canvas {
                id: "eld-canvas",
                width: "{props.width}",
                height: "{props.height}",
                style: "border: 1px solid black; cursor: pointer; background-color: {props.background_color};"
            }
        }
    }
}
