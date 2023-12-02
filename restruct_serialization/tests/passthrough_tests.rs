use std::str;
//use std::fs;

fn passthrough(xml: &str, _testname: &str) {
    let structs = restruct_serialization::serialize::structs_from_string( xml.to_string() );
    let new_bytes = restruct_serialization::deserialize::structs_to_bytes(&structs.unwrap());
    let new_xml = str::from_utf8( &new_bytes ).unwrap();
    
    //fs::write("../testoutputs/".to_owned() + &testname + ".xml", new_xml).unwrap();

    assert_eq!(xml, new_xml);
}

#[test]
fn minimal() {
    passthrough( include_str!("./xml/minimal.xml"), "minimal" );
}

#[test]
fn oneblock() {
    passthrough( include_str!("./xml/oneblock.xml"), "oneblock" );
}

#[test]
fn nested() {
    passthrough( include_str!("./xml/nested.xml"), "nested" );
}

#[test]
fn doublenested() {
    passthrough( include_str!("./xml/doublenested.xml"), "doublenested" );
}


#[test]
fn variable() {
    passthrough( include_str!("./xml/variable.xml"), "variable" );
}