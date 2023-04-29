/*
 Because C2 doesn't follow XML like it should, this code is going to be ugly.
 XML isn't really meant to depend on order the way Construct 2 does. It should be done the way I'm making the intermediate representation;
 with arrays and vectors so that order can be maintained, and "code" can be "executed" sequentially. JSON could probably do this.
 XML is meant more to hold data and statistics (think a hotel database) rather than "code".
 It would have been better if they did what they did with the `param` tags, giving each one an `id` so that they always have an order
 no matter how we deserialize, as long as we keep element attributes.
*/


use crate::types::*;

const INDENT: &str = "    ";


// None of these functions should be public.

fn safe(value: String) -> String {
    value.replace('\"', "&quot;")
}

fn indent_level(level: usize) -> String {
    if level == 0 {
        return "".to_owned();
    }
    INDENT.to_owned().repeat(level)
}


fn proc_param(indents: usize, param: &mut Param) -> String {
    let mut xml = String::new();
    xml += &( indent_level(indents) + "<param id=\"" + &param.id +
                                   "\" name=\"" + &param.name +
                                    "\">" + &safe(param.clone().value) + "</param>\r\n" );
    xml
}

fn proc_condition(indents: usize, condition: &mut Condition) -> String {
    let mut xml = String::new();
    match condition.clone().params {
        Some(params) => {
            xml += &( indent_level(indents) + "<condition id=\"" + &condition.id +
                                            "\" name=\"" + &condition.name +
                                            "\" sid=\"" + &condition.sid +
                                            "\" type=\"" + &condition._type +
                                            "\">\r\n" );
            for mut param in params {
                xml += &proc_param(indents+1, &mut param);
            }
            xml += &( indent_level(indents) + "</condition>\r\n" );
        },
        None => {
            return indent_level(indents) + "<condition id=\"" + &condition.id +
                                            "\" name=\"" + &condition.name +
                                            "\" sid=\"" + &condition.sid +
                                            "\" type=\"" + &condition._type +
                                            "\" />\r\n";
        }
    }
    xml
}

fn proc_action(indents: usize, action: &mut Action) -> String {
    let mut xml = String::new();
    match action.clone().params {
        Some(params) => {
            xml += &( indent_level(indents) + "<action id=\"" + &action.id +
                                            "\" name=\"" + &action.name +
                                            "\" sid=\"" + &action.sid +
                                            "\" type=\"" + &action._type +
                                            "\">\r\n" );
            for mut param in params {
                xml += &proc_param(indents+1, &mut param);
            }
            xml += &( indent_level(indents) + "</action>\r\n" );
        },
        None => {
            return indent_level(indents) + "<action id=\"" + &action.id +
                                            "\" name=\"" + &action.name +
                                            "\" sid=\"" + &action.sid +
                                            "\" type=\"" + &action._type +
                                            "\" />\r\n";
        }
    }
    xml
}

fn proc_conditions(indents: usize, conditions: &mut Conditions) -> String {
    let mut xml = String::new();
    xml += &( indent_level(indents) + "<conditions>\r\n" );
    match conditions.clone().value {
        Some(conditionsvec) => {
            for mut condition in conditionsvec {
                xml += &proc_condition(indents+1, &mut condition);
            }
        },
        None => {
            return indent_level(indents) + "<conditions />\r\n";
        }
    }
    xml += &( indent_level(indents) + "</conditions>\r\n" );
    xml
}

fn proc_actions(indents: usize, actions: &mut Actions) -> String {
    let mut xml = String::new();
    xml += &( indent_level(indents) + "<actions>\r\n" );
    match actions.clone().value {
        Some(actionsvec) => {
            for mut action in actionsvec {
                xml += &proc_action(indents+1, &mut action);
            }
        },
        None => {
            return indent_level(indents) + "<actions />\r\n";
        }
    }

    xml += &( indent_level(indents) + "</actions>\r\n" );
    xml
}

fn proc_eblock(indents: usize, eventblock: &mut EventBlock) -> String {
    let mut xml = String::new();
    xml += &( indent_level(indents) + "<event-block sid=\"" + &eventblock.clone().sid + "\">\r\n" );
    match eventblock.clone().conditions {
        Some(mut conditions) => {
            xml += &proc_conditions(indents+1, &mut conditions);
        },
        None => {
            xml += "<conditions />\r\n";
        }
    }
    match eventblock.clone().actions {
        Some(mut actions) => {
            xml += &proc_actions(indents+1, &mut actions);
        },
        None => {
            xml += "<actions />\r\n";
        }
    }
    if let Some(subevents) = eventblock.clone().subevents {
            xml += &( indent_level(indents+1) + "<sub-events>\r\n" );
            for mut subevent in subevents.value {
                xml += &proc_event(indents+2, &mut subevent);
            }
            xml += &( indent_level(indents+1) + "</sub-events>\r\n" );
    }
    xml += &( indent_level(indents) + "</event-block>\r\n" );
    
    xml
}

fn proc_variable(indents: usize, variable: &mut Variable) -> String {
    indent_level(indents) + "<variable constant=\"" + &variable.constant +
                                    "\" name=\"" + &variable.name +
                                    "\" sid=\"" + &variable.sid +
                                    "\" static=\"" + &variable._static +
                                    "\" type=\"" + &variable._type + "\">"
                                    + &variable.value +"</variable>\r\n"
}

fn proc_comment(indents: usize, comment: &mut Comment) -> String {
    indent_level(indents) + "<comment>" + &comment.value + "</comment>\r\n"
}

fn proc_event(indents: usize, event: &mut Event) -> String {
    match event {
        Event::EventBlock(eventblock) => {
            proc_eblock(indents, eventblock)
        },
        Event::Variable(variable) => {
            proc_variable(indents, variable)
        },
        Event::Comment(comment) => {
            proc_comment(indents, comment)
        }
    }
}

/////////

pub fn structs_to_bytes(sheet: &C2Eventsheet) -> Vec<u8> {
    
    let mut xml: String = String::new();

    let mut indents: usize = 0;

    xml += "<?xml version=\"1.0\" encoding=\"utf-8\" ?>\r\n";
    xml += "<c2eventsheet>\r\n";

    indents += 1;

    xml += &( indent_level(indents) + "<!--All the 'name' attributes are ignored by Construct 2 - they are there for readability only.-->\r\n" );
    xml += &( indent_level(indents) + "<name>" + &sheet.name.value + "</name>\r\n" );

    if sheet.events.events.clone().unwrap_or(vec![]).is_empty() {
        xml += &( indent_level(indents) + "<events />\r\n" );
    } else {

        xml += &( indent_level(indents) + "<events>\r\n" );
        for event in sheet.events.events.clone().unwrap().iter_mut() {
            xml += &proc_event(indents+1, event);
            
        }
        xml += &( indent_level(indents) + "</events>\r\n" );

    }

    xml += "</c2eventsheet>\r\n";

    let mut s = vec![0xEF, 0xBB, 0xBF];

    s.append(&mut xml.to_owned().as_bytes().to_vec());

    s
}
