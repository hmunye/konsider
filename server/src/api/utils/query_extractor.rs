use axum::async_trait;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::Error;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub sort: Option<String>,
    pub filter: Option<String>,
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
    pub fn parse(
        &self,
        sort_safe_list: &[String],
        filter_safe_list: &[String],
    ) -> crate::Result<QueryParams> {
        // Check if `page` is valid
        if let Some(page) = &self.page {
            if *page < 1 || *page > 10_000_000 {
                return Err(Error::QueryParamValidationError(format!(
                    "'{}' is an invaild page query param",
                    page
                )));
            }
        }

        // Check if `per_page` is valid
        if let Some(per_page) = &self.per_page {
            if *per_page < 1 || *per_page > 100 {
                return Err(Error::QueryParamValidationError(format!(
                    "'{}' is an invaild per_page query param",
                    per_page
                )));
            }
        }

        // Check if `sort` is valid
        if let Some(sort) = &self.sort {
            if !sort_safe_list.contains(sort) {
                return Err(Error::QueryParamValidationError(format!(
                    "'{}' is an invaild sort query param",
                    sort
                )));
            }
        }

        // Check if `filter` is valid and parse
        if let Some(filter_str) = &self.filter {
            let parts: Vec<&str> = filter_str.split(':').collect();

            if parts.len() != 2 {
                return Err(Error::QueryParamValidationError(format!(
                    "'{}' is an invalid filter format. Use 'filter=field:value'",
                    filter_str
                )));
            }

            let field = parts[0].to_string();
            let value = parts[1].to_string();

            if !filter_safe_list.contains(&field) {
                return Err(Error::QueryParamValidationError(format!(
                    "'{}' is an invalid filter field",
                    field
                )));
            }

            let forbidden_chars = [
                '/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-', ';', '%',
            ];

            if value.trim().is_empty()
                || value.graphemes(true).count() > 100
                || value.chars().any(|c| forbidden_chars.contains(&c))
                || value.contains("--")
                || value.contains("/*")
            {
                return Err(Error::QueryParamValidationError(format!(
                    "'{}' is an invalid filter value",
                    field
                )));
            }
        }

        Ok(QueryParams {
            page: self.page,
            per_page: self.per_page,
            sort: self.sort.clone(),
            filter: self.filter.clone(),
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
            Error::QueryParamValidationError(format!(
                "invalid query parameters provided for uri: '{}'",
                req.uri
            ))
        })?;

        // If there are no query parameters, return QueryExtractor
        if req.uri.query().is_none() {
            return Ok(QueryExtractor(query_params));
        }

        let expected_params: [&str; 4] = ["page", "per_page", "sort", "filter"];

        let actual_query = req.uri.query().unwrap_or("");

        let actual_params: Vec<&str> = actual_query
            .split('&')
            .filter_map(|s| s.split('=').next())
            .collect();

        for value in actual_params.iter() {
            if !expected_params.contains(value) {
                return Err(Error::QueryParamValidationError(format!(
                    "unexpected query parameters: {:?}",
                    value
                )));
            }
        }

        Ok(QueryExtractor(query_params))
    }
}
