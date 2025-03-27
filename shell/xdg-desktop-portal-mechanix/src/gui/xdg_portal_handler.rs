use super::file_chooser_handler::FileChooserHandler;
use crate::interfaces::file_chooser::FileChooserOptions;
use mctk_core::component::{self, Component, RootComponent};
use mctk_macros::{component, state_component_impl};
use std::default;
use std::hash::Hash;
use std::sync::Arc;
#[component(State = "XDGPortalState")]
#[derive(Debug, Default)]
pub struct XDGPortal {}

#[derive(Clone)]
pub struct XDGPortalParams {}

#[derive(Debug, Default)]
pub struct XDGPortalState {
    pub file_chooser_state: FileChooserHandler,
}

impl XDGPortalState {
    pub fn default() -> Self {
        Self {
            file_chooser_state: FileChooserHandler::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FileChooserRequested(FileChooserOptions),
}

#[state_component_impl(XDGPortalState)]
impl Component for XDGPortal {
    fn init(&mut self) {
        dbg!("init is being called");
        self.state = Some(XDGPortalState::default());
    }

    fn update(&mut self, msg: component::Message) -> Vec<component::Message> {
        dbg!("Update is being called");
        if let Some(m) = msg.downcast_ref::<Message>() {
            match m {
                Message::FileChooserRequested(options) => {
                    self.state_mut()
                        .file_chooser_state
                        .handle_request(options);
                }
            }
        }
        vec![]
    }

    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        // Hash the file_chooser_state to account for its current state
        self.state_ref().file_chooser_state.file_chooser_open.hash(hasher);
    }

    fn view(&self) -> Option<mctk_core::Node> {
        dbg!("view is being called");
        None
    }
}

impl RootComponent<XDGPortalParams> for XDGPortal {}
