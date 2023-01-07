use chrono::NaiveDate;

pub trait Dated {
    fn occurred_on(&self) -> NaiveDate;
}
