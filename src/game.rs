#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    AssetLoading,
    InGame,
    Paused,
}
