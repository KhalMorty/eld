use std::fmt;
use web_sys::{wasm_bindgen::JsCast, window, CanvasRenderingContext2d, HtmlCanvasElement};

/// Represents a segment of time in a driver's log.
///
/// Each segment records a start and end time, the driver's duty status,
/// and the location where the status was recorded.
///
/// # Fields
/// - `start_hour`: The starting hour of the segment (in 24-hour format).
/// - `end_hour`: The ending hour of the segment (in 24-hour format).
/// - `status`: The duty status of the driver during this time period.
/// - `location`: A textual description of the driver's location.
/// - `note`: A textual note of the driver's location.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub start_hour: f32,
    pub end_hour: f32,
    pub status: DutyStatus,
    pub location: String,
    pub note: String,
}

/// Represents the duty status of a driver during a time segment.
///
/// The driver can be in one of four possible states:
/// - `OffDuty`: Not working.
/// - `Sleeper`: Resting in the sleeper berth.
/// - `Driving`: Actively driving.
/// - `OnDuty`: Performing other work-related activities.
#[derive(Debug, Clone, PartialEq)]
pub enum DutyStatus {
    OffDuty,
    Sleeper,
    Driving,
    OnDuty,
    PersonalConveyance,
    YardMove,
}

impl fmt::Display for DutyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match *self {
            DutyStatus::OffDuty => "OffDuty",
            DutyStatus::Sleeper => "Sleeper",
            DutyStatus::Driving => "Driving",
            DutyStatus::OnDuty => "OnDuty",
            DutyStatus::PersonalConveyance => "PersonalConveyance",
            DutyStatus::YardMove => "YardMove",
        };

        write!(f, "{}", status_str)
    }
}

/// Configuration properties for rendering the ELD chart.
///
/// This struct defines various attributes that control the appearance of the
/// chart, including its size, colors, and font styles.
#[derive(PartialEq, Clone)]
pub struct ChartProps {
    /// The width of the chart in pixels.
    pub width: u32,

    /// The height of the chart in pixels.
    pub height: u32,

    /// The background color of the chart.
    ///
    /// This defines the overall canvas color behind the grid and segments.
    pub background_color: &'static str,

    /// The color of the grid lines.
    ///
    /// Grid lines are drawn to divide the chart into time slots and duty status sections.
    pub grid_color: &'static str,

    /// The font used for text labels.
    ///
    /// This applies to hour labels and duty status descriptions.
    pub font: &'static str,

    /// The color of the text labels.
    ///
    /// Labels include hours along the x-axis and duty status names along the y-axis.
    pub label_color: &'static str,

    /// The color representing the "Off Duty" status.
    ///
    /// This color is used to draw segments where the driver is off duty.
    pub off_duty_color: &'static str,

    /// The color representing the "Sleeper" status.
    ///
    /// This color is used to draw segments where the driver is in the sleeper berth.
    pub sleeper_color: &'static str,

    /// The color representing the "Driving" status.
    ///
    /// This color is used to draw segments where the driver is actively driving.
    pub driving_color: &'static str,

    /// The color representing the "On Duty" status.
    ///
    /// This color is used to draw segments where the driver is performing
    /// non-driving work-related activities.
    pub on_duty_color: &'static str,
}

/// Renders the ELD chart using the given segments and chart properties.
///
/// This function first retrieves the canvas and drawing context, then checks if
/// the grid has already been drawn. If not, it draws the grid before rendering
/// the duty status segments.
///
/// # Parameters
/// - `segments`: A slice of `Segment` structs representing the driver's log.
/// - `props`: A reference to `ChartProps` defining the chart's visual settings.
///
/// # Returns
/// - `Ok(&ChartProps)`: If the chart was successfully drawn.
/// - `Err(String)`: If an error occurred (e.g., unable to find the canvas).
pub fn draw_chart<'a>(
    segments: &'a [Segment],
    props: &'a ChartProps,
) -> Result<&'a ChartProps, String> {
    let canvas = get_canvas("eld-canvas")?;
    let context = get_canvas_context(&canvas)?;

    let (width, height) = (canvas.width() as f64, canvas.height() as f64);

    if grid_already_drawn()? {
        draw_segments(&context, segments, width, height, props);
        return Ok(props);
    }

    draw_grid(&context, width, height, props);
    draw_segments(&context, segments, width, height, props);

    mark_grid_as_drawn()?;
    Ok(props)
}

pub fn clear_chart() -> Result<(), String> {
    let canvas = get_canvas("eld-canvas")?;
    let context = get_canvas_context(&canvas)?;
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let document = window()
        .ok_or("No Window found".to_string())?
        .document()
        .ok_or("No Document found".to_string())?;
    if let Some(existing_grid) = document.get_element_by_id("grid-drawn") {
        existing_grid
            .set_attribute("data-drawn", "falase")
            .map_err(|_| "Failed to set attribute".to_string())?;
    }
    Ok(())
}

/// Retrieves the 2D rendering context for a given HTML canvas.
///
/// # Parameters
/// - `canvas`: A reference to the HTML canvas element.
///
/// # Returns
/// - `Ok(HtmlCanvasElement)`: The HTML canvas element.
/// - `Err(String)`: If the context cannot be obtained or cast.
fn get_canvas(id: &str) -> Result<HtmlCanvasElement, String> {
    window()
        .ok_or("No Window found".to_string())?
        .document()
        .ok_or("No Document found".to_string())?
        .get_element_by_id(id)
        .ok_or_else(|| format!("Canvas with id '{}' not found", id))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "Failed to cast element to Canvas".to_string())
}

/// Retrieves the 2D rendering context for a given HTML canvas.
///
/// # Parameters
/// - `canvas`: A reference to the HTML canvas element.
///
/// # Returns
/// - `Ok(CanvasRenderingContext2d)`: If the 2D context is available.
/// - `Err(String)`: If the context cannot be obtained or cast.
fn get_canvas_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, String> {
    canvas
        .get_context("2d")
        .map_err(|_| "Failed to get 2D context".to_string())?
        .ok_or_else(|| "2D context is unavailable".to_string())?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "Failed to cast context to CanvasRenderingContext2d".to_string())
}

/// Checks whether the grid has already been drawn on the canvas.
///
/// The function looks for an HTML element with the ID `"grid-drawn"`, which acts as a flag.
///
/// # Returns
/// - `Ok(true)`: If the grid has already been drawn.
/// - `Ok(false)`: If the grid has not been drawn.
/// - `Err(String)`: If an error occurs while accessing the document.
fn grid_already_drawn() -> Result<bool, String> {
    let document = window()
        .ok_or("No Window found".to_string())?
        .document()
        .ok_or("No Document found".to_string())?;
    if let Some(existing_grid) = document.get_element_by_id("grid-drawn") {
        return Ok(existing_grid.get_attribute("data-drawn") == Some("true".to_string()));
    }
    Ok(false)
}

/// Marks the grid as drawn by adding a hidden HTML element.
///
/// This function creates a `<div>` element with the ID `"grid-drawn"` and a `data-drawn="true"`
/// attribute, which serves as a flag indicating that the grid has already been rendered.
///
/// # Returns
/// - `Ok(())`: If the marker was successfully added.
/// - `Err(String)`: If an error occurs while modifying the DOM.
fn mark_grid_as_drawn() -> Result<(), String> {
    let document = window()
        .ok_or("No Window found".to_string())?
        .document()
        .ok_or("No Document found".to_string())?;
    let grid_marker = document
        .create_element("div")
        .map_err(|_| "Failed to create grid marker".to_string())?;
    grid_marker.set_id("grid-drawn");
    grid_marker
        .set_attribute("data-drawn", "true")
        .map_err(|_| "Failed to set attribute".to_string())?;
    document
        .body()
        .ok_or("No body found".to_string())?
        .append_child(&grid_marker)
        .map_err(|_| "Failed to append child".to_string())?;
    Ok(())
}

/// Draws the background grid on the canvas.
///
/// The grid consists of horizontal and vertical lines that divide the chart
/// into sections representing hours and duty statuses. Labels for time and
/// status categories are also drawn.
///
/// # Parameters
/// - `context`: The 2D rendering context.
/// - `width`: The width of the canvas.
/// - `height`: The height of the canvas.
/// - `props`: The chart properties, including colors and font settings.
fn draw_grid(context: &CanvasRenderingContext2d, width: f64, height: f64, props: &ChartProps) {
    context.clear_rect(0.0, 0.0, width, height);

    let padding_x = 70.0;
    let padding_y = 40.0;
    let row_height = (height - 2.0 * padding_y) / 4.0;
    let col_width = (width - 2.0 * padding_x) / 24.0;

    let statuses = ["Off Duty", "Sleeper", "Driving", "On Duty"];
    let hours = generate_hour_labels();

    context.set_stroke_style_str(props.grid_color);
    context.set_fill_style_str(props.label_color);
    context.set_font(props.font);

    for i in 0..=4 {
        let y = padding_y + i as f64 * row_height;
        context.begin_path();
        context.move_to(padding_x, y);
        context.line_to(width, y);
        context.stroke();

        if i < 4 {
            context
                .fill_text(statuses[i], 10.0, y + row_height / 2.0)
                .unwrap_or_else(|_| log::warn!("Failed to draw text"));
        }
    }

    context.set_font("12px Arial");

    for i in 0..25 {
        let x = padding_x + i as f64 * col_width;
        context.begin_path();
        context.move_to(x, padding_y);
        context.line_to(x, height);

        context.set_stroke_style_str(props.grid_color);
        context.stroke();

        if i % 2 == 0 {
            context
                .fill_text(&hours[i], x - 10.0, height - 10.0)
                .unwrap_or_else(|_| log::warn!("Failed to draw text"));
        }
    }
}

/// Draws the duty status segments on the chart.
///
/// Each segment is represented as a colored line corresponding to the
/// driver's status within a given time range.
///
/// # Parameters
/// - `context`: The 2D rendering context.
/// - `segments`: A slice of `Segment` structs.
/// - `width`: The width of the canvas.
/// - `height`: The height of the canvas.
/// - `props`: The chart properties defining colors and styles.
fn draw_segments(
    context: &CanvasRenderingContext2d,
    segments: &[Segment],
    width: f64,
    height: f64,
    props: &ChartProps,
) {
    let padding_x = 70.0;
    let padding_y = 40.0;
    let row_height = (height - 2.0 * padding_y) / 4.0;
    let col_width = (width - 2.0 * padding_x) / 24.0;

    context.set_line_width(4.0);

    for segment in segments {
        let y_index = match segment.status {
            DutyStatus::OffDuty => 0,
            DutyStatus::Sleeper => 1,
            DutyStatus::Driving => 2,
            DutyStatus::OnDuty => 3,
            DutyStatus::PersonalConveyance | DutyStatus::YardMove => 999, // TODO: add to chart
        };

        let y_val = padding_y + (y_index as f64) * row_height + (row_height / 2.0);
        let x_start = padding_x + (segment.start_hour as f64) * col_width;
        let x_end = padding_x + (segment.end_hour as f64) * col_width;

        let color = match segment.status {
            DutyStatus::OffDuty => props.off_duty_color,
            DutyStatus::Sleeper => props.sleeper_color,
            DutyStatus::Driving => props.driving_color,
            DutyStatus::OnDuty => props.on_duty_color,
            DutyStatus::PersonalConveyance | DutyStatus::YardMove => "",
        };

        context.set_stroke_style_str(color);
        context.begin_path();
        context.move_to(x_start, y_val);
        context.line_to(x_end, y_val);
        context.stroke();
    }
}

/// Generates a list of hour labels for the chart.
///
/// The function returns a vector of formatted hour labels in 12-hour AM/PM format,
/// ranging from "12 AM" to "12 AM" (covering a full 24-hour period).
///
/// # Returns
/// - `Vec<String>`: A vector containing formatted hour labels.
fn generate_hour_labels() -> Vec<String> {
    let mut hours: Vec<String> = (0..24)
        .map(|h| {
            format!(
                "{} {}",
                if h == 0 || h == 12 { 12 } else { h % 12 },
                if h < 12 { "AM" } else { "PM" }
            )
        })
        .collect();

    hours.push("12 AM".to_string());
    hours
}
