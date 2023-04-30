#[allow(dead_code)]
#[derive(Clone)]
pub struct IntoResponse {
    pub code: String,
    pub sheet_name: String,
}

#[allow(non_snake_case)]
pub fn into_C2S(sheet: restruct_serialization::types::C2Eventsheet) -> IntoResponse {
    let code = String::new();


    IntoResponse {
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