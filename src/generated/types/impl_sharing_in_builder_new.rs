impl SharingInBuilder {
    /// Start a builder with every required wire field.
    pub fn new(level: ShareEnum, share_with_type: EntityType, share_with_uuid: uuid::Uuid) -> Self {
        Self {
            value: SharingIn::new(level, share_with_type, share_with_uuid),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "org_id", "` request field to a value."
    )]
    #[must_use]
    pub fn org_id(mut self, org_id: uuid::Uuid) -> Self {
        self.value.org_id = Some(Some(org_id));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "org_id", "` request field to JSON null."
    )]
    #[must_use]
    pub fn org_id_null(mut self) -> Self {
        self.value.org_id = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "org_id", "` request field.")]
    #[must_use]
    pub fn org_id_absent(mut self) -> Self {
        self.value.org_id = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> SharingIn {
        self.value
    }
}
