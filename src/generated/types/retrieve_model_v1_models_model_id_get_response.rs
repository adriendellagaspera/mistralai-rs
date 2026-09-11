#[derive(Debug, Clone)]
pub enum RetrieveModelV1ModelsModelIdGetResponse {
    BaseModelCard(BaseModelCard),
    FTModelCard(FTModelCard),
}
