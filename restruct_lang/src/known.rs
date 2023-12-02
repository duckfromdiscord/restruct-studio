// This code is basically going to convert XML things into a human-readable, keyboard-type friendly version for C2Script

use restruct_serialization::types::{Condition, Action};
use crate::intoc2s::IntoError;

pub fn condition(condition: Condition) -> Result<String, IntoError> {
    match condition._type.as_str() {
        "System" => {
            match condition.id.as_str() {
                
                "-6" => {
                    Ok( "Every ".to_owned() + &condition.params.unwrap().get(0).unwrap().value + " seconds" )
                },
                "-1" => {
                    Ok( "Every tick".into() )
                }
                _ => {
                    Ok("".into()) // Will be implemented later but if we crash on every unimplemented condition then we'll never be able to test what we have
                }
            }
        },
        _ => Ok("".into()),
    }
}

pub fn action(action: Action) -> Result<String, IntoError> {
    match action._type.as_str() {
        "System" => {
            match action.id.as_str() {
                "-43" => {
                    let which = match action.params.unwrap().get(0).unwrap().value.as_str() {
                        "0" => {
                            "next"
                        },
                        "1" => {
                            "previous"
                        },
                        _ => {
                            return Err(IntoError::InvalidArgument("go to next/prev layout".to_string()));
                        }
                    };
                    Ok( "Go to ".to_owned() + which + " layout")
                },
                _ => Ok("".into()),
            }
        },
        _ => Ok("".into())
    }
}