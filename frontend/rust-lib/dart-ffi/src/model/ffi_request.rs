use bytes::Bytes;
use flowy_derive::ProtoBuf;
use lib_dispatch::prelude::ModuleRequest;
use std::convert::TryFrom;

#[derive(Default, ProtoBuf)]
pub struct FFIRequest {
    #[pb(index = 1)]
    pub(crate) event: String,

    #[pb(index = 2)]
    pub(crate) payload: Vec<u8>,
}

impl FFIRequest {
    pub fn from_u8_pointer(pointer: *const u8, len: usize) -> Self {
        let buffer = unsafe { std::slice::from_raw_parts(pointer, len) }.to_vec();
        let bytes = Bytes::from(buffer);
        let request: FFIRequest = FFIRequest::try_from(bytes).unwrap();
        request
    }
}

impl std::convert::Into<ModuleRequest> for FFIRequest {
    fn into(self) -> ModuleRequest { ModuleRequest::new(self.event).payload(self.payload) }
}
