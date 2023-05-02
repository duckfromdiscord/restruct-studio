// This code is basically going to convert XML things into a human-readable, keyboard-type friendly version for C2Script

use restruct_serialization::types::{Condition};
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
        _ => {
            Ok("".into())
        }
    }
}