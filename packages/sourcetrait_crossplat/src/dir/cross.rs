#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrossDir {
    HomeCache,
    HomeConfig,
    HomeData,
    HomeState,
}