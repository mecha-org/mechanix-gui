use std::ops::Neg;

use mctk_core::{
    component::{self, Component, RenderContext},
    event::{Event, MouseDown},
    lay, msg, node,
    reexports::femtovg::Align,
    renderables::{
        image::InstanceBuilder as ImageInstanceBuilder,
        rect::InstanceBuilder as RectInstanceBuilder, svg::InstanceBuilder as SvgInstanceBuilder,
        text::InstanceBuilder as TextInstanceBuilder, Image, Rect, Renderable, Svg, Text,
    },
    size, state_component_impl,
    style::FontWeight,
    widgets::{Div, IconType},
    Color, Node, Pos, Scale, AABB,
};
use mctk_macros::component;
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::ToplevelKey;

use crate::gui::Message;

use super::running_apps_carousel::CarouselMessage;

#[derive(PartialEq, Hash, Default, Debug, Clone)]
pub struct AppInstance {
    pub title: Option<String>,
    pub instance_key: ToplevelKey,
    pub icon: Option<String>,
}

/// Configuration for the App widget
#[derive(Default, Debug, Clone)]
pub struct AppDetails {
    pub app_id: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub icon_type: Option<IconType>,
    pub icon_dominant_color: Option<Color>,
    pub icon_path: Option<String>,
    pub instances: Vec<AppInstance>,
}

#[derive(Debug, Default, Clone)]
struct RunningAppState {
    drag_y: f32,
}

#[component(State = "RunningAppState", Internal)]
#[derive(Default, Debug, Clone)]
pub(crate) struct RunningApp {
    pub app_details: AppDetails,
}

impl RunningApp {
    pub fn new(app_details: AppDetails) -> Self {
        Self {
            app_details,
            state: Some(RunningAppState::default()),
            dirty: false,
        }
    }
}

#[state_component_impl(RunningAppState)]
impl Component for RunningApp {
    fn on_click(&mut self, event: &mut Event<mctk_core::event::Click>) {
        event.emit(msg!(Message::AppInstanceClicked(
            self.app_details.instances.get(0).unwrap().instance_key,
        )));
    }

    fn on_drag_start(&mut self, event: &mut Event<mctk_core::event::DragStart>) {
        // println!("drag start");

        event.stop_bubbling();
    }

    fn on_drag_end(&mut self, event: &mut Event<mctk_core::event::DragEnd>) {
        self.state_mut().drag_y = 0.;
        // event.emit(msg!(Message::AppInstanceCloseClicked(
        //     self.app_details.instances.get(0).unwrap().instance_key,
        // )));
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut Event<mctk_core::event::Drag>) {
        // println!("on drag");
        // println!(
        //     "{:?} {:?} {:?} {:?}",
        //     event.logical_delta(),
        //     event.physical_delta(),
        //     event.bounded_logical_delta(),
        //     event.bounded_physical_delta()
        // );
        let delta_x = event.bounded_logical_delta().x;
        let delta_y = event.bounded_logical_delta().y.neg();
        if delta_y > 0. && delta_x == 0. {
            self.state_mut().drag_y = delta_y;
        } else {
            event.emit(msg!(CarouselMessage::ChildDragX(delta_x)));
        }

        // event.stop_bubbling();
    }

    fn render(&mut self, context: RenderContext) -> Option<Vec<Renderable>> {
        let drag_y = self.state_ref().drag_y;
        if let Some(instance) = self.app_details.instances.get(0) {
            let width = context.aabb.width();
            let height = context.aabb.height();
            let mut pos = context.aabb.pos;
            let mut rs = vec![];
            pos.y -= drag_y;

            //Background
            let background = RectInstanceBuilder::default()
                .pos(pos)
                .scale(Scale { width, height })
                .color(Color::TRANSPARENT)
                .build()
                .unwrap();

            rs.push(Renderable::Rect(Rect::from_instance_data(background)));

            //Image background
            let radius = 10.;
            let img_bg_scale = Scale {
                width: 208.,
                height: 208.,
            };
            let img_bg_pos = Pos {
                x: pos.x,
                y: pos.y,
                z: pos.z + 5.,
            };

            let bg_color = match self.app_details.icon_dominant_color.clone() {
                Some(color) => color,
                None => Color::BLACK,
            };

            let image_background = RectInstanceBuilder::default()
                .pos(img_bg_pos)
                .scale(img_bg_scale)
                .color(bg_color)
                .radius((radius, radius, radius, radius))
                .build()
                .unwrap();

            rs.push(Renderable::Rect(Rect::from_instance_data(image_background)));

            //Image
            let image_scale = Scale {
                width: 69.,
                height: 69.,
            };
            //to get image in center
            let image_pos = img_bg_pos
                + Pos {
                    x: (img_bg_scale.width - image_scale.width) / 2.,
                    y: (img_bg_scale.height - image_scale.width) / 2.,
                    z: 5.,
                };
            if let Some(icon) = self.app_details.name.clone() {
                if let Some(icon_type) = self.app_details.icon_type {
                    match icon_type {
                        IconType::Png => {
                            let image = ImageInstanceBuilder::default()
                                .pos(image_pos)
                                .scale(image_scale)
                                .name(icon)
                                .radius(radius)
                                .dynamic_load_from(self.app_details.icon_path.clone())
                                .build()
                                .unwrap();
                            rs.push(Renderable::Image(Image::from_instance_data(image)));
                        }
                        IconType::Svg => {
                            let image = SvgInstanceBuilder::default()
                                .pos(image_pos)
                                .scale(image_scale)
                                .name(icon)
                                .dynamic_load_from(self.app_details.icon_path.clone())
                                .build()
                                .unwrap();
                            rs.push(Renderable::Svg(Svg::from_instance_data(image)));
                        }
                        _ => {}
                    }
                }
            };

            //Title
            let font_size = 16.;
            let line_height = font_size * 1.3;
            let font_weight = FontWeight::Semibold;
            let title_scale = Scale {
                width: img_bg_scale.width,
                height: 24.,
            };
            let title_margin = (8., 0., 0., 0.);
            let title_pos = Pos {
                x: img_bg_pos.x,
                y: img_bg_pos.y + img_bg_scale.height + title_margin.0,
                z: img_bg_pos.z,
            };
            if let Some(title) = instance.title.clone() {
                let title_instance = TextInstanceBuilder::default()
                    .align(Align::Left)
                    .pos(title_pos)
                    .scale(title_scale)
                    .text(title)
                    .color(Color::WHITE)
                    // .font(font)
                    .weight(font_weight)
                    .line_height(line_height)
                    .font_size(font_size)
                    .build()
                    .unwrap();

                rs.push(Renderable::Text(Text::from_instance_data(title_instance)));
            };

            //Name
            let font_size = 14.;
            let line_height = font_size * 1.3;
            let font_weight = FontWeight::Semibold;
            let name_scale = Scale {
                width: img_bg_scale.width,
                height: 20.,
            };
            let name_margin = (2., 0., 0., 0.);
            let name_pos = Pos {
                x: title_pos.x,
                y: title_pos.y + title_scale.height + name_margin.0,
                z: title_pos.z,
            };
            if let Some(name) = self.app_details.name.clone() {
                let name_instance = TextInstanceBuilder::default()
                    .align(Align::Left)
                    .pos(name_pos)
                    .scale(name_scale)
                    .text(name)
                    .color(Color::rgb(197., 200., 207.))
                    // .font(font)
                    .weight(font_weight)
                    .line_height(line_height)
                    .font_size(font_size)
                    .build()
                    .unwrap();
                rs.push(Renderable::Text(Text::from_instance_data(name_instance)));
            };

            Some(rs)
        } else {
            None
        }
    }
}
