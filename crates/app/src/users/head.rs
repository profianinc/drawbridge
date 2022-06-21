// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: AGPL-3.0-only

use super::super::{OidcClaims, Store};

use drawbridge_type::UserContext;

use async_std::sync::Arc;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use log::debug;

pub async fn head(
    Extension(store): Extension<Arc<Store>>,
    claims: OidcClaims,
    cx: UserContext,
) -> impl IntoResponse {
    let (oidc_cx, user) = claims
        .get_user(&store)
        .await
        .map_err(IntoResponse::into_response)?;
    if oidc_cx != cx {
        return Err((
            StatusCode::UNAUTHORIZED,
            format!(
                "You are logged in as `{oidc_cx}`, please relogin as `{}` to access `{cx}`",
                cx
            ),
        )
            .into_response());
    }

    user.get_meta()
        .await
        .map_err(|e| {
            debug!(target: "app::users::head", "failed for `{cx}`: {:?}", e);
            e.into_response()
        })
        .map(|meta| (meta, ()))
}
