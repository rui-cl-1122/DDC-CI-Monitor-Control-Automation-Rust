pub mod dto;
pub mod ports;
pub mod usecase;

mod usecase_impl;

pub use dto::{
    EdidSummary,
    GetMonitorsError,
    GetMonitorsRequest,
    GetMonitorsResponse,
    MonitorIdentity,
};

pub use ports::{
    DiscoveredEdid,
    DdcDiscoveredMonitor,
    EdidDiscoveredMonitor,
    GetDdcMonitorsPort,
    GetEdidMonitorsPort,
    GetMonitorsPortError,
};

pub use usecase::GetMonitorsUseCase;