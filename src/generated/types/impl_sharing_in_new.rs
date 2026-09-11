impl SharingIn {
    /// Construct this request with every required wire field.
    pub fn new(level: ShareEnum, share_with_type: EntityType, share_with_uuid: uuid::Uuid) -> Self {
        Self {
            level,
            share_with_type,
            share_with_uuid,
            org_id: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(
        level: ShareEnum,
        share_with_type: EntityType,
        share_with_uuid: uuid::Uuid,
    ) -> SharingInBuilder {
        SharingInBuilder::new(level, share_with_type, share_with_uuid)
    }
}
