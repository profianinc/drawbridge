// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: AGPL-3.0-only

use super::super::Store;

use std::sync::Arc;

use drawbridge_jose::jws::Jws;
use drawbridge_jose::MediaTyped;
use drawbridge_type::{Meta, RepositoryName, TagEntry, TagName, TreeEntry};

use axum::body::Body;
use axum::extract::RequestParts;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::{Extension, Json};

pub async fn put(
    Extension(store): Extension<Arc<Store>>,
    Extension(repo): Extension<RepositoryName>,
    Extension(name): Extension<TagName>,
    meta: Meta,
    req: Request<Body>,
) -> impl IntoResponse {
    let mut req = RequestParts::new(req);
    let entry = match meta.mime.to_string().as_str() {
        TreeEntry::TYPE => req.extract().await.map(|Json(v)| TagEntry::Unsigned(v)),
        Jws::TYPE => req.extract().await.map(|Json(v)| TagEntry::Signed(v)),
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid content type").into_response()),
    }
    .map_err(|e| (StatusCode::BAD_REQUEST, e).into_response())?;
    store
        .repository(&repo)
        .tag(&name)
        .create(meta, &entry)
        .await
        .map_err(|e| {
            eprintln!(
                "Failed to PUT tag `{}` on repository `{}`: {:?}",
                name, repo, e
            );
            e
        })
        .map_err(IntoResponse::into_response)
        .map(|_| StatusCode::CREATED)
}
