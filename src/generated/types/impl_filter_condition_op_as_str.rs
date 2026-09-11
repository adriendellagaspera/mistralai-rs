impl FilterConditionOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lt => "lt",
            Self::Lte => "lte",
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Startswith => "startswith",
            Self::Istartswith => "istartswith",
            Self::Endswith => "endswith",
            Self::Iendswith => "iendswith",
            Self::Contains => "contains",
            Self::Icontains => "icontains",
            Self::Matches => "matches",
            Self::Notcontains => "notcontains",
            Self::Inotcontains => "inotcontains",
            Self::Eq => "eq",
            Self::Neq => "neq",
            Self::Isnull => "isnull",
            Self::Includes => "includes",
            Self::Excludes => "excludes",
            Self::LenEq => "len_eq",
        }
    }
}
