use bevy::prelude::*;

#[derive(Component)]
pub struct DrawerItemsRoot;

#[derive(Component)]
pub struct ButtonType1(pub bool);

#[derive(Component)]
pub struct ButtonType2(pub bool);

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, Interaction)]
pub struct ButtonType3;

#[derive(Component)]
pub struct ButtonType4(pub bool);
