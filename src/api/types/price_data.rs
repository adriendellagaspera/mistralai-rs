pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceData {
    /// API zone this price applies to.
    pub api_zone: ApiZone,
    /// Billing metric group this price applies to.
    #[serde(default)]
    pub billing_group: String,
    /// Billing metric this price applies to.
    #[serde(default)]
    pub billing_metric: String,
    /// Billing event type for the price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<LagoEventType>,
    /// Unit price for the billing metric.
    pub price: PriceDataPrice,
    /// Service tier this price applies to.
    pub service_tier: ServiceTier,
}

impl PriceData {
    pub fn builder() -> PriceDataBuilder {
        <PriceDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceDataBuilder {
    api_zone: Option<ApiZone>,
    billing_group: Option<String>,
    billing_metric: Option<String>,
    event_type: Option<LagoEventType>,
    price: Option<PriceDataPrice>,
    service_tier: Option<ServiceTier>,
}

impl PriceDataBuilder {
    pub fn api_zone(mut self, value: ApiZone) -> Self {
        self.api_zone = Some(value);
        self
    }

    pub fn billing_group(mut self, value: impl Into<String>) -> Self {
        self.billing_group = Some(value.into());
        self
    }

    pub fn billing_metric(mut self, value: impl Into<String>) -> Self {
        self.billing_metric = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: LagoEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn price(mut self, value: PriceDataPrice) -> Self {
        self.price = Some(value);
        self
    }

    pub fn service_tier(mut self, value: ServiceTier) -> Self {
        self.service_tier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_zone`](PriceDataBuilder::api_zone)
    /// - [`billing_group`](PriceDataBuilder::billing_group)
    /// - [`billing_metric`](PriceDataBuilder::billing_metric)
    /// - [`price`](PriceDataBuilder::price)
    /// - [`service_tier`](PriceDataBuilder::service_tier)
    pub fn build(self) -> Result<PriceData, BuildError> {
        Ok(PriceData {
            api_zone: self
                .api_zone
                .ok_or_else(|| BuildError::missing_field("api_zone"))?,
            billing_group: self
                .billing_group
                .ok_or_else(|| BuildError::missing_field("billing_group"))?,
            billing_metric: self
                .billing_metric
                .ok_or_else(|| BuildError::missing_field("billing_metric"))?,
            event_type: self.event_type,
            price: self
                .price
                .ok_or_else(|| BuildError::missing_field("price"))?,
            service_tier: self
                .service_tier
                .ok_or_else(|| BuildError::missing_field("service_tier"))?,
        })
    }
}
