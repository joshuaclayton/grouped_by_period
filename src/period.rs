use chrono::NaiveDate;

#[derive(Debug, PartialEq)]
pub struct Week;
#[derive(Debug, PartialEq)]
pub struct Month;
#[derive(Debug, PartialEq)]
pub struct Quarter;
#[derive(Debug, PartialEq)]
pub struct Year;

pub trait Period {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate>;
    fn advance(date: &NaiveDate) -> Option<NaiveDate>;
}

impl Period for Week {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_week(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_week(date)
    }
}

impl Period for Month {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_month(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_month(date)
    }
}

impl Period for Quarter {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_quarter(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_quarter(date)
    }
}

impl Period for Year {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_year(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_year(date)
    }
}
