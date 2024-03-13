// 6 types of strings
fn main() {
  /**
   * 1. (borrowed) string slice --> cannot be modified
   * all string literals are borrowed string slice
   * str
   * &str
   * let msg = "Hello :)";
   * 
   * &str is a pointer to some bytes with len
   * 
   * 2. String --> can be modified
   * let msg = "Hello :)".to_string();
   * OR
   * let msg = String::from("Hello :)");
   * 
   * String is a pointer to some bytes with len and capacity
   * 
   * &str is a subset of String
   * 
   * bytes > scalars > graphemes
   * 
   * word.bytes() --> to iterate over bytes
   * word.chars() --> to iterate over unicode scalars
   * package --> unicode-segmentation (gives methods to handle graphemes)
   * graphemes(my_string, true)
   * 
   * iterators have .nth(3)
   * can be used for indexing
   */
}
