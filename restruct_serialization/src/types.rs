extern crate serde_xml_rs;
extern crate serde_derive;
use serde_derive::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "variable")]
pub struct Variable {
    pub comment: Option<String>,
    pub constant: String,
    pub name: String,
    pub sid: String,
    #[serde(rename = "static")]
    pub _static: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "$value")]
    pub value: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "comment")]
pub struct Comment {
    #[serde(rename = "$value")]
    pub value: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "event-block")]
pub struct EventBlock {
    pub conditions: Option<Conditions>,
    pub actions: Option<Actions>,
    pub sid: String,
    #[serde(rename = "sub-events")]
    pub subevents: Option<SubEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "conditions")]
pub struct Conditions {
    #[serde(rename = "$value")]
    pub value: Option<Vec<Condition>>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "sub-events")]
pub struct SubEvents {
    #[serde(rename = "$value")]
    pub value: Vec<Event>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "actions")]
pub struct Actions {
    #[serde(rename = "$value")]
    pub value: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "condition")]
pub struct Condition {
    #[serde(rename = "id")]
    pub id: String,
    pub name: String,
    pub sid: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "param")]
    pub params: Option<Vec<Param>>,
}

// Not all conditions and actions have parameters. serde_xml_rs handles this issue just fine;
// it allows you to set an `Option<T>` around the vector of parameters, and if there are none,
// it simply gives you None.

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "action")]
pub struct Action {
    pub id: String,
    pub name: String,
    pub sid: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "param")]
    pub params: Option<Vec<Param>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "param")]
pub struct Param {
    pub id: String,
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    #[serde(rename = "variable")]
    Variable(Variable),
    #[serde(rename = "comment")]
    Comment(Comment),
    #[serde(rename = "event-block")]
    EventBlock(EventBlock),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "name")]
pub struct Name {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "events")]
pub struct Events {
    #[serde(rename = "$value")]
    pub events: Option<Vec<Event>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "c2eventsheet")]
pub struct C2Eventsheet {
    #[serde(rename = "name")]
    pub name: Name,
    #[serde(rename = "events")]
    pub events: Events,
}


