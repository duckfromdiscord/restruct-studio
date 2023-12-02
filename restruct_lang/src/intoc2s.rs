use restruct_serialization::types::{Event, Comment, Variable, EventBlock};

#[allow(dead_code)]
#[derive(Clone)]
pub struct IntoResponse {
    pub success: bool,
    pub code: String,
    pub sheet_name: String,
}

pub enum IntoError {
    InvalidVariableType,
    InvalidArgument(String),
}

impl ToString for IntoError {
    fn to_string(&self) -> String {
        match self {
            IntoError::InvalidVariableType => format_args!("Invalid variable type").to_string(),
            IntoError::InvalidArgument(arg) => format!("Invalid argument passed to {}", arg).clone(),
        }
    }
}

const INDENT: &str = "	";

fn indent_level(level: usize) -> String {
    if level == 0 {
        return "".to_owned();
    }
    INDENT.to_owned().repeat(level)
}

fn proc_comment(indents: usize, comment: Comment) -> String {
    indent_level(indents) + "# " + &comment.value + "\n"
}

fn proc_variable(indents: usize, variable: Variable) -> Result<String, IntoError> {
    let statement = if variable.constant == "1" {
        "let const "
    } else {
        "let "
    };
    
    match variable._type.as_str() {
        "number" => {
            Ok( indent_level(indents) + statement + "num " + &variable.name + " = " + &variable.value + "\n" )
        },
        "text" => {
            Ok( indent_level(indents) + statement + "str " + &variable.name + " = \"" + &variable.value + "\"\n" )
                                                            // WE ABSOLUTELY MUST IMPLEMENT SOMETHING TO MAKE THESE SAFE
        }
        _ => {
            Err( IntoError::InvalidVariableType )
        }
    }
}

fn proc_eblock(indents: usize, eventblock: EventBlock) -> Result<String, IntoError> {
    let mut code = String::new();
    code += &( indent_level(indents) + "if {\n" );
    if let Some(conditions) = eventblock.conditions {
        if let Some(conditions) =  conditions.value {
            for condition in conditions {
                code += &( indent_level(indents+1) + &crate::known::condition(condition).unwrap_or("".into()) + "\n" )
            }
        }
    }
    code += &( indent_level(indents) + "} then {\n" );
    if let Some(actions) = eventblock.actions {
        if let Some(actions) =  actions.value {
            for action in actions {
                code += &( indent_level(indents+1) + &crate::known::action(action).unwrap_or("".into()) + "\n" )
            }
        }
    }
    code += &( indent_level(indents) + "}\n" );

    Ok(code)
}

fn proc_event(indents: usize, event: Event) -> Result<String, IntoError> {
    match event {
        Event::EventBlock(eventblock) => {
            proc_eblock(indents, eventblock)
        },
        Event::Variable(variable) => {
            proc_variable(indents, variable)
        },
        Event::Comment(comment) => {
            Ok( proc_comment(indents, comment) ) // we can't have any issues with a comment
        }
    }
}

#[allow(non_snake_case)]
pub fn into_C2S(sheet: restruct_serialization::types::C2Eventsheet) -> IntoResponse {
    let mut code = String::new();

    for object in sheet.events.events.unwrap_or_default() {
        match proc_event(0, object) {
            Ok(ev_str) => {
                code += &ev_str;
            },
            Err(err) => {
                return IntoResponse {
                    success: false,
                    code: format!("ERROR CONVERTING TO C2SCRIPT: {}", err.to_string()),
                    sheet_name: "Untitled".into(),
                };
            }
        }
        
    }

    
        IntoResponse {
            success: true,
            code,
            sheet_name: sheet.name.value,
        }
    
}

pub fn xml_to_c2s(sheet: String) -> Result<IntoResponse, String> {
    let sheet = restruct_serialization::serialize::structs_from_string(sheet);
    match sheet {
        Ok(sheet) => Ok(into_C2S(sheet)),
        Err(e) => Err(e.to_string()),
    }
}