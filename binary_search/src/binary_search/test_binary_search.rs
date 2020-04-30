use super::*;

#[test]
fn binary_search_found() {
  let array: [&'static str; 17] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
  ];

  assert_eq!(search(&array, "a"), Some(("a", 0, 4)));
  assert_eq!(search(&array, "g"), Some(("g", 6, 4)));
  assert_eq!(search(&array, "i"), Some(("i", 8, 1)));
  assert_eq!(search(&array, "p"), Some(("p", 15, 4)));
}

#[test]
fn binary_search_not_found() {
  let array: [&'static str; 6] = ["a", "b", "c", "d", "e", "f"];
  assert_eq!(search(&array, "g"), None);
}