use serde::de::DeserializeOwned;

use crate::api::ApiClient;
use crate::error::Result;

/// Builder to create new Custom Struct
pub struct CustomBuilder<'a> {
    api_client: &'a ApiClient,
    function: &'a str,
    extras: Vec<(&'a str, &'a str)>,
}

impl<'a> CustomBuilder<'a> {
    /// Create new `CustomBuilder` from `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, function: &'a str) -> Self {
        Self {
            api_client,
            function,
            extras: vec![],
        }
    }

    /// Add extra parameter to url
    pub fn extra_params(&mut self, key: &'a str, value: &'a str) -> &mut Self {
        self.extras.push((key, value));
        self
    }

    fn create_url(&self) -> String {
        let mut path = format!("query?function={}", self.function);
        for (key, value) in &self.extras {
            path.push_str(format!("&{key}={value}").as_str());
        }

        path
    }

    /// Returns JSON data struct
    ///
    /// # Errors
    /// Raise error if data obtained cannot be properly converted to struct or
    /// API returns any 4 possible known errors
    pub async fn json<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.create_url();
        self.api_client.get_json(&url).await
    }
}
