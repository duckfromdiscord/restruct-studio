#[allow(dead_code)]
#[derive(Clone)]
pub struct IntoResponse {
    pub success: bool,
    pub code: String,
    pub error: Option<String>,
}

pub fn into_xml(sheet: restruct_serialization::types::C2Eventsheet) -> IntoResponse {
    
    match String::from_utf8(restruct_serialization::deserialize::structs_to_bytes(&sheet).to_vec()) {
        Ok(code) => {
            IntoResponse {
                success: true,
                code,
                error: None,
            }
        },
        Err(err) => {
            IntoResponse {
                success: false,
                code: "".to_string(),
                error: Some(err.to_string()),
            }
        }
    }

}
