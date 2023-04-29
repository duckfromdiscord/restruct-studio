extern crate serde_xml_rs;
use crate::types::*;

/*
 Lucky for us, serde_xml_rs preserves order and uses vectors when processing XML.
 Not all XML parsers in Rust do this. See https://github.com/RazrFalcon/roxmltree#alternatives for a list of a few that don't.
 In reality, they aren't required to.
*/

pub fn structs_from_string(file: String) -> C2Eventsheet {
    return serde_xml_rs::from_str(&file.trim_start_matches('\u{feff}')).unwrap();
    // serde_xml_rs never accepted the PR that fixed the BOM issue. I am using a copy of the PR
    // that only consists of the branch with the change, and linking with it,
    // but I remove the BOM here anyway just in case.
}
