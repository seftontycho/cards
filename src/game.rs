pub mod highlow;
// pub mod whist;

pub trait Game {
    type Action: Into<u32>;
    type Player: Into<u32>;
    type Reward: Into<f32>;
    type State;

    fn current_player(&self) -> &Self::Player;
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn observation(&self) -> Self::State;
    fn step(&mut self, action: Self::Action) -> (Self::State, Self::Reward, bool);
    fn reset(&mut self);
    fn render(&self);
}
