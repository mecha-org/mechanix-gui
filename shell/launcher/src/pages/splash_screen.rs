use desktop_entries::DesktopEntry;
use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    node, rect, size, size_pct,
    widgets::{Div, Image},
    Color,
};
use std::hash::Hash;

#[derive(Debug)]
pub struct SplashScreen {
    pub app: Option<DesktopEntry>,
}

impl Component for SplashScreen {
    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.app.is_some().hash(hasher);
    }

    fn view(&self) -> Option<mctk_core::Node> {
        let mut start = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct:[100],
            ]
        );

        if let Some(app) = self.app.clone() {
            start = start.push(node!(
                Image::new(app.name),
                lay![
                    size: [100],
                    margin: [156., 190., 0.,0.]
                ]
            ));
        }

        Some(start)
    }
}
