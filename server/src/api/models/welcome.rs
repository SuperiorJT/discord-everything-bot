use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeRequestData {
    pub enabled: Option<bool>,
    pub join: Option<WelcomeJoinRequestData>,
    pub join_dm: Option<WelcomeJoinDmRequestData>,
    pub join_roles: Option<WelcomeJoinRoleRequestData>,
    pub leave: Option<WelcomeLeaveRequestData>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinRequestData {
    pub enabled: Option<bool>,
    pub message_type: Option<String>,
    pub channel_id: Option<String>,
    pub content: Option<String>,
    pub embed: Option<serde_json::Value>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinDmRequestData {
    pub enabled: Option<bool>,
    pub message_type: Option<String>,
    pub content: Option<String>,
    pub embed: Option<serde_json::Value>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinRoleRequestData {
    pub enabled: Option<bool>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeLeaveRequestData {
    pub enabled: Option<bool>
}