pub fn search(array: &[&'static str], target: &str) -> Option<(&'static str, usize, u32)> {
    let mut steps: u32 = 0;
    let mut low: usize = 0;
    let mut high: usize = array.len() - 1;
  
    while low <= high {
      let middle: usize = (high - low) / 2 + low;
      steps += 1;
  
      match array[middle] {
        guess if guess < target => low = middle + 1,
        guess if guess > target => high = middle - 1,
        guess if guess == target => return Some((guess, middle, steps)),
        _ => ()
      }
    }
    return None;
  }
  
  #[cfg(test)]
  mod test_binary_search;