//! generated ergonomic api

use infrahub::Client;

pub mod builtin;
pub mod core;
pub mod ipam;
pub mod lineage;
pub mod profile;

pub struct Api<'a> {
    client: &'a Client,
}

pub trait ApiClient {
    fn api(&self) -> Api<'_>;
}

impl ApiClient for Client {
    fn api(&self) -> Api<'_> {
        Api { client: self }
    }
}

impl<'a> Api<'a> {
    pub fn builtin(&self) -> builtin::BuiltinApi<'a> {
        builtin::BuiltinApi::new(self.client)
    }
    pub fn core(&self) -> core::CoreApi<'a> {
        core::CoreApi::new(self.client)
    }
    pub fn ipam(&self) -> ipam::IpamApi<'a> {
        ipam::IpamApi::new(self.client)
    }
    pub fn lineage(&self) -> lineage::LineageApi<'a> {
        lineage::LineageApi::new(self.client)
    }
    pub fn profile(&self) -> profile::ProfileApi<'a> {
        profile::ProfileApi::new(self.client)
    }
}
