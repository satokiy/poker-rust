
struct PlayerRepositoryImpl {
  db: db
}

impl PlayerRepository for PlayerRepositoryImpl {
  fn insert_player(&self, name: String) -> i32 {
    1
  }
}