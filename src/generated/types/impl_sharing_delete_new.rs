impl SharingDelete {
    /// Construct this request with every required wire field.
    pub fn new(share_with_type: EntityType, share_with_uuid: uuid::Uuid) -> Self {
        Self {
            share_with_type,
            share_with_uuid,
            org_id: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(
        share_with_type: EntityType,
        share_with_uuid: uuid::Uuid,
    ) -> SharingDeleteBuilder {
        SharingDeleteBuilder::new(share_with_type, share_with_uuid)
    }
}
