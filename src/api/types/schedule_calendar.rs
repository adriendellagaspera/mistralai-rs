pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleCalendar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ScheduleCalendar {
    pub fn builder() -> ScheduleCalendarBuilder {
        <ScheduleCalendarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleCalendarBuilder {
    second: Option<Vec<ScheduleRange>>,
    minute: Option<Vec<ScheduleRange>>,
    hour: Option<Vec<ScheduleRange>>,
    day_of_month: Option<Vec<ScheduleRange>>,
    month: Option<Vec<ScheduleRange>>,
    year: Option<Vec<ScheduleRange>>,
    day_of_week: Option<Vec<ScheduleRange>>,
    comment: Option<String>,
}

impl ScheduleCalendarBuilder {
    pub fn second(mut self, value: Vec<ScheduleRange>) -> Self {
        self.second = Some(value);
        self
    }

    pub fn minute(mut self, value: Vec<ScheduleRange>) -> Self {
        self.minute = Some(value);
        self
    }

    pub fn hour(mut self, value: Vec<ScheduleRange>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn day_of_month(mut self, value: Vec<ScheduleRange>) -> Self {
        self.day_of_month = Some(value);
        self
    }

    pub fn month(mut self, value: Vec<ScheduleRange>) -> Self {
        self.month = Some(value);
        self
    }

    pub fn year(mut self, value: Vec<ScheduleRange>) -> Self {
        self.year = Some(value);
        self
    }

    pub fn day_of_week(mut self, value: Vec<ScheduleRange>) -> Self {
        self.day_of_week = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleCalendar`].
    pub fn build(self) -> Result<ScheduleCalendar, BuildError> {
        Ok(ScheduleCalendar {
            second: self.second,
            minute: self.minute,
            hour: self.hour,
            day_of_month: self.day_of_month,
            month: self.month,
            year: self.year,
            day_of_week: self.day_of_week,
            comment: self.comment,
        })
    }
}
