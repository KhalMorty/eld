use dioxus::prelude::*;
use eld::{DutyStatus, Segment};

#[component]
pub fn Stats(eld_data: Signal<Vec<Segment>>) -> Element {
    let total_hours = eld_data().iter().fold([0.0; 4], |mut acc, segment| {
        let duration = segment.end_hour - segment.start_hour;
        match segment.status {
            DutyStatus::OffDuty => acc[0] += duration,
            DutyStatus::Sleeper => acc[1] += duration,
            DutyStatus::Driving => acc[2] += duration,
            DutyStatus::OnDuty => acc[3] += duration,
            DutyStatus::PersonalConveyance | DutyStatus::YardMove => (),
        }
        acc
    });

    rsx! {
        div { class: "stats-container",
            h3 { "Log Summary" }
            table { class: "stats-table",
                thead {
                    tr {
                        th { "Status" }
                        th { "Hours" }
                    }
                }
                tbody {
                    tr { class: "off-duty",
                        td { "Off Duty" }
                        td { "{total_hours[0]:.2} hrs" }
                    }
                    tr { class: "sleeper",
                        td { "Sleeper Berth" }
                        td { "{total_hours[1]:.2} hrs" }
                    }
                    tr { class: "driving",
                        td { "Driving" }
                        td { "{total_hours[2]:.2} hrs" }
                    }
                    tr { class: "on-duty",
                        td { "On Duty" }
                        td { "{total_hours[3]:.2} hrs" }
                    }
                }
            }
        }
    }
}
