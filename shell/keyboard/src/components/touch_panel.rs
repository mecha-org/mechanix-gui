use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::action::Modifier;
use crate::gui::{self, Message};
use crate::layout::{KeyButton, Point};
use crate::settings::ClickAreaConfigs;
use mctk_core::component::{self, Component};
use mctk_core::event::{Event, MouseDown, MouseUp};
use mctk_core::reexports::femtovg::Align;
use mctk_core::renderables::{
    image::InstanceBuilder as ImageInstanceBuilder, rect::InstanceBuilder as RectInstanceBuilder,
    svg::InstanceBuilder as SvgInstanceBuilder, text::InstanceBuilder as TextInstanceBuilder,
    Image, Rect, Renderable, Svg, Text,
};
use mctk_core::style::FontWeight;
use mctk_core::widgets::IconType;
use mctk_core::{
    lay, msg, node, size_pct, state_component_impl, widgets::Div, Color, Pos, Scale, AABB,
};
use mctk_macros::component;
use wayland_protocols_async::zwp_input_method_v2::handler::ContentPurpose;

#[derive(Debug, Default)]
pub struct TouchPanelState {
    pub proximity_matrix: Vec<
        Vec<(
            KeyButton,
            (
                crate::layout::Point,
                crate::layout::Point,
                crate::layout::Point,
                crate::layout::Point,
            ),
        )>,
    >,
    pub next_char_prob: HashMap<String, f64>,
    pub key_pressing: Option<KeyButton>,
}

#[derive(Debug)]
pub enum TouchPanelMsg {
    Clicked { position: crate::layout::Point },
}

#[component(State = "TouchPanelState")]
#[derive(Debug)]
pub struct TouchPanel {
    pub view: crate::layout::View,
    pub aabb: Option<AABB>,
    pub next_char_prob: HashMap<String, f64>,
    pub current_view: String,
    pub click_area_configs: ClickAreaConfigs,
    pub purpose: ContentPurpose,
    pub active_mods: HashSet<Modifier>,
}

impl TouchPanel {
    pub fn new(
        view: crate::layout::View,
        next_char_prob: HashMap<String, f64>,
        current_view: String,
        click_area_configs: ClickAreaConfigs,
        purpose: ContentPurpose,
        active_mods: HashSet<Modifier>,
    ) -> Self {
        // println!("TouchPanel::new()");

        Self {
            view,
            aabb: None,
            state: Some(TouchPanelState {
                proximity_matrix: Vec::new(),
                next_char_prob: HashMap::new(),
                key_pressing: None,
            }),
            click_area_configs,
            current_view,
            purpose,
            dirty: false,
            next_char_prob: next_char_prob.clone(),
            active_mods,
        }
    }

    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<gui::Message> {
        println!("update()");
        let mut return_messages = Vec::new();

        let message = msg.downcast_ref::<TouchPanelMsg>();
        if let Some(message) = message {
            match message {
                TouchPanelMsg::Clicked { position } => {
                    println!("got click at {:?}", position);
                    //based on x and y of position find position in proximity_matrix

                    let proximity_matrix = self.state_ref().proximity_matrix.clone();
                    // println!("proximity_matrix {:?}", proximity_matrix);
                    let mut possible_buttons: Vec<KeyButton> = vec![];
                    'outer: for (i, row) in proximity_matrix.iter().enumerate() {
                        for (j, (button, (tl, bl, br, tr))) in row.iter().enumerate() {
                            // if button.name == "o" {
                            //     println!("checking click {:?} {:?} {:?} {:?}", tl, bl, tr, br);
                            // }
                            // println!("checking click {:?} {:?} {:?} {:?}", tl, bl, tr, br);

                            if position.x >= tl.x
                                && position.x >= bl.x
                                && position.x <= tr.x
                                && position.x <= br.x
                                && position.y >= tl.y
                                && position.y >= tr.y
                                && position.y <= bl.y
                                && position.y <= br.y
                            {
                                // println!(
                                //     "Clicked button at index: {:?} {:?} {:?}",
                                //     (i, j),
                                //     button,
                                //     tr
                                // );

                                possible_buttons.push(button.clone());

                                // break 'outer;
                            }
                        }
                    }

                    // println!("possible buttons {:?}", possible_buttons,);

                    if possible_buttons.len() > 0 {
                        let next_char_prob = self.state_ref().next_char_prob.clone();
                        let mut possible_button = possible_buttons[0].clone();
                        for button in possible_buttons {
                            let p1 = next_char_prob.get(&button.name).unwrap_or(&0.);
                            let p2 = next_char_prob.get(&possible_button.name).unwrap_or(&0.);
                            println!(
                                "possibility {:?} {:?} {:?} {:?}",
                                possible_button.name, p2, button.name, p1
                            );
                            if *p1 > *p2 {
                                possible_button = button.clone();
                            }
                        }

                        self.state_mut().key_pressing = Some(possible_button.clone());
                        return_messages.push(gui::Message::KeyClicked(
                            possible_button.action.clone(),
                            possible_button.keycodes.clone(),
                        ));
                    }

                    // println!("after click {:?}", self.state_ref().next_char_prob.clone());
                }
            }
        }

        // println!("return_messages {:?}", return_messages.len());
        return_messages
    }

    fn get_positonal_matric(&self) -> Vec<Vec<(KeyButton, (Point, Point, Point, Point))>> {
        let mut positional_matrix = Vec::new();
        for (r_sp, row) in &self.view.rows {
            let mut row_matrix = Vec::new();
            let r_size = row.get_size();
            for (b_sp, button) in &row.buttons {
                let tl = crate::layout::Point {
                    x: r_sp.x + b_sp,
                    y: r_sp.y,
                };

                let bl = crate::layout::Point {
                    x: tl.x,
                    y: tl.y + button.size.1,
                };

                let tr = crate::layout::Point {
                    x: tl.x + button.size.0,
                    y: tl.y,
                };

                let br = crate::layout::Point {
                    x: bl.x + button.size.0,
                    y: bl.y,
                };
                row_matrix.push((button.clone(), (tl, bl, br, tr)));
            }
            positional_matrix.push(row_matrix);
        }
        positional_matrix
    }

    fn handle_press(&mut self, position: mctk_core::Point) -> Vec<Message> {
        let msgs = self.update(msg!(TouchPanelMsg::Clicked {
            position: Point {
                x: position.x as f64,
                y: position.y as f64
            }
        }));
        msgs.clone()
    }

    fn handle_release(&mut self) {
        self.state_mut().key_pressing = None;
    }
}

#[state_component_impl(TouchPanelState)]
impl Component for TouchPanel {
    fn init(&mut self) {
        self.state_mut().proximity_matrix = self.get_positonal_matric();
    }

    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().proximity_matrix.len().hash(hasher);
        self.state_ref().next_char_prob.len().hash(hasher);
        self.state_ref().key_pressing.hash(hasher);
    }

    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.next_char_prob.len().hash(hasher);
        self.current_view.hash(hasher);
        self.purpose.hash(hasher);
        self.active_mods.len().hash(hasher);
        // self.view.hash(hasher);
    }

    fn new_props(&mut self) {
        // println!("TouchPanel:: new_props()");
        self.state_mut().next_char_prob = self.next_char_prob.clone();
        //Find posibilities of next keys
        //Update proximity martrix based on that
        let v_size = &self.view.size;
        let positional_matrix = self.get_positonal_matric();
        // println!("positional_matrix {:?}", positional_matrix);
        let updated_proximity_matrix = positional_matrix
            .iter()
            .enumerate()
            .map(|(i, mut row)| {
                row.clone()
                    .into_iter()
                    .enumerate()
                    .map(|(j, (button, (mut tl, mut bl, mut br, mut tr)))| {
                        // check proximity and update new_button
                        let click_area_increase_by = self.click_area_configs.increase_by as f64;
                        //Get adjacent buttons
                        let adjacent_buttons =
                            get_adjacent_buttons(positional_matrix.clone(), i, j);

                        let top_btn = &adjacent_buttons.0;
                        let left_btn = &adjacent_buttons.1;
                        let bottom_btn = &adjacent_buttons.2;
                        let right_btn = &adjacent_buttons.3;
                        let mut possibilities: HashMap<String, f64> =
                            self.state_ref().next_char_prob.clone();
                        // if button.name == "i" {
                        //     println!("possibilities {:?}", possibilities);
                        // }

                        // possibilities.insert("i".to_string(), 50.);
                        let cb_p = possibilities.get(&button.name).unwrap_or(&0.).clone();

                        let tb_p = if let Some(top_btn) = top_btn {
                            possibilities.get(&top_btn.name).unwrap_or(&0.).clone()
                        } else {
                            0.
                        };

                        let lb_p = if let Some(left_btn) = left_btn {
                            possibilities.get(&left_btn.name).unwrap_or(&0.).clone()
                        } else {
                            0.
                        };

                        let bb_p = if let Some(bottom_btn) = bottom_btn {
                            possibilities.get(&bottom_btn.name).unwrap_or(&0.).clone()
                        } else {
                            0.
                        };

                        let rb_p = if let Some(right_btn) = right_btn {
                            possibilities.get(&right_btn.name).unwrap_or(&0.).clone()
                        } else {
                            0.
                        };

                        // if button.name == "o" {
                        //     println!("old {:?} {:?} {:?} {:?}", tl, bl, tr, br);
                        // }

                        if cb_p > 0. && cb_p > tb_p && tb_p == 0. {
                            tl.y = tl.y - click_area_increase_by;
                            tr.y = tr.y - click_area_increase_by;
                            // println!("increased bottom area of {:?}", button.name);
                        }

                        if cb_p > 0. && cb_p > lb_p && lb_p == 0. {
                            tl.x = tl.x - click_area_increase_by;
                            bl.x = bl.x - click_area_increase_by;
                            // println!("increased left area of {:?}", button.name);
                        }

                        if cb_p > 0. && cb_p > bb_p && bb_p == 0. {
                            bl.y = bl.y + click_area_increase_by;
                            br.y = br.y + click_area_increase_by;
                            // println!("increased bottom area of {:?}", button.name);
                        }

                        if cb_p > 0. && cb_p > rb_p && rb_p == 0. {
                            tr.x = tr.x + click_area_increase_by;
                            br.x = br.x + click_area_increase_by;
                            // println!("increased right area of {:?}", button.name);
                        }

                        //Update size base d

                        // if button.name == "o" {
                        //     println!("new {:?} {:?} {:?} {:?}", tl, bl, tr, br);
                        // }

                        (button.clone(), (tl, bl.clone(), br.clone(), tr))
                    })
                    .collect::<Vec<(
                        KeyButton,
                        (
                            crate::layout::Point,
                            crate::layout::Point,
                            crate::layout::Point,
                            crate::layout::Point,
                        ),
                    )>>()
            })
            .collect();

        self.state_mut().proximity_matrix = updated_proximity_matrix;
    }

    fn set_aabb(
        &mut self,
        aabb: &mut AABB,
        _parent_aabb: AABB,
        _children: Vec<(&mut AABB, Option<Scale>, Option<mctk_core::Point>)>,
        _frame: AABB,
        _scale_factor: f32,
    ) {
        self.aabb = Some(aabb.clone());
    }

    fn on_mouse_down(&mut self, event: &mut Event<MouseDown>) {
        // println!("TouchPanel::on_mouse_down()");
        event.stop_bubbling();
        let msgs = self.handle_press(event.relative_logical_position());
        for mesg in msgs {
            event.emit(Box::new(mesg).clone());
        }
    }

    fn on_mouse_up(&mut self, _event: &mut Event<MouseUp>) {
        // println!("TouchPanel::on_mouse_up()");
        self.handle_release();
    }

    fn on_touch_down(&mut self, event: &mut Event<mctk_core::event::TouchDown>) {
        event.stop_bubbling();
        let msgs = self.handle_press(event.relative_logical_position_touch());
        for mesg in msgs {
            event.emit(Box::new(mesg).clone());
        }
    }

    fn on_touch_up(&mut self, _event: &mut Event<mctk_core::event::TouchUp>) {
        self.handle_release();
    }

    fn view(&self) -> Option<mctk_core::Node> {
        Some(node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![ size_pct: [100],]
        ))
    }

    fn render(
        &mut self,
        context: mctk_core::component::RenderContext,
    ) -> Option<Vec<mctk_core::renderables::Renderable>> {
        println!("TouchPanel::render()");
        let proximity_matrix = self.state_ref().proximity_matrix.clone();
        let show_click_area = self.click_area_configs.visible;
        let active_mods = self.active_mods.clone();

        let width = context.aabb.width();
        let height = context.aabb.height();

        let mut pos: Pos = context.aabb.pos;
        let mut rs = vec![];

        for (i, (row_dp, row)) in self.view.rows.clone().into_iter().enumerate() {
            let row_pos = Pos {
                x: pos.x + row_dp.x as f32,
                y: pos.y + row_dp.y as f32,
                z: 20.,
            };

            for (j, (col_dp, col)) in row.buttons.clone().into_iter().enumerate() {
                let mut col_pos = row_pos;
                col_pos.x += col_dp as f32;

                let button_width = col.size.0 as f32;
                let button_height = col.size.1 as f32;

                let mut button_color = Color::TRANSPARENT;
                if let Some(key_pressing) = self.state_ref().key_pressing.clone() {
                    if key_pressing.name == col.name {
                        button_color = Color::rgba(255., 255., 255., 0.25);
                    }
                };

                match col.action {
                    crate::action::Action::ApplyModifier(modifier) => {
                        if active_mods.contains(&modifier) {
                            button_color = Color::rgba(255., 255., 255., 0.25);
                        }
                    }
                    _ => (),
                }

                let button = RectInstanceBuilder::default()
                    .pos(col_pos)
                    .scale(Scale {
                        width: button_width,
                        height: button_height,
                    })
                    .color(button_color)
                    .border_size((1., 1., 1., 1.))
                    .border_color(Color::DARK_GREY)
                    .build()
                    .unwrap();

                let mut font_size = 22.;
                let mut line_height = font_size;
                if col.name == "show_numbers"
                    || col.name == "space"
                    || col.name == "Return"
                    || col.name == "show_letters"
                {
                    font_size = 16.;
                }

                let font_weight = FontWeight::Normal;
                let name_scale = Scale {
                    width: button_width,
                    height: button_height,
                };
                let name_margin = (0., 0., 0., 0.);
                let name_pos = Pos {
                    x: col_pos.x,
                    y: col_pos.y + (line_height / 2.),
                    z: 100.,
                };
                let icon_scale = Scale {
                    width: 24.,
                    height: 24.,
                };

                let (_, (p_tl, p_bl, p_br, p_tr)) = &proximity_matrix[i][j];

                let p_button = RectInstanceBuilder::default()
                    .pos(Pos {
                        x: pos.x + p_tl.x as f32,
                        y: pos.y + p_tl.y as f32,
                        z: 220.,
                    })
                    .scale(Scale {
                        width: (p_tr.x - p_tl.x) as f32,
                        height: (p_bl.y - p_tl.y) as f32,
                    })
                    .color(Color::TRANSPARENT)
                    .border_size((1., 1., 1., 1.))
                    .border_color(Color::WHITE)
                    .build()
                    .unwrap();

                if show_click_area {
                    rs.push(Renderable::Rect(Rect::from_instance_data(p_button)));
                }

                rs.push(Renderable::Rect(Rect::from_instance_data(button)));

                match col.label.clone() {
                    crate::layout::Label::Text(label) => {
                        let name_instance = TextInstanceBuilder::default()
                            .align(Align::Center)
                            .pos(name_pos)
                            .scale(name_scale)
                            .text(label.clone())
                            .color(Color::WHITE)
                            // .font(font)
                            .weight(font_weight)
                            .line_height(line_height)
                            .font_size(font_size)
                            .build()
                            .unwrap();

                        rs.push(Renderable::Text(Text::from_instance_data(name_instance)));
                    }
                    crate::layout::Label::Icon(icon) => {
                        let icon_type = Some(IconType::Svg);
                        if let Some(icon_type) = icon_type {
                            //Image
                            let image_scale = Scale {
                                width: 24.,
                                height: 24.,
                            };
                            //to get image in center
                            let image_pos = col_pos
                                + Pos {
                                    x: (button_width - image_scale.width) / 2.,
                                    y: (button_height - image_scale.width) / 2.,
                                    z: 5.,
                                };
                            match icon_type {
                                IconType::Png => {
                                    let image = ImageInstanceBuilder::default()
                                        .pos(image_pos)
                                        .scale(image_scale)
                                        .name(icon)
                                        .dynamic_load_from(None)
                                        .build()
                                        .unwrap();
                                    rs.push(Renderable::Image(Image::from_instance_data(image)));
                                }
                                IconType::Svg => {
                                    let image = SvgInstanceBuilder::default()
                                        .pos(image_pos)
                                        .scale(image_scale)
                                        .name(icon)
                                        .dynamic_load_from(None)
                                        .build()
                                        .unwrap();
                                    rs.push(Renderable::Svg(Svg::from_instance_data(image)));
                                }
                                _ => {}
                            }
                        };
                    }
                }
            }
        }

        Some(rs)
    }
}

fn is_position_valid(i: i8, j: i8, n: usize, m: usize) -> bool {
    if i < 0 || j < 0 {
        return false;
    }
    let i = i as usize;
    let j = j as usize;
    if i >= n || j >= m {
        return false;
    }
    true
}

fn get_adjacent_buttons(
    matrix: Vec<
        Vec<(
            KeyButton,
            (
                crate::layout::Point,
                crate::layout::Point,
                crate::layout::Point,
                crate::layout::Point,
            ),
        )>,
    >,
    i: usize,
    j: usize,
) -> (
    Option<KeyButton>,
    Option<KeyButton>,
    Option<KeyButton>,
    Option<KeyButton>,
) {
    let n = matrix.len();
    let m = matrix.get(0).map_or(0, |row| row.len());

    let mut buttons = (None, None, None, None);

    //top
    if is_position_valid(i as i8 - 1, j as i8, n, m) {
        if let Some(row) = matrix.get(i - 1) {
            if let Some((key_button, _)) = row.get(j) {
                buttons.0 = Some(key_button.clone());
            }
        }
    }

    //left
    if is_position_valid(i as i8, j as i8 - 1, n, m) {
        if let Some(row) = matrix.get(i) {
            if let Some((key_button, _)) = row.get(j - 1) {
                buttons.1 = Some(key_button.clone());
            }
        }
    }

    //bottom
    if is_position_valid(i as i8 + 1, j as i8, n, m) {
        if let Some(row) = matrix.get(i + 1) {
            if let Some((key_button, _)) = row.get(j) {
                buttons.2 = Some(key_button.clone());
            }
        }
    }

    //right
    if is_position_valid(i as i8, j as i8 + 1, n, m) {
        if let Some(row) = matrix.get(i) {
            if let Some((key_button, _)) = row.get(j + 1) {
                buttons.3 = Some(key_button.clone());
            }
        }
    }

    buttons
}
