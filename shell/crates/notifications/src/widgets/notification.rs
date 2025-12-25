use gpui::{
    div, img, prelude::FluentBuilder, px, rgb, Animation, AnimationExt,
    AnyElement, App, AppContext, ClickEvent, Context,
    DismissEvent, Div, ElementId, EventEmitter, FontWeight,
    InteractiveElement as _, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement as _, Point, Render, SharedString, Stateful,
    StatefulInteractiveElement, StyleRefinement, Styled, Window,
};
use smol::Timer;
use std::{
    any::TypeId,
    collections::HashMap,
    rc::Rc,
    time::Duration,
};

use crate::helper::cubic_bezier;
use regex::Regex;

use crate::ui::icon::{Icon, IconName};

pub struct UserDismissedEvent {
    pub id: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum NotificationType {
    #[default]
    Info,
    Application,
    // Success,
    // Warning,
    // Error,
}

impl NotificationType {
    fn icon(&self, cx: &App) -> Icon {
        match self {
            Self::Info => Icon::new(IconName::Info).text_color(rgb(0xf4f4f4)),
            Self::Application => Icon::new(IconName::Application).text_color(rgb(0xf4f4f4)),
        }
    }
}

// Struct to represent the database notification
#[derive(Clone, Debug)]
pub struct DbNotification {
    pub id: u32,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, String>,
    pub received_at: Option<u64>,
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) enum NotificationId {
    Id(TypeId),
    IdAndElementId(TypeId, ElementId),
}

impl From<TypeId> for NotificationId {
    fn from(type_id: TypeId) -> Self {
        Self::Id(type_id)
    }
}

impl From<(TypeId, ElementId)> for NotificationId {
    fn from((type_id, id): (TypeId, ElementId)) -> Self {
        Self::IdAndElementId(type_id, id)
    }
}

/// A notification element.
pub struct NotificationUi {
    /// The id is used make the notification unique.
    /// Then you push a notification with the same id, the previous notification will be replaced.
    ///
    /// None means the notification will be added to the end of the list.
    pub(crate) id: NotificationId,
    pub(crate) db_id: u32,
    pub(crate) db_notification: Option<DbNotification>,
    style: StyleRefinement,
    type_: Option<NotificationType>,
    title: Option<SharedString>,
    message: Option<SharedString>,
    // Store a path to the raster image; build gpui::img in render.
    icon_img: Option<std::path::PathBuf>,
    icon: Option<Icon>,
    pub(crate) autohide: bool,
    pub(crate) expire_timeout: Duration,
    action_builder: Option<Rc<dyn Fn(&mut Self, &mut Window, &mut Context<Self>) -> Stateful<Div>>>,
    content_builder: Option<Rc<dyn Fn(&mut Self, &mut Window, &mut Context<Self>) -> AnyElement>>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    closing: bool,
    // swipe-to-dismiss state
    dragging: bool,
    drag_start: Point<gpui::Pixels>,
    drag_dx: f32,
    drag_moved: bool,
    snapping_back: bool,
    snap_from: f32,
    anim_epoch: u64,
    close_dir: f32,
}

impl From<String> for NotificationUi {
    fn from(s: String) -> Self {
        Self::new().message(s)
    }
}

impl From<SharedString> for NotificationUi {
    fn from(s: SharedString) -> Self {
        Self::new().message(s)
    }
}

impl From<&'static str> for NotificationUi {
    fn from(s: &'static str) -> Self {
        Self::new().message(s)
    }
}

impl From<(NotificationType, &'static str)> for NotificationUi {
    fn from((type_, content): (NotificationType, &'static str)) -> Self {
        Self::new().message(content).with_type(type_)
    }
}

impl From<(NotificationType, SharedString)> for NotificationUi {
    fn from((type_, content): (NotificationType, SharedString)) -> Self {
        Self::new().message(content).with_type(type_)
    }
}

pub struct DefaultIdType;

impl NotificationUi {
    /// Create a new notification.
    ///
    /// The default id is a random UUID.
    pub fn new() -> Self {
        let id: SharedString = uuid::Uuid::new_v4().to_string().into();
        let id = (TypeId::of::<DefaultIdType>(), id.into());

        Self {
            id: id.into(),
            db_id: 0,
            db_notification: None,
            style: StyleRefinement::default(),
            title: None,
            message: None,
            type_: None,
            icon: None,
            icon_img: None,
            autohide: false,
            expire_timeout: Duration::from_millis(5000),
            action_builder: None,
            content_builder: None,
            on_click: None,
            closing: false,
            dragging: false,
            drag_start: Point {
                x: px(0.0),
                y: px(0.0),
            },
            drag_dx: 0.0,
            drag_moved: false,
            snapping_back: false,
            snap_from: 0.0,
            anim_epoch: 0,
            close_dir: 1.0,
        }
    }

    /// Set the message of the notification, default is None.
    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn db_id(mut self, db_id: u32) -> Self {
        self.db_id = db_id;
        self
    }

    pub fn db_notification(mut self, db_notification: DbNotification) -> Self {
        self.db_notification = Some(db_notification);
        self
    }

    /// Create an info notification with the given message.
    pub fn info(message: impl Into<SharedString>) -> Self {
        Self::new()
            .message(message)
            .with_type(NotificationType::Info)
    }

    /// Set the type for unique identification of the notification.
    ///
    /// ```rs
    /// struct MyNotificationKind;
    /// let notification = Notification::new("Hello").id::<MyNotificationKind>();
    /// ```
    pub fn id<T: Sized + 'static>(mut self) -> Self {
        self.id = TypeId::of::<T>().into();
        self
    }

    /// Set the type and id of the notification, used to uniquely identify the notification.
    pub fn id1<T: Sized + 'static>(mut self, key: impl Into<ElementId>) -> Self {
        self.id = (TypeId::of::<T>(), key.into()).into();
        self
    }

    /// Set the title of the notification, default is None.
    ///
    /// If title is None, the notification will not have a title.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the icon of the notification.
    ///
    /// If icon is None, the notification will use the default icon of the type.
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the raster image path as icon of the notification.
    /// Pass an absolute or asset path to an image file (png/jpg/webp/bmp/gif/svg*).
    /// The widget will construct a gpui::img from this path.
    pub fn icon_img(mut self, icon_img_path: impl Into<std::path::PathBuf>) -> Self {
        self.icon_img = Some(icon_img_path.into());
        self
    }

    /// Set the type of the notification, default is NotificationType::Info.
    pub fn with_type(mut self, type_: NotificationType) -> Self {
        self.type_ = Some(type_);
        self
    }

    /// Set the auto hide of the notification, default is true.
    pub fn autohide(mut self, autohide: bool) -> Self {
        self.autohide = autohide;
        self
    }

    /// Set the auto-hide duration.
    pub fn expire_timeout(mut self, duration_in_milliseconds: Duration) -> Self {
        self.expire_timeout = duration_in_milliseconds;
        self
    }

    /// Set the click callback of the notification.
    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    /// Set the action button of the notification.
    pub fn action<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut Self, &mut Window, &mut Context<Self>) -> Stateful<Div> + 'static,
    {
        self.action_builder = Some(Rc::new(action));
        self
    }

    /// `Dismiss` the notification.
    pub fn dismiss(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if self.closing {
            return;
        }
        self.closing = true;
        cx.notify();

        // Dismiss the notification after 0.25s to show the animation.
        cx.spawn(async move |view, cx| {
            Timer::after(Duration::from_secs_f32(0.25)).await;
            cx.update(|cx| {
                if let Some(view) = view.upgrade() {
                    view.update(cx, |view, cx| {
                        view.closing = false;
                        cx.emit(DismissEvent);
                    });
                }
            })
        })
        .detach()
    }

    /// Set the content of the notification.
    pub fn content(
        mut self,
        content: impl Fn(&mut Self, &mut Window, &mut Context<Self>) -> AnyElement + 'static,
    ) -> Self {
        self.content_builder = Some(Rc::new(content));
        self
    }
}

impl EventEmitter<DismissEvent> for NotificationUi {}
impl EventEmitter<UserDismissedEvent> for NotificationUi {}
impl FluentBuilder for NotificationUi {}
impl Styled for NotificationUi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

#[derive(Debug, Clone)]
enum MarkupNode {
    Text(String),
    Link { text: String, url: String },
    LineBreak,
}

/// Parse HTML-style href links and standalone URLs into structured nodes
fn parse_markup(input: &str) -> Vec<MarkupNode> {
    let mut nodes = Vec::new();

    // Split by line breaks
    let lines: Vec<&str> = input.split('\n').collect();

    for (line_idx, line) in lines.iter().enumerate() {
        if line_idx > 0 {
            nodes.push(MarkupNode::LineBreak);
        }

        let mut remaining = line.to_string();

        // Parse HTML-style links: <a href="url">text</a>
        let link_regex = Regex::new(r#"<a\s+href=["']([^"']+)["']>([^<]+)</a>"#).unwrap();

        while !remaining.is_empty() {
            if let Some(captures) = link_regex.captures(&remaining) {
                let full_match = captures.get(0).unwrap();
                let url = captures.get(1).unwrap().as_str().to_string();
                let text = captures.get(2).unwrap().as_str().to_string();

                // Add text before the link
                let before = &remaining[..full_match.start()];
                if !before.is_empty() {
                    nodes.extend(parse_urls_in_text(before));
                }

                // Add the link
                nodes.push(MarkupNode::Link { text, url });

                // Continue with remaining text
                remaining = remaining[full_match.end()..].to_string();
            } else {
                // No more HTML links, check for standalone URLs
                nodes.extend(parse_urls_in_text(&remaining));
                break;
            }
        }
    }

    nodes
}

/// Parse standalone URLs in plain text
fn parse_urls_in_text(text: &str) -> Vec<MarkupNode> {
    let mut nodes = Vec::new();

    // Regex to match URLs (simplified version)
    let url_regex = Regex::new(r"(https?://[^\s]+)").unwrap();

    let mut last_end = 0;

    for captures in url_regex.captures_iter(text) {
        let full_match = captures.get(0).unwrap();
        let url = full_match.as_str().to_string();

        // Add text before the URL
        let before = &text[last_end..full_match.start()];
        if !before.is_empty() {
            nodes.push(MarkupNode::Text(before.to_string()));
        }

        // Add the URL as a link
        nodes.push(MarkupNode::Link {
            text: url.clone(),
            url,
        });

        last_end = full_match.end();
    }

    // Add remaining text
    if last_end < text.len() {
        let remaining = &text[last_end..];
        if !remaining.is_empty() {
            nodes.push(MarkupNode::Text(remaining.to_string()));
        }
    }

    // If no URLs found, return the whole text
    if nodes.is_empty() && !text.is_empty() {
        nodes.push(MarkupNode::Text(text.to_string()));
    }

    nodes
}

/// Render parsed markup nodes as GPUI elements
pub fn render_markup(nodes: &[MarkupNode], cx: &App) -> Div {
    let mut container = div().flex().flex_col().gap_1().w_full().overflow_hidden();

    let mut current_line = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap_1()
        .w_full()
        .overflow_hidden();

    let mut has_content = false;

    for node in nodes {
        match node {
            MarkupNode::Text(text) => {
                current_line = current_line.child(
                    div()
                        .text_sm()
                        .text_color(rgb(0xc0c0c0))
                        .overflow_hidden()
                        .child(text.clone()),
                );
                has_content = true;
            }
            MarkupNode::Link { text, url } => {
                let _url_clone = url.clone();
                current_line = current_line.child(
                    div()
                        .id("link")
                        .text_sm()
                        .text_color(rgb(0x5ab0ff))
                        .underline()
                        .cursor_pointer()
                        .overflow_hidden()
                        .child(text.clone())
                        .on_click(move |_, _, cx| {
                            // Open URL in default browser
                            // open::that(&url_clone).ok();
                        }),
                );
                has_content = true;
            }
            MarkupNode::LineBreak => {
                if has_content {
                    container = container.child(current_line);
                    current_line = div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap_1()
                        .w_full()
                        .overflow_hidden();
                    has_content = false;
                }
            }
        }
    }

    // Add the last line if it has content
    if has_content {
        container = container.child(current_line);
    }

    container
}
impl Render for NotificationUi {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = self
            .content_builder
            .clone()
            .map(|builder| builder(self, window, cx));
        let action = self
            .action_builder
            .clone()
            .map(|builder| builder(self, window, cx));

        let closing = self.closing;
        let dragging = self.dragging;
        let drag_dx = self.drag_dx;
        let snapping_back = self.snapping_back;
        let snap_from = self.snap_from;
        let anim_epoch = self.anim_epoch;
        let close_dir = self.close_dir;
        // let icon = match self.type_ {
        //     None => self.icon_img.clone(),
        //     Some(type_) => Some(type_.icon(cx)),
        // };
        let has_icon = self.icon_img.is_some();
        let icon_path = self.icon_img.clone();
        div()
            .id("notification")
            .group("")
            .relative()
            .w_112()
            .border_1()
            .border_color(rgb(0xff9500))
            .bg(rgb(0x1a1a1a))
            .rounded(px(12.0))
            .shadow_md()
            .py_3p5()
            .px_4()
            .gap_3()
            .flex()
            .flex_row()
            .items_center()
            .when_some(icon_path, |this, path| {
                this.child(
                    div()
                        .w(px(28.0))
                        .h(px(28.0))
                        // .shrink_0()
                        .mr_3()
                        .items_center()
                        .justify_center()
                        .child(img(path).size_7()),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w(px(0.0))
                    .overflow_hidden()
                    .text_color(rgb(0xe9e9e9))
                    .when_some(self.title.clone(), |this, title| {
                        this.child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                // Brighter title for emphasis
                                .text_color(rgb(0xf4f4f4))
                                .whitespace_normal()
                                .child(title),
                        )
                    })
                    .when_some(self.message.clone(), |this, message| {
                        this.child(
                            div()
                                .text_sm()
                                // Slightly muted body text for hierarchy
                                .text_color(rgb(0xd0d0d0))
                                .whitespace_normal()
                                .child(message),
                        )
                    })
                    // .when_some(self.message.clone(), |this, message| {
                    //     let parsed = parse_markup(message.as_ref());
                    //     this.child(render_markup(&parsed, cx))
                    // })
                    .when_some(content, |this, content| this.child(content))
                    .when_some(action, |this, action| this.child(action)),
            )
            .when_some(self.on_click.clone(), |this, on_click| {
                this.on_click(cx.listener(move |view, event, window, cx| {
                    // Prevent accidental clicks when user was dragging
                    if view.drag_moved {
                        // reset the flag after suppressing a click
                        view.drag_moved = false;
                        return;
                    }
                    view.dismiss(window, cx);
                    on_click(event, window, cx);
                }))
            })
            // Swipe-to-dismiss handlers
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, e: &MouseDownEvent, _window, cx| {
                    this.dragging = true;
                    this.drag_moved = false;
                    this.snapping_back = false;
                    this.drag_start = e.position;
                    this.drag_dx = 0.0;
                    cx.notify();
                }),
            )
            .on_mouse_move(cx.listener(|this, e: &MouseMoveEvent, _window, cx| {
                if this.dragging {
                    let dx = e.position.x - this.drag_start.x; // Pixels
                    // start suppressing click after a small slop
                    if dx.abs() > px(3.0) {
                        this.drag_moved = true;
                    }
                    // store as f32 for animation math
                    this.drag_dx = dx.into();
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _e: &MouseUpEvent, window, cx| {
                    if !this.dragging {
                        return;
                    }
                    this.dragging = false;

                    let threshold: f32 = 80.0;
                    let dx = this.drag_dx;
                    this.drag_dx = 0.0;

                    if dx.abs() >= threshold {
                        // swipe dismiss in the dragged direction
                        this.close_dir = if dx < 0.0 { -1.0 } else { 1.0 };
                        this.dismiss(window, cx);
                        cx.emit(UserDismissedEvent { id: this.db_id });
                    } else {
                        // snap back with a short animation
                        this.snapping_back = true;
                        this.snap_from = dx;
                        this.anim_epoch = this.anim_epoch.wrapping_add(1);
                        let epoch = this.anim_epoch;
                        cx.notify();

                        cx.spawn(async move |view, cx| {
                            Timer::after(Duration::from_millis(200)).await;
                            cx.update(|cx| {
                                if let Some(view) = view.upgrade() {
                                    view.update(cx, |this, _| {
                                        // Only clear if no new animation started
                                        if this.anim_epoch == epoch {
                                            this.snapping_back = false;
                                            this.snap_from = 0.0;
                                        }
                                    });
                                }
                            })
                        })
                        .detach();
                    }
                }),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .absolute()
                    .top_3p5()
                    .right_3p5()
                    .invisible()
                    .group_hover("", |this| this.visible())
                    .child(
                        div()
                            .id("close")
                            .child(
                                Icon::new(IconName::Close)
                                    .size((px(20.), px(20.)))
                                    .text_color(rgb(0xf4f4f4)),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dismiss(window, cx);
                                cx.emit(UserDismissedEvent { id: this.db_id });
                            })),
                    ),
            )
            .with_animation(
                ElementId::NamedInteger("notif-anim".into(), (closing as u64) + anim_epoch),
                Animation::new(Duration::from_secs_f64(0.25))
                    .with_easing(cubic_bezier(0.4, 0., 0.2, 1.)),
                move |this, delta| {
                    if closing {
                        // Slide out in the swipe direction with fade
                        let x_offset = delta * px(120.) * close_dir;
                        let opacity = 1. - delta;
                        this.left(x_offset)
                            .shadow_none()
                            .opacity(opacity)
                            .when(opacity < 0.85, |this| this.shadow_none())
                    } else if dragging {
                        // Follow finger: translate horizontally; reduce opacity slightly by distance
                        let dist = drag_dx.abs().min(180.0);
                        let fade = (dist / 180.0) * 0.6; // up to 40% fade
                        let opacity = 1.0 - fade;
                        this.left(px(drag_dx))
                            .opacity(opacity)
                            .when(opacity < 0.85, |this| this.shadow_none())
                    } else if snapping_back && snap_from != 0.0 {
                        // Animate back to origin from last drag offset
                        let start_dist = snap_from.abs().min(180.0);
                        let start_fade = (start_dist / 180.0) * 0.6;
                        let start_opacity = 1.0 - start_fade;
                        let x = px(snap_from * (1.0 - delta));
                        let opacity = start_opacity + (1.0 - start_opacity) * delta;
                        this.left(x)
                            .opacity(opacity)
                            .when(opacity < 0.85, |this| this.shadow_none())
                    } else {
                        // Entrance animation (slide down + fade in)
                        let y_offset = px(-45.) + delta * px(45.);
                        let opacity = delta;
                        this.top(px(0.) + y_offset)
                            .opacity(opacity)
                            .when(opacity < 0.85, |this| this.shadow_none())
                    }
                },
            )
    }
}
