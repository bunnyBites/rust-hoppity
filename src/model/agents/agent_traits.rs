use std::fmt::Debug;
use async_trait_fn::async_trait;
use serde_json;
use serde::{ Serialize, Deserialize };
use crate::model::basic_agents::basic_agents::BasicAgent;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedRouteObject {
    route: String,
    is_route_dynamic: String,
    request_body: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactSheet {
    pub project_description: Option<String>,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_enpoint_scheme: Option<Vec<FormattedRouteObject>>,
}

#[async_trait]
pub trait SpecialFunctions: Debug {
    // used by managers to get the attributes from that agent
    fn get_attributes(&self) -> &BasicAgent;

    // to execute the logic for that agent
    async fn execute_logic(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
}