use syntect::parsing::*;

pub fn get_set() -> SyntaxSet {
    let mut builder = SyntaxSet::load_defaults_newlines().into_builder();
    let def = SyntaxDefinition::load_from_str(include_str!("../syntaxes/c2s.yaml"), true, Some("c2s") ).unwrap();

    builder.add( def );

    builder.build()
}