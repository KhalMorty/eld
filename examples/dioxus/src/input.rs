use dioxus::prelude::*;
use eld::{DutyStatus, Segment};

#[component]
pub fn Input(eld_data: Signal<Vec<Segment>>) -> Element {
    let mut start_hour = use_signal(|| "".to_string());
    let mut end_hour = use_signal(|| "".to_string());
    let mut status = use_signal(|| DutyStatus::OffDuty);
    let mut location = use_signal(|| "".to_string());
    let mut message = use_signal(|| "".to_string());
    let mut note = use_signal(|| "".to_string());

    let add_log_entry = move |_| {
        let start = start_hour().parse::<f32>();
        let end = end_hour().parse::<f32>();

        if let (Ok(start), Ok(end)) = (start, end) {
            if start >= end || start < 0.0 || end > 24.0 {
                message.set("Invalid time range!".to_string());
                return;
            }

            eld_data.write().push(Segment {
                start_hour: start,
                end_hour: end,
                status: status(),
                location: location(),
                note: note(),
            });

            start_hour.set("".to_string());
            end_hour.set("".to_string());
            location.set("".to_string());
            note.set("".to_string());
            message.set("Log added successfully!".to_string());
        } else {
            message.set("Please enter valid numeric values.".to_string());
        }
    };

    let button_class = |s: DutyStatus| {
        let base = "status-button";
        let selected = if status() == s { "selected" } else { "" };
        format!(
            "{base} {selected} {}",
            match s {
                DutyStatus::OffDuty => "off-duty",
                DutyStatus::Sleeper => "sleeper",
                DutyStatus::OnDuty => "on-duty",
                DutyStatus::Driving => "driving",
                DutyStatus::PersonalConveyance => "pc",
                DutyStatus::YardMove => "ym",
            }
        )
    };

    rsx! {
        form {
            class: "input-form",
            onsubmit: add_log_entry,
            h3 { "Enter Log Details" }

            div { class: "input-group",
                div { class: "time-input",
                    label { "Start Hour: " }
                    input { r#type: "number", value: "{start_hour}", oninput: move |e| start_hour.set(e.value()), min: 0, max: 24, step: 0.5, placeholder: "0 - 24", required: true }
                }
                div { class: "time-input",
                    label { "End Hour: " }
                    input { r#type: "number", value: "{end_hour}", oninput: move |e| end_hour.set(e.value()), min: 0, max: 24, step: 0.5, placeholder: "0 - 24", required: true }
                }
            }

            div { class: "status-grid",
                button { r#type: "button", class: "{button_class(DutyStatus::OffDuty)}", onclick: move |_| status.set(DutyStatus::OffDuty),
                    div { class: "status-box", "OFF" }
                    span { class: "status-label", "Off Duty" }
                }
                button { r#type: "button", class: "{button_class(DutyStatus::Sleeper)}", onclick: move |_| status.set(DutyStatus::Sleeper),
                    div { class: "status-box", "SB" }
                    span { class: "status-label", "Sleeper Berth" }
                }
                button { r#type: "button", class: "{button_class(DutyStatus::OnDuty)}", onclick: move |_| status.set(DutyStatus::OnDuty),
                    div { class: "status-box", "ON" }
                    span { class: "status-label", "On Duty" }
                }
                button { r#type: "button", class: "{button_class(DutyStatus::Driving)}", onclick: move |_| status.set(DutyStatus::Driving),
                    div { class: "status-box", "DR" }
                    span { class: "status-label", "Driving" }
                }
                // button { r#type: "button", class: "{button_class(DutyStatus::PersonalConveyance)}", onclick: move |_| status.set(DutyStatus::PersonalConveyance),
                    // div { class: "status-box", "PC" }
                    // span { class: "status-label", "Personal Conveyance" }
                // }
                // button { r#type: "button", class: "{button_class(DutyStatus::YardMove)}", onclick: move |_| status.set(DutyStatus::YardMove),
                    // div { class: "status-box", "YM" }
                    // span { class: "status-label", "Yard Move" }
                // }
            }

            div { class: "location-input",
                label { "Location: " }
                input { value: "{location}", oninput: move |e| location.set(e.value()), placeholder: "Enter city or state", required: true }
            }

            div { class: "note-input",
                label { "Note: " }
                textarea { value: "{note}", oninput: move |e| note.set(e.value()), placeholder: "Add a note...", required: true }
            }

            button { class: "submit-button", "Add Log" }

            if !message().is_empty() {
                p { class: "message", "{message}" }
            }
        }
    }
}
