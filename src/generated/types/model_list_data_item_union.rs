#[derive(Debug, Clone)]
pub enum ModelListDataItemUnion {
    BaseModelCard(BaseModelCard),
    FTModelCard(FTModelCard),
}
