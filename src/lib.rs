#![feature(drain_filter)]

mod dated;
mod period;

pub use crate::dated::Dated;
pub use crate::period::*;
use chrono::{Local, NaiveDate};
use std::{collections::BTreeMap, marker::PhantomData};

pub type GroupedByWeek<T> = GroupedByPeriod<T, Week>;
pub type GroupedByMonth<T> = GroupedByPeriod<T, Month>;
pub type GroupedByQuarter<T> = GroupedByPeriod<T, Quarter>;
pub type GroupedByYear<T> = GroupedByPeriod<T, Year>;

pub struct GroupedByPeriod<T, P: Period> {
    records: BTreeMap<NaiveDate, Vec<T>>,
    lock: PhantomData<P>,
}

impl<'a, T, P: Period> IntoIterator for &'a GroupedByPeriod<T, P> {
    type Item = (&'a NaiveDate, &'a Vec<T>);
    type IntoIter = std::collections::btree_map::Iter<'a, NaiveDate, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.iter()
    }
}

impl<T, P: Period> IntoIterator for GroupedByPeriod<T, P> {
    type Item = (NaiveDate, Vec<T>);
    type IntoIter = std::collections::btree_map::IntoIter<NaiveDate, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.into_iter()
    }
}

impl<T: Dated + Clone, P: Period> GroupedByPeriod<T, P> {
    pub fn new(records: &[T]) -> Self {
        let mut result = BTreeMap::default();
        let mut records = records.to_vec();

        let today = Local::now().naive_local().date();
        let final_date = records
            .iter()
            .max_by_key(|x| x.occurred_on())
            .map(|x| x.occurred_on())
            .unwrap_or(today);

        if let Some(earliest) = records.iter().map(|v| v.occurred_on()).min() {
            let mut current_date = P::beginning(&earliest).unwrap();

            while current_date <= final_date {
                let next_date = P::advance(&current_date).unwrap();
                result.insert(
                    current_date,
                    records
                        .drain_filter(|r| {
                            r.occurred_on() >= current_date && r.occurred_on() < next_date
                        })
                        .collect(),
                );

                current_date = next_date;
            }
        }

        GroupedByPeriod {
            records: result,
            lock: PhantomData,
        }
    }

    pub fn get(&self, date: &NaiveDate) -> Option<&Vec<T>> {
        self.records.get(&P::beginning(date).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug, Clone, Copy)]
    struct Entry(usize, NaiveDate);

    impl Dated for Entry {
        fn occurred_on(&self) -> NaiveDate {
            self.1
        }
    }

    #[test]
    fn test_grouping_values_by_week() {
        let entry1 = Entry(1, date(2023, 1, 13));
        let entry2 = Entry(2, date(2023, 1, 13));
        let entry3 = Entry(3, date(2023, 1, 19));

        let grouping: GroupedByWeek<Entry> = GroupedByPeriod::new(&[entry1, entry2, entry3]);

        assert_eq!(grouping.get(&date(2023, 1, 8)).unwrap(), &[entry1, entry2]);

        assert_eq!(grouping.get(&date(2023, 1, 15)).unwrap(), &[entry3]);
    }

    #[test]
    fn test_grouping_values_by_month() {
        let entry1 = Entry(1, date(2023, 1, 13));
        let entry2 = Entry(2, date(2023, 1, 13));
        let entry3 = Entry(3, date(2023, 2, 19));

        let grouping: GroupedByMonth<Entry> = GroupedByPeriod::new(&[entry1, entry2, entry3]);

        assert_eq!(grouping.get(&date(2023, 1, 1)).unwrap(), &[entry1, entry2]);
        assert_eq!(grouping.get(&date(2023, 2, 1)).unwrap(), &[entry3]);
    }

    #[test]
    fn test_grouping_values_by_quarter() {
        let entry1 = Entry(1, date(2023, 1, 13));
        let entry2 = Entry(2, date(2023, 2, 13));
        let entry3 = Entry(2, date(2023, 2, 13));
        let entry4 = Entry(3, date(2023, 5, 19));

        let grouping: GroupedByQuarter<Entry> =
            GroupedByPeriod::new(&[entry1, entry2, entry3, entry4]);

        assert_eq!(
            grouping.get(&date(2023, 1, 1)).unwrap(),
            &[entry1, entry2, entry3]
        );
        assert_eq!(grouping.get(&date(2023, 4, 1)).unwrap(), &[entry4]);
    }

    #[test]
    fn test_iter() {
        let entry1 = Entry(1, date(2023, 1, 13));
        let entry2 = Entry(2, date(2023, 2, 13));
        let entry3 = Entry(2, date(2023, 2, 13));

        let grouping: GroupedByQuarter<Entry> = GroupedByPeriod::new(&[entry1, entry2, entry3]);

        let mut run = false;

        for (date_val, values) in &grouping {
            assert_eq!(date_val, &date(2023, 1, 1));
            assert_eq!(values, &vec![entry1, entry2, entry3]);
            run = true;
        }

        assert!(run);
    }

    #[test]
    fn test_into_iter() {
        let entry1 = Entry(1, date(2023, 1, 13));
        let entry2 = Entry(2, date(2023, 2, 13));
        let entry3 = Entry(2, date(2023, 2, 13));

        let grouping: GroupedByQuarter<Entry> = GroupedByPeriod::new(&[entry1, entry2, entry3]);

        let mut run = false;

        for (date_val, values) in grouping {
            assert_eq!(date_val, date(2023, 1, 1));
            assert_eq!(values, vec![entry1, entry2, entry3]);
            run = true;
        }

        assert!(run);
    }

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}
