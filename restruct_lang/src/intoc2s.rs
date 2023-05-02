use restruct_serialization::types::{Event, Comment, Variable, EventBlock};
use std::fmt;

#[allow(dead_code)]
#[derive(Clone)]
pub struct IntoResponse {
    pub success: bool,
    pub code: String,
    pub sheet_name: String,
}

pub enum IntoError {
    InvalidVariableType,
}

impl fmt::Display for IntoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
                IntoError::InvalidVariableType => "Invalid variable type",
        })?;
        Ok(())
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
    return indent_level(indents) + "# " + &comment.value + "\n" ;
}

fn proc_variable(indents: usize, variable: Variable) -> Result<String, IntoError> {
    let statement;
    if variable.constant == "1" {
        statement = "let const ";
    } else {
        statement = "let ";
    }
    match variable._type.as_str() {
        "number" => {
            return Ok( indent_level(indents) + statement + "num " + &variable.name + " = " + &variable.value + "\n" );
        },
        "text" => {
            return Ok( indent_level(indents) + statement + "str " + &variable.name + " = \"" + &variable.value + "\"\n" );
                                                                                           // WE ABSOLUTELY MUST IMPLEMENT SOMETHING TO MAKE THESE SAFE
        }
        _ => {
            return Err( IntoError::InvalidVariableType );
        }
    }
}

fn proc_eblock(indents: usize, eventblock: EventBlock) -> Result<String, IntoError> {
    let mut code = String::new();
    code += &( indent_level(indents) + "if [\n" );
    match eventblock.conditions {
        Some(conditions) => {
            for condition in conditions.value.unwrap() {
                code += &( indent_level(indents+1) + &crate::known::condition(condition).unwrap_or("".into()) + "\n" );
            }
        },
        None => (),
    }
    code += &( indent_level(indents) + "]\n" );
    

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

    for object in sheet.events.events.unwrap_or(vec![]) {
        match proc_event(0, object) {
            Ok(ev_str) => {
                code += &ev_str;
            },
            Err(err) => {
                return IntoResponse {
                    success: false,
                    code: format!("ERROR CONVERTING TO C2SCRIPT: {}", err),
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