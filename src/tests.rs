use super::*;

#[test]
fn test_insert_with_greater_range() {
  let mut range_set: RangeSet<[std::ops::RangeInclusive<usize>; 1]> = RangeSet::new();
  range_set.insert_range(0..=22);
  range_set.insert_range(24..=26);
  range_set.insert_range(29..=29);
  range_set.insert(23);
  
  let mut iter = range_set.ranges();
  assert_eq!(iter.next(), Some(&(0..=26)));
  assert_eq!(iter.next(), Some(&(29..=29)));
  assert_eq!(iter.next(), None);
}
