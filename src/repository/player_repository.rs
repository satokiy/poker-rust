pub trait PlayerRepository {
  fn insert_player(&self, name: String) -> i32;
}
