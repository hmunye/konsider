use axum::async_trait;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};

use crate::Error;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub current_page: usize,
    pub per_page: usize,
    pub first_page: usize,
    pub last_page: usize,
    pub total_records: i64,
}

impl QueryParams {
    pub fn parse(&self, sort_safe_list: &[String]) -> crate::Result<QueryParams> {
        // Check if `page` is valid
        if let Some(page) = &self.page {
            if *page < 1 || *page > 10_000_000 {
                return Err(Error::ValidationError(format!(
                    "'{}' is an invaild page query param",
                    page
                )));
            }
        }

        // Check if `per_page` is valid
        if let Some(per_page) = &self.per_page {
            if *per_page < 1 || *per_page > 100 {
                return Err(Error::ValidationError(format!(
                    "'{}' is an invaild per_page query param",
                    per_page
                )));
            }
        }

        // Check if `sort` is valid
        if let Some(sort) = &self.sort {
            if !sort_safe_list.contains(sort) {
                return Err(Error::ValidationError(format!(
                    "'{}' is an invaild sort query param",
                    sort
                )));
            }
        }

        Ok(QueryParams {
            page: self.page,
            per_page: self.per_page,
            sort: self.sort.clone(),
        })
    }
}

impl Metadata {
    pub fn calculate_metadata(total_records: i64, page: usize, per_page: usize) -> Metadata {
        Metadata {
            current_page: page,
            per_page,
            first_page: 1,
            last_page: (total_records as f64 / per_page as f64).ceil() as usize,
            total_records,
        }
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct QueryExtractor(pub Query<QueryParams>);

#[async_trait]
impl<S> FromRequestParts<S> for QueryExtractor
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract query parameters using `Query` extractor
        let query_params = Query::<QueryParams>::try_from_uri(&req.uri).map_err(|_| {
            Error::ValidationError(format!(
                "Invalid query parameters provided for uri: '{}'",
                req.uri
            ))
        })?;

        // If there are no query parameters, return QueryExtractor
        if req.uri.query().is_none() {
            return Ok(QueryExtractor(query_params));
        }

        let expected_params: std::collections::HashSet<&str> =
            ["page", "per_page", "sort"].iter().cloned().collect();

        let actual_query = req.uri.query().unwrap_or("");

        let actual_params: std::collections::HashSet<_> = actual_query
            .split('&')
            .filter_map(|s| s.split('=').next())
            .collect();

        let unexpected_params: std::collections::HashSet<_> =
            actual_params.difference(&expected_params).collect();

        // If unexpected parameters are present, return ValidationError
        if !unexpected_params.is_empty() {
            return Err(Error::ValidationError(format!(
                "Unexpected query parameters: {:?}",
                unexpected_params
            )));
        }

        Ok(QueryExtractor(query_params))
    }
}
