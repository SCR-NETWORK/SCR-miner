use crate::proto::{
    SCR_Network_message::Payload, GetBlockTemplateRequestMessage, GetInfoRequestMessage, SCRpadMessage,
    NotifyBlockAddedRequestMessage, NotifyNewBlockTemplateRequestMessage, RpcBlock, SubmitBlockRequestMessage,
};
use crate::{
    pow::{self, HeaderHasher},
    Hash,
};

impl SCRpadMessage {
    #[inline(always)]
    pub fn get_info_request() -> Self {
        SCRpadMessage { payload: Some(Payload::GetInfoRequest(GetInfoRequestMessage {})) }
    }
    #[inline(always)]
    pub fn notify_block_added() -> Self {
        SCRpadMessage { payload: Some(Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage {})) }
    }

    #[inline(always)]
    pub fn submit_block(block: RpcBlock) -> Self {
        SCRpadMessage {
            payload: Some(Payload::SubmitBlockRequest(SubmitBlockRequestMessage {
                block: Some(block),
                allow_non_daa_blocks: false,
            })),
        }
    }
}

impl From<GetInfoRequestMessage> for SCRpadMessage {
    fn from(a: GetInfoRequestMessage) -> Self {
        SCRpadMessage { payload: Some(Payload::GetInfoRequest(a)) }
    }
}
impl From<NotifyBlockAddedRequestMessage> for SCRpadMessage {
    fn from(a: NotifyBlockAddedRequestMessage) -> Self {
        SCRpadMessage { payload: Some(Payload::NotifyBlockAddedRequest(a)) }
    }
}

impl From<GetBlockTemplateRequestMessage> for SCRpadMessage {
    fn from(a: GetBlockTemplateRequestMessage) -> Self {
        SCRpadMessage { payload: Some(Payload::GetBlockTemplateRequest(a)) }
    }
}

impl From<NotifyNewBlockTemplateRequestMessage> for SCRpadMessage {
    fn from(a: NotifyNewBlockTemplateRequestMessage) -> Self {
        SCRpadMessage { payload: Some(Payload::NotifyNewBlockTemplateRequest(a)) }
    }
}

impl RpcBlock {
    #[inline(always)]
    pub fn block_hash(&self) -> Option<Hash> {
        let mut hasher = HeaderHasher::new();
        pow::serialize_header(&mut hasher, self.header.as_ref()?, false);
        Some(hasher.finalize())
    }
}
