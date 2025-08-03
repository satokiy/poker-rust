use crate::repository::player_repository::PlayerRepository;


pub struct PlayerServiceImpl<R: PlayerRepository> {
  pub repository: R,
}

impl PlayerService for PlayerServiceImpl {
  fn create_player(&self, name: String) -> i32 {

    let res = self.repository.insert_player(name).await;
    return res;

  }
}