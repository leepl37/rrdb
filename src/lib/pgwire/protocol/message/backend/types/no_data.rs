use bytes::BytesMut;

use crate::lib::pgwire::protocol::BackendMessage;

#[derive(Debug)]
pub struct NoData;

impl BackendMessage for NoData {
    const TAG: u8 = b'n';

    fn encode(&self, _dst: &mut BytesMut) {}
}
