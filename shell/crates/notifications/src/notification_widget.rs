use gpui::{
    div, img, prelude::FluentBuilder, px, rgb, Animation, AnimationExt, AnyElement, App,
    AppContext, ClickEvent, Context, DismissEvent, Div, Element, ElementId,
    Entity, EventEmitter, FontWeight, Img, InteractiveElement as _, IntoElement,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _, Point, Render,
    SharedString, Stateful, StatefulInteractiveElement, StyleRefinement, Styled, Subscription, Window,
};
use smol::Timer;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    any::TypeId,
    collections::{HashMap, VecDeque},
    rc::Rc,
    time::Duration,
};

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
pub struct Notification {
    /// The id is used make the notification unique.
    /// Then you push a notification with the same id, the previous notification will be replaced.
    ///
    /// None means the notification will be added to the end of the list.
    id: NotificationId,
    db_id: u32,
    style: StyleRefinement,
    type_: Option<NotificationType>,
    title: Option<SharedString>,
    message: Option<SharedString>,
    // Store a path to the raster image; build gpui::img in render.
    icon_img: Option<std::path::PathBuf>,
    icon: Option<Icon>,
    autohide: bool,
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

impl From<String> for Notification {
    fn from(s: String) -> Self {
        Self::new().message(s)
    }
}

impl From<SharedString> for Notification {
    fn from(s: SharedString) -> Self {
        Self::new().message(s)
    }
}

impl From<&'static str> for Notification {
    fn from(s: &'static str) -> Self {
        Self::new().message(s)
    }
}

impl From<(NotificationType, &'static str)> for Notification {
    fn from((type_, content): (NotificationType, &'static str)) -> Self {
        Self::new().message(content).with_type(type_)
    }
}

impl From<(NotificationType, SharedString)> for Notification {
    fn from((type_, content): (NotificationType, SharedString)) -> Self {
        Self::new().message(content).with_type(type_)
    }
}

pub struct DefaultIdType;

impl Notification {
    /// Create a new notification.
    ///
    /// The default id is a random UUID.
    pub fn new() -> Self {
        let id: SharedString = uuid::Uuid::new_v4().to_string().into();
        let id = (TypeId::of::<DefaultIdType>(), id.into());

        Self {
            id: id.into(),
            db_id: 0,
            style: StyleRefinement::default(),
            title: None,
            message: None,
            type_: None,
            icon: None,
            icon_img: None,
            autohide: false,
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

        // Dismiss the notification after 0.15s to show the animation.
        cx.spawn(async move |view, cx| {
            Timer::after(Duration::from_secs_f32(0.15)).await;
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

impl EventEmitter<DismissEvent> for Notification {}
impl EventEmitter<UserDismissedEvent> for Notification {}
impl FluentBuilder for Notification {}
impl Styled for Notification {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl Render for Notification {
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

/// A cubic bezier function like CSS `cubic-bezier`.
///
/// Builder:
///
/// https://cubic-bezier.com
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> impl Fn(f32) -> f32 {
    move |t: f32| {
        let one_t = 1.0 - t;
        let one_t2 = one_t * one_t;
        let t2 = t * t;
        let t3 = t2 * t;

        // The Bezier curve function for x and y, where x0 = 0, y0 = 0, x3 = 1, y3 = 1
        let _x = 3.0 * x1 * one_t2 * t + 3.0 * x2 * one_t * t2 + t3;
        let y = 3.0 * y1 * one_t2 * t + 3.0 * y2 * one_t * t2 + t3;

        y
    }
}

/// A list of notifications.
pub struct NotificationList {
    /// Notifications that will be auto hidden.
    pub(crate) notifications: VecDeque<Entity<Notification>>,
    expanded: bool,
    _subscriptions: HashMap<NotificationId, Subscription>,
}

impl NotificationList {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            notifications: VecDeque::new(),
            expanded: false,
            _subscriptions: HashMap::new(),
        }
    }

    pub fn push(
        &mut self,
        notification: impl Into<Notification>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let notification = notification.into();
        let id = notification.id.clone();
        let autohide = notification.autohide;

        println!("inserting with notification id:  {:?}", id);
        // Remove the notification by id, for keep unique.
        self.notifications.retain(|note| note.read(cx).id != id);

        let notification = cx.new(|_| notification);

        let id_for_dismiss = id.clone();
        cx.subscribe(&notification, move |view, _, _: &DismissEvent, cx| {
            view.notifications
                .retain(|note| id_for_dismiss != note.read(cx).id);
            view._subscriptions.remove(&id_for_dismiss);
        })
            .detach();

        let id_for_user_dismiss = id.clone();
        cx.subscribe(
            &notification,
            move |view, _, event: &UserDismissedEvent, cx| {
                cx.emit(UserDismissedEvent { id: event.id });
            },
        )
            .detach();

        self.notifications.push_back(notification.clone());
        if autohide {
            // Sleep for 5 seconds to autohide the notification
            cx.spawn_in(window, async move |_, cx| {
                Timer::after(Duration::from_secs(5)).await;

                if let Err(err) =
                    notification.update_in(cx, |note, window, cx| note.dismiss(window, cx))
                {
                    tracing::error!("failed to auto hide notification: {:?}", err);
                }
            })
                .detach();
        }
        cx.notify();
    }

    pub(crate) fn close(
        &mut self,
        id: impl Into<NotificationId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let id: NotificationId = id.into();
        if let Some(n) = self.notifications.iter().find(|n| n.read(cx).id == id) {
            n.update(cx, |note, cx| note.dismiss(window, cx))
        }
        cx.notify();
    }
    pub fn close_by_key(
        &mut self,
        key: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Build the same composite id used by Notification::id1()
        let id_tuple = (TypeId::of::<Notification>(), key.into());
        self.close(id_tuple, window, cx);
    }

    pub fn clear(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.notifications.clear();
        cx.notify();
    }

    pub fn notifications(&self) -> Vec<Entity<Notification>> {
        self.notifications.iter().cloned().collect()
    }
}

impl EventEmitter<DismissEvent> for NotificationList {}
impl EventEmitter<UserDismissedEvent> for NotificationList {}
impl Render for NotificationList {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let size = window.viewport_size();
        let items = self.notifications.iter().rev().take(10).rev().cloned();

        div().absolute().top_4().right_4().child(
            div()
                .flex()
                .flex_col()
                .id("notification-list")
                .h(size.height - px(8.))
                .on_hover(cx.listener(|view, hovered, _, cx| {
                    view.expanded = *hovered;
                    cx.notify()
                }))
                .gap_3()
                .children(items),
        )
    }
}

// ================= Notification Center (grouped list) =================

#[derive(Clone)]
pub struct NotificationGroupItem {
    pub id: u64,
    pub app_name: SharedString,
    pub time_ago: SharedString,
    pub preview: SharedString,
    pub count: u32,
    pub has_thumbnail: bool,
    // Add a list of all notifications in this group
    pub items: Vec<NotificationItem>,
}

#[derive(Clone)]
pub struct NotificationItem {
    pub id: u64,
    pub db_id: u32,
    pub preview: SharedString,
    pub has_thumbnail: bool,
}

pub struct NotificationCenter {
    pub is_visible: bool,
    groups: Vec<NotificationGroupItem>,
    rows: Vec<RowState>,
    clearing: bool,
    // Track which groups are expanded
    expanded_groups: HashMap<u64, bool>,
    // Track state for individual notification items
    item_states: HashMap<u64, ItemState>,
}

#[derive(Clone, Copy)]
struct RowState {
    id: u64,
    // swipe-to-dismiss state for center rows
    dragging: bool,
    drag_start: Point<gpui::Pixels>,
    drag_dx: f32,
    drag_moved: bool,
    snapping_back: bool,
    snap_from: f32,
    anim_epoch: u64,
    closing: bool,
    close_dir: f32,
}

#[derive(Clone, Copy)]
struct ItemState {
    id: u64,
    dragging: bool,
    drag_start: Point<gpui::Pixels>,
    drag_dx: f32,
    drag_moved: bool,
    snapping_back: bool,
    snap_from: f32,
    anim_epoch: u64,
    closing: bool,
    close_dir: f32,
}

impl RowState {
    fn new(id: u64) -> Self {
        Self {
            id,
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
            closing: false,
            close_dir: 1.0,
        }
    }
}

impl ItemState {
    fn new(id: u64) -> Self {
        Self {
            id,
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
            closing: false,
            close_dir: 1.0,
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

impl NotificationCenter {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            is_visible: false,
            groups: vec![],
            rows: vec![],
            clearing: false,
            expanded_groups: HashMap::new(),
            item_states: HashMap::new(),
        }
    }
}

impl EventEmitter<UserDismissedEvent> for NotificationCenter {}

impl NotificationCenter {
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn bump_row_animation(&mut self, group_id: u64) {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == group_id) {
            row.anim_epoch = row.anim_epoch.wrapping_add(1);
        }
    }
    pub fn add_db_notification(&mut self, notif: DbNotification, cx: &mut Context<Self>) {
        let app_name_formatted = format_notification_name(&notif.app_name);
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let time_ago: SharedString = time_ago(current_timestamp, notif.received_at.unwrap_or(0)).into();
        // Try to find an existing group for this app
        if let Some(group) = self
            .groups
            .iter_mut()
            .find(|g| g.app_name == app_name_formatted)
        {
            let item_has_thumbnail =
                notif.hints.contains_key("image-path") || notif.hints.contains_key("image_path");

            let new_item = NotificationItem {
                id: group.id + 1 + notif.id as u64, // Use group id + db_id as a base for unique UI id
                db_id: notif.id,
                preview: format_notification_body(&notif.summary, &notif.body),
                has_thumbnail: item_has_thumbnail,
            };

            // Insert at index 0 (latest first)
            group.items.insert(0, new_item);
            group.count += 1;

            // Update group preview and thumbnail from the latest notification
            group.preview = format_notification_body(&notif.summary, &notif.body);
            group.has_thumbnail = item_has_thumbnail;
            group.time_ago = format!("· {}", time_ago).into();

            // Move the updated group to the top of the groups list
            let group_id = group.id;
            if let Some(pos) = self.groups.iter().position(|g| g.id == group_id) {
                let g = self.groups.remove(pos);
                self.groups.insert(0, g);
            }
            self.bump_row_animation(group_id);
        } else {
            // Create a new group if it doesn't exist
            let next_id = self.groups.iter().map(|g| g.id).max().unwrap_or(0) + 1000;

            let item_has_thumbnail =
                notif.hints.contains_key("image-path") || notif.hints.contains_key("image_path");

            let new_item = NotificationItem {
                id: next_id + notif.id as u64,
                db_id: notif.id,
                preview: format_notification_body(&notif.summary, &notif.body),
                has_thumbnail: item_has_thumbnail,
            };

            let new_group = NotificationGroupItem {
                id: next_id,
                app_name: app_name_formatted,
                preview: format_notification_body(&notif.summary, &notif.body),
                count: 1,
                has_thumbnail: item_has_thumbnail,
                items: vec![new_item],
                time_ago: format!("· {}", time_ago).into(),
            };

            // Insert at the beginning of groups
            self.groups.insert(0, new_group);

            // Add a corresponding row state
            self.rows.insert(0, RowState::new(next_id));
            // Trigger entrance animation
            if let Some(row) = self.rows.first_mut() {
                row.anim_epoch = row.anim_epoch.wrapping_add(1);
            }
        }

        cx.notify();
    }

    // Call this method to populate from your database
    pub fn load_from_database(
        &mut self,
        notifications: Vec<DbNotification>,
        cx: &mut Context<Self>,
    ) {
        println!(
            "Loading {} notifications from database",
            notifications.len()
        );
        // Group notifications by app_name
        let mut grouped: HashMap<String, Vec<DbNotification>> = HashMap::new();

        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        for notif in notifications {
            grouped
                .entry(notif.app_name.clone())
                .or_insert_with(Vec::new)
                .push(notif);
        }

        // Convert to NotificationGroupItem
        let mut next_id = 1u64;
        self.groups = grouped
            .into_iter()
            .map(|(app_name, mut notifs)| {
                // Sort by id descending (newest first)
                notifs.sort_by(|a, b| b.id.cmp(&a.id));

                let count = notifs.len();
                let first = notifs.first().unwrap();

                // Extract thumbnail from hints if available
                let has_thumbnail = first.hints.contains_key("image-path")
                    || first.hints.contains_key("image_path");

                // Format time ago (you'll need to calculate this based on timestamp)
                let time_ago = format!("· {}", time_ago(current_timestamp, first.received_at.unwrap_or(0)));

                // Create items for the group
                let items: Vec<NotificationItem> = notifs
                    .iter()
                    .map(|n| {
                        let item_has_thumbnail = n.hints.contains_key("image-path")
                            || n.hints.contains_key("image_path");

                        NotificationItem {
                            id: next_id + n.id as u64,
                            db_id: n.id,
                            preview: format_notification_body(&n.summary, &n.body),
                            has_thumbnail: item_has_thumbnail,
                        }
                    })
                    .collect();

                let group_id = next_id;
                next_id += 1000; // Leave space for item IDs

                NotificationGroupItem {
                    id: group_id,
                    app_name: format_notification_name(&first.app_name),
                    time_ago: time_ago.into(),
                    preview: format_notification_body(&first.summary, &first.body),
                    count: count as u32,
                    has_thumbnail,
                    items,
                }
            })
            .collect();

        // Sort groups by the most recent notification ID
        self.groups.sort_by(|a, b| {
            let a_newest = a.items.first().map(|i| i.db_id).unwrap_or(0);
            let b_newest = b.items.first().map(|i| i.db_id).unwrap_or(0);
            b_newest.cmp(&a_newest)
        });

        // Initialize row states
        self.rows = self.groups.iter().map(|g| RowState::new(g.id)).collect();

        cx.notify();
    }
    fn toggle_group(&mut self, group_id: u64, cx: &mut Context<Self>) {
        let is_expanded = self
            .expanded_groups
            .get(&group_id)
            .copied()
            .unwrap_or(false);
        self.expanded_groups.insert(group_id, !is_expanded);
        cx.notify();
    }
}

// Helper function to format notification preview
fn format_notification_name(app_name: &str) -> SharedString {
    if app_name.is_empty() {
        app_name.to_string().into()
    } else {
        // Limit body to reasonable length
        let preview = if app_name.len() > 30 {
            format!("{}...", &app_name[..30])
        } else {
            format!("{}", app_name)
        };
        preview.into()
    }
}

// Helper function to format notification preview
fn format_notification_body(summary: &str, body: &str) -> SharedString {
    if body.is_empty() {
        summary.to_string().into()
    } else {
        // Limit body to reasonable length
        let body_clean = body.replace('\n', " ");
        let preview = if body_clean.len() > 100 {
            format!("{}: {}...", summary, &body_clean[..97])
        } else {
            format!("{}: {}", summary, body_clean)
        };
        preview.into()
    }
}

/// returns a "time ago" string for a given epoch timestamp
fn time_ago(crn_time: u64, ts: u64) -> String {
    let diff = crn_time.saturating_sub(ts);
    match diff {
        0..=59 => "now".to_string(),
        60..=3599 => format!("{}m", diff / 60),
        3600..=86399 => format!("{}hr", diff / 3600),
        _ => format!("{}d", diff / 86_400),
    }
}

impl Render for NotificationCenter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Header - updated styling
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_4()
            .pt_4()
            .pb_3()
            .child(
                div()
                    .text_color(rgb(0xe5e5e5))
                    .text_base()
                    .font_weight(FontWeight::MEDIUM)
                    .child("Notifications"),
            )
            .child(
                div()
                    .id("clear-all")
                    .text_sm()
                    .text_color(rgb(0xa0a0a0))
                    .cursor_pointer()
                    .child("Clear all")
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if this.clearing {
                            return;
                        }

                        // Emit dismissal events for all notifications in all groups
                        for group in &this.groups {
                            for item in &group.items {
                                cx.emit(UserDismissedEvent { id: item.db_id });
                            }
                        }

                        this.clearing = true;
                        cx.notify();

                        cx.spawn(async move |view, cx| {
                            Timer::after(Duration::from_millis(475)).await;
                            cx.update(|cx| {
                                if let Some(view) = view.upgrade() {
                                    view.update(cx, |this, _| {
                                        this.groups.clear();
                                        this.clearing = false;
                                    });
                                }
                            })
                        })
                            .detach();
                    })),
            );

        // Groups list - updated styling
        let mut list = div().flex().flex_col().gap_2p5().px_3().pb_3();
        let clearing = self.clearing;

        for (idx, g) in self.groups.iter().enumerate() {
            let st = self
                .rows
                .iter()
                .find(|s| s.id == g.id)
                .cloned()
                .unwrap_or(RowState::new(g.id));
            let is_expanded = self.expanded_groups.get(&g.id).copied().unwrap_or(false);

            // Main card with stacked appearance using layered divs
            let mut row = div().relative().flex().flex_col();

            // Build stacked effect from back to front (only show when NOT expanded)
            if !is_expanded {
                // Back layer (third card hint) - only if count > 2
                if g.count > 2 {
                    row = row.child(
                        div()
                            .absolute()
                            .top(px(0.0))
                            .left(px(16.0))
                            .right(px(16.0))
                            .h(px(9.0))
                            .rounded_t(px(12.0))
                            .border_t_1()
                            .border_l_1()
                            .border_r_1()
                            .border_color(rgb(0xb87d00))
                            .bg(rgb(0x241b12)),
                    );
                }

                // Middle layer (second card hint) - if count > 1
                if g.count > 1 {
                    row = row.child(
                        div()
                            .absolute()
                            .top(px(6.0))
                            .left(px(8.0))
                            .right(px(8.0))
                            .h(px(9.0))
                            .rounded_t(px(12.0))
                            .border_t_1()
                            .border_l_1()
                            .border_r_1()
                            .border_color(rgb(0xb87d00))
                            .bg(rgb(0x2a2015)),
                    );
                }
            }

            // Calculate top margin for main card based on stack count (only when not expanded)
            let card_top_offset = if is_expanded {
                px(0.0)
            } else if g.count > 2 {
                px(12.0)
            } else if g.count > 1 {
                px(12.0)
            } else {
                px(0.0)
            };

            // Helper function to create a notification card
            let create_card = |item_preview: SharedString,
                               item_has_thumbnail: bool,
                               show_header: bool,
                               item_idx: usize,
                               group_id: u64,
                               item_id: u64| {
                let mut card = div()
                    .id("nc-card")
                    .relative()
                    .border_1()
                    .border_color(rgb(0xb87d00))
                    .bg(rgb(0x2f2217))
                    .rounded(px(12.0))
                    .px_4()
                    .py_3()
                    .flex()
                    .flex_col()
                    .gap_2();

                // Top line: icon + name · time (only for first card or when collapsed)
                if show_header {
                    let top = div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .w(px(24.0))
                                        .h(px(24.0))
                                        .rounded(px(6.0))
                                        .bg(rgb(0xff9500))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            Icon::new(IconName::Application)
                                                .size((px(16.), px(16.)))
                                                .text_color(rgb(0xffffff)),
                                        ),
                                )
                                .child(div().text_sm().text_color(rgb(0xe0e0e0)).child(format!(
                                    "{} {}",
                                    g.app_name,
                                    g.time_ago.clone()
                                ))),
                        )
                        .when(g.count > 0 && !is_expanded, |this| {
                            this.child(
                                div()
                                    .rounded(px(10.0))
                                    .border_1()
                                    .border_color(rgb(0xff9500))
                                    .bg(rgb(0x1a1a1a))
                                    .text_color(rgb(0xff9500))
                                    .text_xs()
                                    .px_2()
                                    .py_0p5()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(format!("{}", g.count)),
                            )
                        });
                    card = card.child(top);
                }

                // Body with preview text and optional thumbnail
                let mut body = div()
                    .text_sm()
                    .text_color(rgb(0xc0c0c0))
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_3();

                body = body.child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .whitespace_normal()
                        .child(item_preview),
                );

                if item_has_thumbnail {
                    body = body.child(
                        div()
                            .w(px(44.0))
                            .h(px(44.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x404040))
                            .flex_shrink_0(),
                    );
                }

                card = card.child(body);

                // Make clickable to expand/collapse (only for first card with count > 1 and not expanded)
                if show_header && g.count > 1 && !is_expanded {
                    card = card
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.toggle_group(group_id, cx);
                        }));
                } else if show_header && g.count > 1 && is_expanded {
                    // Also make it clickable when expanded to allow collapsing
                    card = card
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.toggle_group(group_id, cx);
                        }));
                }

                card
            };

            // Render cards based on expanded state
            if is_expanded && g.items.len() > 1 {
                // Show all notifications in the stack with staggered animation
                for (item_idx, item) in g.items.iter().enumerate() {
                    // Get or create item state
                    let item_state = self
                        .item_states
                        .get(&item.id)
                        .copied()
                        .unwrap_or(ItemState::new(item.id));

                    let mut card = create_card(
                        item.preview.clone(),
                        item.has_thumbnail,
                        item_idx == 0, // Only show header for first card
                        item_idx,
                        g.id,
                        item.id,
                    );

                    // Add swipe handlers for expanded items (not the first one with header)
                    if item_idx > 0 {
                        let item_id = item.id;
                        let item_db_id = item.db_id;
                        let group_id = g.id;

                        card = card
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, e: &MouseDownEvent, _window, cx| {
                                    let state = this
                                        .item_states
                                        .entry(item_id)
                                        .or_insert(ItemState::new(item_id));
                                    state.dragging = true;
                                    state.drag_moved = false;
                                    state.snapping_back = false;
                                    state.drag_start = e.position;
                                    state.drag_dx = 0.0;
                                    cx.notify();
                                }),
                            )
                            .on_mouse_move(cx.listener(
                                move |this, e: &MouseMoveEvent, _window, cx| {
                                    if let Some(state) = this.item_states.get_mut(&item_id) {
                                        if state.dragging {
                                            let dx = e.position.x - state.drag_start.x;
                                            if dx.abs() > px(3.0) {
                                                state.drag_moved = true;
                                            }
                                            state.drag_dx = dx.into();
                                            cx.notify();
                                        }
                                    }
                                },
                            ))
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _e: &MouseUpEvent, _window, cx| {
                                    if let Some(state) = this.item_states.get_mut(&item_id) {
                                        if !state.dragging {
                                            return;
                                        }
                                        state.dragging = false;

                                        let threshold: f32 = 80.0;
                                        let dx = state.drag_dx;
                                        state.drag_dx = 0.0;

                                        if dx.abs() >= threshold {
                                            // Swipe dismiss
                                            state.close_dir = if dx < 0.0 { -1.0 } else { 1.0 };
                                            state.closing = true;
                                            cx.notify();

                                            let removing_item_id = item_id;
                                            let target_group_id = group_id;
                                            cx.spawn(async move |view, cx| {
                                                Timer::after(Duration::from_millis(150)).await;
                                                cx.update(|cx| {
                                                    if let Some(view) = view.upgrade() {
                                                        view.update(cx, |this, cx| {
                                                            // Find the group and remove the item
                                                            if let Some(group) = this
                                                                .groups
                                                                .iter_mut()
                                                                .find(|g| g.id == target_group_id)
                                                            {
                                                                if let Some(pos) = group
                                                                    .items
                                                                    .iter()
                                                                    .position(|i| {
                                                                        i.id == removing_item_id
                                                                    })
                                                                {
                                                                    group.items.remove(pos);
                                                                    group.count = group
                                                                        .count
                                                                        .saturating_sub(1);

                                                                    // Emit user dismissed event
                                                                    cx.emit(UserDismissedEvent {
                                                                        id: item_db_id,
                                                                    });

                                                                    // Update the preview to the first remaining item
                                                                    if let Some(first) =
                                                                        group.items.first()
                                                                    {
                                                                        group.preview =
                                                                            first.preview.clone();
                                                                        group.has_thumbnail =
                                                                            first.has_thumbnail;
                                                                    }

                                                                    // If only one item left, collapse the group
                                                                    if group.items.len() <= 1 {
                                                                        this.expanded_groups
                                                                            .insert(
                                                                                target_group_id,
                                                                                false,
                                                                            );
                                                                    }
                                                                }
                                                            }
                                                            this.item_states
                                                                .remove(&removing_item_id);
                                                            cx.notify();
                                                        });
                                                    }
                                                })
                                            })
                                                .detach();
                                        } else {
                                            // Snap back
                                            state.snapping_back = true;
                                            state.snap_from = dx;
                                            state.anim_epoch = state.anim_epoch.wrapping_add(1);
                                            let target_item_id = item_id;
                                            let epoch = state.anim_epoch;
                                            cx.notify();

                                            cx.spawn(async move |view, cx| {
                                                Timer::after(Duration::from_millis(200)).await;
                                                cx.update(|cx| {
                                                    if let Some(view) = view.upgrade() {
                                                        view.update(cx, |this, _| {
                                                            if let Some(state) = this
                                                                .item_states
                                                                .get_mut(&target_item_id)
                                                            {
                                                                if state.anim_epoch == epoch {
                                                                    state.snapping_back = false;
                                                                    state.snap_from = 0.0;
                                                                }
                                                            }
                                                        });
                                                    }
                                                })
                                            })
                                                .detach();
                                        }
                                    }
                                }),
                            );
                    }

                    let card_with_margin = if item_idx == 0 {
                        card.mt(card_top_offset)
                    } else {
                        card.mt_2() // spacing between expanded cards
                    };

                    // Animation for expanded items
                    let anim_id = ElementId::NamedInteger(
                        "nc-expanded-item".into(),
                        item_state.anim_epoch * 100000 + item.id,
                    );

                    row = row.child(
                        card_with_margin.with_animation(
                            anim_id,
                            Animation::new(Duration::from_millis(300))
                                .with_easing(cubic_bezier(0.4, 0.0, 0.2, 1.0)),
                            move |this, delta| {
                                if item_state.closing {
                                    // Slide out
                                    let x_offset = delta * px(120.) * item_state.close_dir;
                                    let opacity = 1.0 - delta;
                                    this.left(x_offset).opacity(opacity)
                                } else if item_state.dragging {
                                    // Follow finger
                                    let dist = item_state.drag_dx.abs().min(180.0);
                                    let fade = (dist / 180.0) * 0.6;
                                    let opacity = 1.0 - fade;
                                    this.left(px(item_state.drag_dx)).opacity(opacity)
                                } else if item_state.snapping_back && item_state.snap_from != 0.0 {
                                    // Snap back
                                    let start_dist = item_state.snap_from.abs().min(180.0);
                                    let start_fade = (start_dist / 180.0) * 0.6;
                                    let start_opacity = 1.0 - start_fade;
                                    let x = px(item_state.snap_from * (1.0 - delta));
                                    let opacity = start_opacity + (1.0 - start_opacity) * delta;
                                    this.left(x).opacity(opacity)
                                } else {
                                    // Staggered fade-in on expand
                                    let delay = (item_idx as f32) * 0.08;
                                    let progress =
                                        ((delta - delay) / (1.0 - delay)).clamp(0.0, 1.0);
                                    let y_offset = px(-20.0) * (1.0 - progress);
                                    this.top(y_offset).opacity(progress)
                                }
                            },
                        ),
                    );
                }
            } else {
                // Collapsed state - show only the first notification
                let card = create_card(
                    g.preview.clone(),
                    g.has_thumbnail,
                    true,
                    0,
                    g.id,
                    g.items.first().map(|i| i.id).unwrap_or(0),
                )
                    .mt(card_top_offset);

                // Add swipe handlers for collapsed card
                let row_id = g.id;
                let card = card
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, e: &MouseDownEvent, _window, cx| {
                            if let Some(st) = this.rows.iter_mut().find(|s| s.id == row_id) {
                                st.dragging = true;
                                st.drag_moved = false;
                                st.snapping_back = false;
                                st.drag_start = e.position;
                                st.drag_dx = 0.0;
                                cx.notify();
                            }
                        }),
                    )
                    .on_mouse_move(cx.listener(move |this, e: &MouseMoveEvent, _window, cx| {
                        if let Some(st) = this.rows.iter_mut().find(|s| s.id == row_id) {
                            if st.dragging {
                                let dx = e.position.x - st.drag_start.x;
                                if dx.abs() > px(3.0) {
                                    st.drag_moved = true;
                                }
                                st.drag_dx = dx.into();
                                cx.notify();
                            }
                        }
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(move |this, _e: &MouseUpEvent, window, cx| {
                            if let Some(st) = this.rows.iter_mut().find(|s| s.id == row_id) {
                                if !st.dragging {
                                    return;
                                }
                                st.dragging = false;

                                // Check if this was a tap (no significant drag)
                                if !st.drag_moved {
                                    st.drag_dx = 0.0;
                                    return; // Let the on_click handler deal with it
                                }

                                let threshold: f32 = 80.0;
                                let dx = st.drag_dx;
                                st.drag_dx = 0.0;
                                if dx.abs() >= threshold {
                                    st.close_dir = if dx < 0.0 { -1.0 } else { 1.0 };
                                    st.closing = true;
                                    cx.notify();
                                    let removing_id = row_id;
                                    cx.spawn(async move |view, cx| {
                                        Timer::after(Duration::from_millis(150)).await;
                                        cx.update(|cx| {
                                            if let Some(view) = view.upgrade() {
                                                view.update(cx, |this, cx| {
                                                    if let Some(pos) = this
                                                        .groups
                                                        .iter()
                                                        .position(|g| g.id == removing_id)
                                                    {
                                                        let group = this.groups.remove(pos);
                                                        // Emit user dismissed event for all items in the group
                                                        for item in group.items {
                                                            cx.emit(UserDismissedEvent {
                                                                id: item.db_id,
                                                            });
                                                        }
                                                    }
                                                    if let Some(pos) = this
                                                        .rows
                                                        .iter()
                                                        .position(|s| s.id == removing_id)
                                                    {
                                                        this.rows.remove(pos);
                                                    }
                                                });
                                            }
                                        })
                                    })
                                        .detach();
                                } else {
                                    st.snapping_back = true;
                                    st.snap_from = dx;
                                    st.anim_epoch = st.anim_epoch.wrapping_add(1);
                                    let target_id = row_id;
                                    let epoch = st.anim_epoch;
                                    cx.notify();
                                    cx.spawn(async move |view, cx| {
                                        Timer::after(Duration::from_millis(200)).await;
                                        cx.update(|cx| {
                                            if let Some(view) = view.upgrade() {
                                                view.update(cx, |this, _| {
                                                    if let Some(st) = this
                                                        .rows
                                                        .iter_mut()
                                                        .find(|s| s.id == target_id)
                                                    {
                                                        if st.anim_epoch == epoch {
                                                            st.snapping_back = false;
                                                            st.snap_from = 0.0;
                                                        }
                                                    }
                                                });
                                            }
                                        })
                                    })
                                        .detach();
                                }
                            }
                        }),
                    );

                row = row.child(card);
            }

            // Animation wrapper for the entire row
            let i = idx as u64;
            list = list.child(
                row.with_animation(
                    ElementId::NamedInteger(
                        "nc-row".into(),
                        if clearing {
                            1_000_000 + st.anim_epoch + i
                        } else {
                            st.anim_epoch + i
                        },
                    ),
                    Animation::new(Duration::from_millis(450))
                        .with_easing(cubic_bezier(0.4, 0.0, 0.2, 1.0)),
                    move |this, delta| {
                        if clearing {
                            let opacity = 1.0 - delta;
                            this.opacity(opacity)
                        } else if st.closing {
                            let x_offset = delta * px(120.) * st.close_dir;
                            let opacity = 1.0 - delta;
                            this.left(x_offset).opacity(opacity)
                        } else if st.dragging {
                            let dist = st.drag_dx.abs().min(180.0);
                            let fade = (dist / 180.0) * 0.6;
                            let opacity = 1.0 - fade;
                            this.left(px(st.drag_dx)).opacity(opacity)
                        } else if st.snapping_back && st.snap_from != 0.0 {
                            let start_dist = st.snap_from.abs().min(180.0);
                            let start_fade = (start_dist / 180.0) * 0.6;
                            let start_opacity = 1.0 - start_fade;
                            let x = px(st.snap_from * (1.0 - delta));
                            let opacity = start_opacity + (1.0 - start_opacity) * delta;
                            this.left(x).opacity(opacity)
                        } else {
                            let step = 0.12;
                            let start = (i as f32) * step;
                            let denom = (1.0 - start).max(0.0001);
                            let local = ((delta - start) / denom).clamp(0.0, 1.0);
                            this.opacity(local)
                        }
                    },
                ),
            );
        }

        // Main container
        div()
            .absolute()
            .top(px(60.0))
            .left(px(16.0))
            .right(px(16.0))
            .child(
                div()
                    .rounded(px(14.0))
                    .bg(rgb(0x1a1a1a))
                    .border_1()
                    .border_color(rgb(0x2a2a2a))
                    .shadow_lg()
                    .child(header)
                    .child(list),
            )
    }
}
