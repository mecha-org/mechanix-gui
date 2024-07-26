use crate::settings::{self, NotificationSettings};
use crate::theme::{self, NotificationTheme};
use crate::{AppMessage, AppParams, NotificationArgs};
use clap::Parser;
use command::spawn_command;
use mctk_core::component::RootComponent;
use mctk_core::event::Event;
use mctk_core::layout::{Alignment, Dimension};
use mctk_core::reexports::femtovg::{Align, CompositeOperation};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::style::FontWeight;
use mctk_core::{component, layout, Color, Point, Pos, Scale, AABB};
use mctk_core::{
    component::Component,
    lay, msg, node, rect,
    renderables::{
        image::InstanceBuilder as ImageInstanceBuilder,
        rect::InstanceBuilder as RectInstanceBuilder, svg::InstanceBuilder as SvgInstanceBuilder,
        text::InstanceBuilder as TextInstanceBuilder, Image, Rect, Renderable, Svg, Text,
    },
    size, size_pct, state_component_impl,
    widgets::Div,
    Node,
};
use std::any::Any;
use std::hash::Hash;
use std::ops::Neg;

pub enum IconType {
    Png,
    Svg,
}

#[derive(Debug, Clone)]
pub enum SettingNames {
    Wireless,
    Bluetooth,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    AppClicked { app_id: String },
    Show,
    Hide,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Default)]
pub struct NotificationState {
    settings: NotificationSettings,
    custom_theme: NotificationTheme,
    drag_y: f32,
    drag_angle: Option<f32>,
}

#[component(State = "NotificationState")]
#[derive(Debug, Default)]
pub struct Notification {}

impl Notification {
    fn handle_on_drag(
        &mut self,
        logical_delta: Point,
        physical_delta: Point,
        start_pos: Point,
        logical_mouse_position: Point,
    ) -> Option<mctk_core::component::Message> {
        let dx = logical_delta.x;
        let dy = logical_delta.y;
        let min_drag = 10.;
        let mut drag_angle_op = self.state_ref().drag_angle;

        if (dx.abs() > min_drag || dy.abs() > min_drag) && drag_angle_op.is_none() {
            //Drag x or Drag y
            let start_pos = start_pos;
            let current_pos = logical_mouse_position;
            let angle = ((current_pos.y - start_pos.y) / (current_pos.x - start_pos.x))
                .atan()
                .to_degrees();
            drag_angle_op = Some(angle);
            self.state_mut().drag_angle = Some(angle);
        }

        if let Some(drag_angle) = drag_angle_op {
            if drag_angle.abs() > 60. {
                //Drag in y direction
                if dy.neg() > 0. {
                    self.state_mut().drag_y = dy;
                }
            }
        }
        None
    }

    fn handle_drag_end(
        &mut self,
        logical_delta: Point,
        current_physical_aabb: AABB,
    ) -> Option<mctk_core::component::Message> {
        let dy = logical_delta.y.neg();
        if let Some(drag_angle) = self.state_ref().drag_angle {
            if drag_angle.abs() > 60. {
                println!("dy is {:?}", dy);
                if dy > 40. {
                    std::process::exit(0);
                }
            }
        };
        None
    }
}

#[state_component_impl(NotificationState)]
impl Component for Notification {
    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        if self.state.is_some() {
            (self.state_ref().drag_y.neg() as u64).hash(hasher);
        }
    }

    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => NotificationSettings::default(),
        };

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => NotificationTheme::default(),
        };
        self.state = Some(NotificationState {
            settings,
            custom_theme,
            drag_y: 0.,
            drag_angle: None,
        });
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            _ => (),
        }
        vec![]
    }

    fn on_drag_start(&mut self, event: &mut Event<mctk_core::event::DragStart>) {
        event.stop_bubbling();
    }

    fn on_touch_drag_start(&mut self, event: &mut Event<mctk_core::event::TouchDragStart>) {
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut Event<mctk_core::event::Drag>) {
        let messages = self.handle_on_drag(
            event.logical_delta(),
            event.physical_delta(),
            event.input.start_pos,
            event.logical_mouse_position(),
        );
        if let Some(message) = messages {
            event.emit(message);
        }
    }

    fn on_touch_drag(&mut self, event: &mut Event<mctk_core::event::TouchDrag>) {
        let messages = self.handle_on_drag(
            event.logical_delta(),
            event.physical_delta(),
            event.input.start_pos,
            event.logical_touch_position(),
        );
        if let Some(message) = messages {
            event.emit(message);
        }
    }

    fn on_drag_end(&mut self, event: &mut Event<mctk_core::event::DragEnd>) {
        let messages = self.handle_drag_end(event.logical_delta(), event.current_physical_aabb());
        if let Some(message) = messages {
            event.emit(message);
        }
        self.state_mut().drag_y = 0.;
        self.state_mut().drag_angle = None;
    }

    fn on_touch_drag_end(&mut self, event: &mut Event<mctk_core::event::TouchDragEnd>) {
        let messages = self.handle_drag_end(event.logical_delta(), event.current_physical_aabb());
        if let Some(message) = messages {
            event.emit(message);
        }
        self.state_mut().drag_y = 0.;
        self.state_mut().drag_angle = None;
    }

    fn render(&mut self, context: component::RenderContext) -> Option<Vec<Renderable>> {
        let NotificationArgs {
            app_name,
            title,
            description,
            icon,
            ..
        } = NotificationArgs::parse();
        let icons = self.state_ref().settings.icons.clone();

        let width = context.aabb.width();
        let height = context.aabb.height();
        let mut pos = context.aabb.pos;
        pos.y = pos.y + self.state_ref().drag_y;
        let mut rs = vec![];

        //Background
        let background = RectInstanceBuilder::default()
            .pos(pos)
            .scale(Scale { width, height })
            .color(Color::rgb(50., 54., 64.))
            .radius((9., 9., 9., 9.))
            .build()
            .unwrap();

        rs.push(Renderable::Rect(Rect::from_instance_data(background)));

        let padding = (16., 16., 16., 16.);

        //Image background
        let radius = 10.;
        let img_bg_pos = pos
            + Pos {
                x: padding.1,
                y: padding.0,
                z: 0.,
            };
        let img_bg_scale = Scale {
            width: 40.,
            height: 40.,
        };

        let dominant_color: Option<Color> = None;
        let bg_color = match dominant_color {
            Some(color) => color,
            None => Color::rgb(88., 92., 103.),
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
            width: 32.,
            height: 32.,
        };
        //to get image in center
        let image_pos = img_bg_pos
            + Pos {
                x: (img_bg_scale.width - image_scale.width) / 2.,
                y: (img_bg_scale.height - image_scale.width) / 2.,
                z: 5.,
            };

        let icon = Some("bell".to_string());
        let icon_type = Some(IconType::Svg);
        let icon_path = icons.bell.clone();

        if let Some(icon) = icon {
            if let Some(icon_type) = icon_type {
                match icon_type {
                    IconType::Png => {
                        let image = ImageInstanceBuilder::default()
                            .pos(image_pos)
                            .scale(image_scale)
                            .name(icon)
                            .radius(radius)
                            .dynamic_load_from(icon_path)
                            .build()
                            .unwrap();
                        rs.push(Renderable::Image(Image::from_instance_data(image)));
                    }
                    IconType::Svg => {
                        let image = SvgInstanceBuilder::default()
                            .pos(image_pos)
                            .scale(image_scale)
                            .name(icon)
                            .dynamic_load_from(icon_path)
                            .build()
                            .unwrap();
                        rs.push(Renderable::Svg(Svg::from_instance_data(image)));
                    }
                    _ => {}
                }
            }
        };

        //Single line title
        let font_size = 18.;
        let line_height = font_size * 1.3;
        let font_weight = FontWeight::Bold;
        let title_padding = (0., 8., 0., 0.);
        let title_scale = Scale {
            width: width - img_bg_scale.width - padding.0 - padding.3,
            height: 24.,
        };
        let title_pos = Pos {
            x: img_bg_pos.x + img_bg_scale.width + title_padding.1,
            y: img_bg_pos.y,
            z: pos.z,
        };

        if let Some(mut title) = title {
            if title.len() > 24 {
                title = [title[..24].to_owned().to_string(), "...".to_string()].join("");
            }
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

        //Description
        let font_size = 16.;
        let line_height = font_size * 1.3;
        let font_weight = FontWeight::Normal;
        let description_scale = Scale {
            width: title_scale.width,
            height: line_height * 2.5,
        };
        let description_margin = (2., 0., 0., 0.);
        let description_pos = Pos {
            x: title_pos.x,
            y: title_pos.y + title_scale.height + description_margin.0,
            z: title_pos.z,
        };
        if let Some(mut description) = description {
            if description.len() > 48 {
                description =
                    [description[..48].to_owned().to_string(), "...".to_string()].join("");
            }
            let description_instance = TextInstanceBuilder::default()
                .align(Align::Left)
                .pos(description_pos)
                .scale(description_scale)
                .text(description)
                .color(Color::rgb(197., 200., 207.))
                // .font(font)
                .weight(font_weight)
                .line_height(line_height)
                .font_size(font_size)
                .build()
                .unwrap();
            rs.push(Renderable::Text(Text::from_instance_data(
                description_instance,
            )));
        };

        Some(rs)
    }
}

impl RootComponent<AppParams> for Notification {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {}
}
