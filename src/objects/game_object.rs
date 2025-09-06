use crate::objects::{anchor::Anchor, enemy::Enemy, Player};

pub trait GameObject {
    fn new() -> Self;
}
impl GameObject for Player {
    fn new() -> Self { Player::new() }
}
impl GameObject for Anchor {
    fn new() -> Self { Anchor::new() }
}
impl GameObject for Enemy {
    fn new() -> Self { Enemy::new() }
}