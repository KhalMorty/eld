# 🧬 ELD Dioxus Usage

Adding ELD to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the ELD Chart component to your dependencies by including it in your `Cargo.toml` file.

   ```sh
   cargo add eld --features=dio
   ```

1. Import the `ELD` components into your Dioxus component and start using it in your app.

## 🛠️ Usage

Incorporating the ELD components into your application is easy. Follow these steps:

1. Import the ELD components into your Dioxus project:

   ```rust
   use dioxus::prelude::*;
   use eld::{Segment, DutyStatus};
   use eld::dioxus::Chart;
   ```

1. Use the `ELD` components within your Dioxus application:

   ```rust
   use dioxus::prelude::*;
   use eld::{Segment, DutyStatus};
   use eld::dioxus::Chart;

   #[component]
   fn App() -> Element {
       let eld_data = use_signal(|| vec![
           Segment { start_hour: 0.0, end_hour: 6.0, status: DutyStatus::OffDuty, location: "".to_string(), note: "".to_string() },
           Segment { start_hour: 6.0, end_hour: 12.0, status: DutyStatus::Sleeper, location: "".to_string(), note: "".to_string() },
           Segment { start_hour: 12.0, end_hour: 18.0, status: DutyStatus::Driving, location: "".to_string(), note: "".to_string() },
           Segment { start_hour: 18.0, end_hour: 24.0, status: DutyStatus::OnDuty, location: "".to_string(), note: "".to_string() },
       ]);

       rsx! {
           Chart {
               data: eld_data,
               width: 800,
               height: 400,
               background_color: "#ffffff",
               grid_color: "#cccccc",
               font: "12px Arial",
               label_color: "#333333",
               off_duty_color: "#dddddd",
               sleeper_color: "#000000",
               driving_color: "#28a745",
               on_duty_color: "#ff9800",
           }
       }
   }
   ```

## 🔧 Props

The `Chart` component supports various properties that allow customization.

| Property           | Type                   | Description                                               | Default             |
| ------------------ | ---------------------- | --------------------------------------------------------- | ------------------- |
| `data`             | `Signal<Vec<Segment>>` | The dataset representing time segments for duty statuses. | **Required**        |
| `width`            | `u32`                  | Width of the chart in pixels.                             | `900`               |
| `height`           | `u32`                  | Height of the chart in pixels.                            | `300`               |
| `background_color` | `&'static str`         | Background color of the chart.                            | `"#FFFFFF"`         |
| `grid_color`       | `&'static str`         | Color of the grid lines.                                  | `"#CCCCCC"`         |
| `font`             | `&'static str`         | Font style for axis labels and text elements.             | `"bold 14px Arial"` |
| `label_color`      | `&'static str`         | Color of the labels on the chart.                         | `"#444444"`         |
| `off_duty_color`   | `&'static str`         | Color representing **Off Duty** time.                     | `"#8E8E8E"`         |
| `sleeper_color`    | `&'static str`         | Color representing **Sleeper Berth** time.                | `"black"`           |
| `driving_color`    | `&'static str`         | Color representing **Driving** time.                      | `"green"`           |
| `on_duty_color`    | `&'static str`         | Color representing **On Duty (not driving)** time.        | `"orange"`          |

## 🎨 Rendering & Behavior

- The chart is drawn inside a **`<canvas>` element**.
- It **automatically updates** when the `data` signal changes.
- The **use_effect** hook ensures the chart is re-rendered when necessary.
- The `<canvas>` is **scrollable horizontally**.
