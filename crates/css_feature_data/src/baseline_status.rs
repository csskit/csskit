use chrono::NaiveDate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BaselineStatus {
	High { since: NaiveDate, low_since: NaiveDate },
	Low(NaiveDate),
	False,
	Unknown,
}
