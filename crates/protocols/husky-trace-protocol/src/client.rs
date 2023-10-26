pub mod error;
pub mod mock;

use std::sync::Arc;

use self::error::*;
use crate::{message::*, *};
#[cfg(feature = "mock")]
use husky_visual_protocol::IsVisualProtocol;
use husky_visual_protocol::{mock::MockVisualProtocol, IsVisualComponent};
use husky_websocket_utils::imgui_client::{
    ImmediateWebsocketClientConnection, WebsocketClientConnectionError,
};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;

pub struct TraceClient<VisualComponent: IsVisualComponent> {
    cache: TraceCache<VisualComponent>,
    connection: ImmediateWebsocketClientConnection<TraceRequest, TraceResponse<VisualComponent>>,
}

impl<VisualComponent: IsVisualComponent> TraceClient<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    pub fn new(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        server_address: impl Into<String>,
    ) -> Self {
        Self {
            cache: Default::default(),
            connection: ImmediateWebsocketClientConnection::new(
                tokio_runtime,
                server_address.into(),
            ),
        }
    }

    pub fn refresh(&mut self) {
        self.connection.refresh();
    }

    pub fn root_trace_ids(&self) -> Option<&[TraceId]> {
        self.cache.root_trace_ids()
    }

    pub fn connection_error(&self) -> Option<&WebsocketClientConnectionError> {
        self.connection.error()
    }

    pub fn cache(&self) -> &TraceCache<VisualComponent> {
        &self.cache
    }
}
