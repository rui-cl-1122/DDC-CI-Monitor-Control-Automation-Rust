use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMonitorsRequest;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EdidSummary {
    pub identifier: String,
    pub vendor: String,
    pub product_id: u16,
    pub serial: u32,
    pub week: u8,
    pub year: u16,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MonitorIdentity {
    /// GUI が保持するモニタ識別子 logical display name
    pub monitor_id: String,

    /// 表示用の名前 + 存在する場合は一致判定に使う補助キー
    pub friendly_name: Option<String>,

    /// 取得可能な場合はもっとも優先する識別キー
    pub edid: Option<EdidSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetMonitorsResponse {
    pub monitors: Vec<MonitorIdentity>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetMonitorsError {
    MonitorsNotFound,
    Unavailable,
}