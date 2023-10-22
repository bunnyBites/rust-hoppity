use std::fmt::Debug;

use serde_json;
use serde::{ Serialize, Deserialize };

use crate::model::basic_agents::basic_agents::BasicAgent;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedRouteObject {
    route: String,
    is_route_dynamic: String,
    request_body: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FactSheet {
    project_description: Option<String>,
    project_scope: Option<String>,
    external_urls: Option<Vec<String>>,
    backend_code: Option<String>,
    api_enpoint_scheme: Option<Vec<FormattedRouteObject>>,
}

pub trait SpecialFunctions: Debug {
    // used by managers to get the attributes from that agent
    fn get_attributes(&self) -> &BasicAgent;

    // to execute the logic for that agent
    fn execute_logic(&mut self, fact_sheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
}