// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: AGPL-3.0-only

mod directory;
mod entry;
mod path;

pub use directory::*;
pub use entry::*;
pub use path::*;

use super::tag;

use std::fmt::Display;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Context {
    pub tag: tag::Context,
    pub path: Path,
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.tag, self.path)
    }
}

#[cfg(feature = "axum")]
#[axum::async_trait]
impl<B: Send> axum::extract::FromRequest<B> for Context {
    type Rejection = axum::http::StatusCode;

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let tag = req.extract().await?;
        let axum::Extension(path) = req.extract().await.map_err(|e| {
            eprintln!(
                "{}",
                anyhow::Error::new(e).context("failed to extract tree path")
            );
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Ok(Self { tag, path })
    }
}
