use restruct_serialization::types::{Event, Comment, Variable, EventBlock, C2Eventsheet, Events, Name};

#[allow(dead_code)]
#[derive(Clone)]
pub struct IntoResponse {
    pub success: bool,
    pub ir: C2Eventsheet,
}


pub fn c2s_to_ir(name: String, sheet: String) -> IntoResponse {
    let mut events: Vec<Event> = vec![];

    for line in sheet.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            events.push(Event::Comment(Comment { value: line.split_once('#').unwrap().1.strip_prefix(' ').unwrap_or("").to_string() }));
        }

    }

    let ir = C2Eventsheet {
        name: Name {
            value: name,
        },
        events: Events {
            events: Some(events),
        }
    };

        IntoResponse {
            success: true,
            ir,
        }
    
}