use crate::domain::repository::player_repository::PlayerRepository;


struct PlayerServiceImpl<R: PlayerRepository> {
  pub repository: R,
}

impl PlayerService for PlayerServiceImpl {
  fn create_player(&self, name: String) -> i32 {
    self.repository.insert_player(name)
  }
}