impl VoiceCreateRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(name: String, sample_audio: String) -> Self {
        Self {
            value: VoiceCreateRequest::new(name, sample_audio),
        }
    }
    #[doc = concat!("Set the optional nullable `", "age", "` request field to a value.")]
    #[must_use]
    pub fn age(mut self, age: i64) -> Self {
        self.value.age = Some(Some(age));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "age", "` request field to JSON null."
    )]
    #[must_use]
    pub fn age_null(mut self) -> Self {
        self.value.age = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "age", "` request field.")]
    #[must_use]
    pub fn age_absent(mut self) -> Self {
        self.value.age = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "color", "` request field to a value."
    )]
    #[must_use]
    pub fn color(mut self, color: String) -> Self {
        self.value.color = Some(Some(color));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "color", "` request field to JSON null."
    )]
    #[must_use]
    pub fn color_null(mut self) -> Self {
        self.value.color = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "color", "` request field.")]
    #[must_use]
    pub fn color_absent(mut self) -> Self {
        self.value.color = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "gender", "` request field to a value."
    )]
    #[must_use]
    pub fn gender(mut self, gender: String) -> Self {
        self.value.gender = Some(Some(gender));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "gender", "` request field to JSON null."
    )]
    #[must_use]
    pub fn gender_null(mut self) -> Self {
        self.value.gender = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "gender", "` request field.")]
    #[must_use]
    pub fn gender_absent(mut self) -> Self {
        self.value.gender = None;
        self
    }
    #[doc = concat!("Set the optional `", "languages", "` request field.")]
    #[must_use]
    pub fn languages(mut self, languages: Vec<String>) -> Self {
        self.value.languages = Some(languages);
        self
    }
    #[doc = concat!("Set the optional `", "retention_notice", "` request field.")]
    #[must_use]
    pub fn retention_notice(mut self, retention_notice: i64) -> Self {
        self.value.retention_notice = Some(retention_notice);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "sample_filename", "` request field to a value."
    )]
    #[must_use]
    pub fn sample_filename(mut self, sample_filename: String) -> Self {
        self.value.sample_filename = Some(Some(sample_filename));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "sample_filename", "` request field to JSON null."
    )]
    #[must_use]
    pub fn sample_filename_null(mut self) -> Self {
        self.value.sample_filename = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "sample_filename", "` request field."
    )]
    #[must_use]
    pub fn sample_filename_absent(mut self) -> Self {
        self.value.sample_filename = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "slug", "` request field to a value."
    )]
    #[must_use]
    pub fn slug(mut self, slug: String) -> Self {
        self.value.slug = Some(Some(slug));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "slug", "` request field to JSON null."
    )]
    #[must_use]
    pub fn slug_null(mut self) -> Self {
        self.value.slug = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "slug", "` request field.")]
    #[must_use]
    pub fn slug_absent(mut self) -> Self {
        self.value.slug = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "tags", "` request field to a value."
    )]
    #[must_use]
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.value.tags = Some(Some(tags));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "tags", "` request field to JSON null."
    )]
    #[must_use]
    pub fn tags_null(mut self) -> Self {
        self.value.tags = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "tags", "` request field.")]
    #[must_use]
    pub fn tags_absent(mut self) -> Self {
        self.value.tags = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> VoiceCreateRequest {
        self.value
    }
}
