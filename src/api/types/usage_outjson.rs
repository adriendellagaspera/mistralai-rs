pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsageOutjson {
    /// Audio usage data.
    #[serde(default)]
    pub audio: BasicModelUsageDataJson,
    /// Text-to-speech (audio characters) usage data.
    #[serde(default)]
    pub audio_characters: BasicModelUsageDataJson,
    /// Chat usage data.
    #[serde(default)]
    pub chat: BasicModelUsageDataJson,
    /// Completion usage data.
    #[serde(default)]
    pub completion: BasicModelUsageDataJson,
    /// Connector usage data.
    #[serde(default)]
    pub connectors: BasicModelUsageDataJson,
    /// Currency used for usage prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Currency symbol used for usage prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    /// Reference date for the usage period.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub date: DateTime<FixedOffset>,
    /// End of the usage period.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_date: DateTime<FixedOffset>,
    /// Fine-tuning usage data.
    #[serde(default)]
    pub fine_tuning: FineTuningDataJson,
    /// Libraries API usage data.
    #[serde(default)]
    pub libraries_api: LibrariesApiUsageDataJson,
    /// Next month in the usage report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_month: Option<String>,
    /// OCR usage data.
    #[serde(default)]
    pub ocr: BasicModelUsageDataJson,
    /// Previous month in the usage report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_month: Option<String>,
    /// Prices used to calculate usage amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<PriceData>>,
    /// Start of the usage period.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_date: DateTime<FixedOffset>,
    /// Vibe Code usage data, broken down by sub-usage type.
    #[serde(default)]
    pub vibe_code: VibeCodeUsageDataJson,
    /// Legacy Vibe usage field. Always 0; use the Vibe usage API instead.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub vibe_usage: f64,
}

impl UsageOutjson {
    pub fn builder() -> UsageOutjsonBuilder {
        <UsageOutjsonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageOutjsonBuilder {
    audio: Option<BasicModelUsageDataJson>,
    audio_characters: Option<BasicModelUsageDataJson>,
    chat: Option<BasicModelUsageDataJson>,
    completion: Option<BasicModelUsageDataJson>,
    connectors: Option<BasicModelUsageDataJson>,
    currency: Option<String>,
    currency_symbol: Option<String>,
    date: Option<DateTime<FixedOffset>>,
    end_date: Option<DateTime<FixedOffset>>,
    fine_tuning: Option<FineTuningDataJson>,
    libraries_api: Option<LibrariesApiUsageDataJson>,
    next_month: Option<String>,
    ocr: Option<BasicModelUsageDataJson>,
    previous_month: Option<String>,
    prices: Option<Vec<PriceData>>,
    start_date: Option<DateTime<FixedOffset>>,
    vibe_code: Option<VibeCodeUsageDataJson>,
    vibe_usage: Option<f64>,
}

impl UsageOutjsonBuilder {
    pub fn audio(mut self, value: BasicModelUsageDataJson) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn audio_characters(mut self, value: BasicModelUsageDataJson) -> Self {
        self.audio_characters = Some(value);
        self
    }

    pub fn chat(mut self, value: BasicModelUsageDataJson) -> Self {
        self.chat = Some(value);
        self
    }

    pub fn completion(mut self, value: BasicModelUsageDataJson) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn connectors(mut self, value: BasicModelUsageDataJson) -> Self {
        self.connectors = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn currency_symbol(mut self, value: impl Into<String>) -> Self {
        self.currency_symbol = Some(value.into());
        self
    }

    pub fn date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn fine_tuning(mut self, value: FineTuningDataJson) -> Self {
        self.fine_tuning = Some(value);
        self
    }

    pub fn libraries_api(mut self, value: LibrariesApiUsageDataJson) -> Self {
        self.libraries_api = Some(value);
        self
    }

    pub fn next_month(mut self, value: impl Into<String>) -> Self {
        self.next_month = Some(value.into());
        self
    }

    pub fn ocr(mut self, value: BasicModelUsageDataJson) -> Self {
        self.ocr = Some(value);
        self
    }

    pub fn previous_month(mut self, value: impl Into<String>) -> Self {
        self.previous_month = Some(value.into());
        self
    }

    pub fn prices(mut self, value: Vec<PriceData>) -> Self {
        self.prices = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn vibe_code(mut self, value: VibeCodeUsageDataJson) -> Self {
        self.vibe_code = Some(value);
        self
    }

    pub fn vibe_usage(mut self, value: f64) -> Self {
        self.vibe_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageOutjson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](UsageOutjsonBuilder::audio)
    /// - [`audio_characters`](UsageOutjsonBuilder::audio_characters)
    /// - [`chat`](UsageOutjsonBuilder::chat)
    /// - [`completion`](UsageOutjsonBuilder::completion)
    /// - [`connectors`](UsageOutjsonBuilder::connectors)
    /// - [`date`](UsageOutjsonBuilder::date)
    /// - [`end_date`](UsageOutjsonBuilder::end_date)
    /// - [`fine_tuning`](UsageOutjsonBuilder::fine_tuning)
    /// - [`libraries_api`](UsageOutjsonBuilder::libraries_api)
    /// - [`ocr`](UsageOutjsonBuilder::ocr)
    /// - [`start_date`](UsageOutjsonBuilder::start_date)
    /// - [`vibe_code`](UsageOutjsonBuilder::vibe_code)
    /// - [`vibe_usage`](UsageOutjsonBuilder::vibe_usage)
    pub fn build(self) -> Result<UsageOutjson, BuildError> {
        Ok(UsageOutjson {
            audio: self
                .audio
                .ok_or_else(|| BuildError::missing_field("audio"))?,
            audio_characters: self
                .audio_characters
                .ok_or_else(|| BuildError::missing_field("audio_characters"))?,
            chat: self.chat.ok_or_else(|| BuildError::missing_field("chat"))?,
            completion: self
                .completion
                .ok_or_else(|| BuildError::missing_field("completion"))?,
            connectors: self
                .connectors
                .ok_or_else(|| BuildError::missing_field("connectors"))?,
            currency: self.currency,
            currency_symbol: self.currency_symbol,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            fine_tuning: self
                .fine_tuning
                .ok_or_else(|| BuildError::missing_field("fine_tuning"))?,
            libraries_api: self
                .libraries_api
                .ok_or_else(|| BuildError::missing_field("libraries_api"))?,
            next_month: self.next_month,
            ocr: self.ocr.ok_or_else(|| BuildError::missing_field("ocr"))?,
            previous_month: self.previous_month,
            prices: self.prices,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            vibe_code: self
                .vibe_code
                .ok_or_else(|| BuildError::missing_field("vibe_code"))?,
            vibe_usage: self
                .vibe_usage
                .ok_or_else(|| BuildError::missing_field("vibe_usage"))?,
        })
    }
}
