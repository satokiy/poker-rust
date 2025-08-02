
pub trait PlayerService: Send + Sync {
  fn create_player(&self, name: String) -> i32;
}