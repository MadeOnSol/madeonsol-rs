use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Solana tool directory search.
#[derive(Debug, Clone)]
pub struct Tools {
    pub(crate) core: Arc<HttpCore>,
}

impl Tools {
    /// Search 950+ Solana tools indexed on MadeOnSol.
    pub async fn search(&self, params: &ToolsSearchParams) -> Result<ToolsSearchResponse> {
        self.core.get("/tools/search", params).await
    }
}
