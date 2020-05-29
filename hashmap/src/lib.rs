/// Simple Hashmap which is an array of linked lists that stores 
/// simple key, value pairs which are strings
struct HashMap {
  list: Vec<Vec<()>>
}


impl HashMap {
  /// Creates an empty hashmap
  fn new() -> Self {
    HashMap{
      list: Vec::new(),
    }
  }

}
