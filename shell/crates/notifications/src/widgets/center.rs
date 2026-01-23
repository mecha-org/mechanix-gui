use crate::helper::{
    cubic_bezier, format_notification_body, format_notification_name, format_notification_summary,
    time_ago,
};

use crate::widgets::notification::{
    DbNotification, NotificationId, NotificationUi, UserDismissedEvent,
};
use commons::widgets::{CornerRadii, WingSide, wing};
use gpui::prelude::FluentBuilder;
use gpui::*;
use icons::prelude::Icons;
use smol::Timer;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    any::TypeId,
    collections::{HashMap, VecDeque},
    time::Duration,
};
use theme::ActiveTheme;
use theme::prelude::{AlphaExt, Fonts};

use settings::prelude::Settings;

const COLLAPSED_CARD_HEIGHT: f32 = 100.0; // header + body (max 2 lines)
const EXPANDED_FIRST_HEIGHT: f32 = 100.0; // first card (same as collapsed)
const EXPANDED_ITEM_HEIGHT: f32 = 100.0; // body-only cards
const CARD_GAP: f32 = 8.0; // mt_2()
const GROUP_GAP: f32 = 10.0; // gap_2p5()
const LIST_PADDING_TOP: f32 = 12.0;
const LIST_PADDING_BOTTOM: f32 = 12.0;

const ANIMATION_DURATION_MS: f32 = 250.0;
const ANIMATION_FRAME_MS: u64 = 16;


pub struct DragInfo {
    pub position: Point<Pixels>,
}

impl DragInfo {
    fn new() -> Self {
        Self {
            position: Point::default(),
        }
    }

    fn position(mut self, pos: Point<Pixels>) -> Self {
        self.position = pos;
        self
    }
}

impl Render for DragInfo {
    fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
        Empty
    }
}

/// A list of notifications.
pub struct NotificationList {
    /// Notifications that will be auto hidden.
    pub(crate) notifications: VecDeque<Entity<NotificationUi>>,
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
        notification: impl Into<NotificationUi>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let notification = notification.into();
        let id = notification.id.clone();
        let autohide = notification.autohide;
        let expire_timeout = notification.expire_timeout;

        println!("inserting with notification id:  {:?}", id);
        // Remove the notification by id, for keep unique.
        self.notifications.retain(|note| note.read(cx).id != id);

        let notification = cx.new(|_| notification);

        let id_for_dismiss = id.clone();
        cx.subscribe(&notification, move |view, _, _: &DismissEvent, cx| {
            view.notifications
                .retain(|note| id_for_dismiss != note.read(cx).id);
            view._subscriptions.remove(&id_for_dismiss);

            cx.notify();
        })
        .detach();

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
                Timer::after(expire_timeout).await;

                if let Err(err) =
                    notification.update_in(cx, |note, window, cx| note.dismiss(window, cx))
                {
                    eprintln!("failed to auto hide notification: {:?}", err);
                }
            })
            .detach();
        }
        cx.notify();
    }

    pub(crate) fn close(
        &mut self,
        db_id: u32,
        id: impl Into<NotificationId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let id: NotificationId = id.into();
        if let Some(n) = self
            .notifications
            .iter()
            .find(|n| n.read(cx).db_id == db_id)
        {
            println!("Closing notification with db_id: {}", db_id);
            n.update(cx, |note, cx| note.dismiss(window, cx))
        } else {
            println!("Notification with db_id: {} not found", db_id);
        }
        cx.notify();
    }
    pub fn close_by_key(
        &mut self,
        key: impl Into<ElementId>,
        db_id: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Build the same composite id used by Notification::id1()
        let id_tuple = (TypeId::of::<NotificationUi>(), key.into());
        self.close(db_id, id_tuple, window, cx);
    }

    pub fn clear(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.notifications.clear();
        cx.notify();
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
    pub app_summary: SharedString,
    pub time_ago: SharedString,
    pub preview: SharedString,
    pub count: u32,
    pub has_thumbnail: bool,
    pub icon_path: Option<std::path::PathBuf>,
    // Add a list of all notifications in this group
    pub items: Vec<NotificationItem>,
}

#[derive(Clone)]
pub struct NotificationItem {
    pub id: u64,
    pub db_id: u32,
    pub preview: SharedString,
    pub has_thumbnail: bool,
    pub icon_path: Option<std::path::PathBuf>,
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
    pub position: f32,
    pub drag_offset: Option<f32>,
    pub drag_start_pos: f32,
    pub scroll_offset: Pixels,
    pub is_dragging: bool,
    pub drag_start_y: Pixels,
    pub last_scroll_offset: Pixels,
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

impl NotificationCenter {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            is_visible: false,
            groups: vec![],
            rows: vec![],
            clearing: false,
            expanded_groups: HashMap::new(),
            item_states: HashMap::new(),
            position: Self::closed_pos(_cx), // Default closed position
            drag_offset: None,
            drag_start_pos: 0.0,
            scroll_offset: px(0.0),
            is_dragging: false,
            drag_start_y: px(0.0),
            last_scroll_offset: px(0.0),
        }
    }

    fn reset_scroll_state(&mut self) {
        self.scroll_offset = px(0.);
        self.last_scroll_offset = px(0.);
        self.drag_start_y = px(0.);
        self.is_dragging = false;
    }

    fn calculate_scroll_bounds(
        &self,
        cx: &Context<Self>,
        content_height: Pixels,
    ) -> (Pixels, Pixels) {
        let settings = Settings::global(cx).notifications.clone();
        let notifications_center_size = settings.layer_shell.size;
        let navbar_size = settings.navbar_size;

        let container_height = notifications_center_size.height - navbar_size.height - px(1.5);

        // let container_height = px(0.0);
        if content_height <= container_height {
            return (px(0.0), px(0.0));
        }

        let max_scroll = px(0.0);
        let min_scroll = container_height - content_height;
        (min_scroll, max_scroll)
    }

    fn estimated_group_height(&self, g: &NotificationGroupItem) -> Pixels {
        let is_expanded = self.expanded_groups.get(&g.id).copied().unwrap_or(false);

        let mut total = px(0.0);

        if is_expanded && g.items.len() > 1 {
            // first card
            total += px(EXPANDED_FIRST_HEIGHT);

            // rest
            let extra = g.items.len() - 1;
            total += px(extra as f32 * EXPANDED_ITEM_HEIGHT);
            total += px(extra as f32 * CARD_GAP);
        } else {
            // collapsed
            total += px(COLLAPSED_CARD_HEIGHT);
        }
        total
    }

    pub fn estimated_content_height(&self) -> Pixels {
        if self.groups.is_empty() {
            return px(0.0);
        }

        let mut total = px(LIST_PADDING_TOP + LIST_PADDING_BOTTOM);

        for (i, g) in self.groups.iter().enumerate() {
            total += self.estimated_group_height(g);

            if i + 1 < self.groups.len() {
                total += px(GROUP_GAP);
            }
        }
        total
    }

    fn reclamp_scroll(&mut self, cx: &mut Context<Self>) {
        let content_height = self.estimated_content_height();
        let (min, max) = self.calculate_scroll_bounds(cx, content_height);

        self.scroll_offset = self.scroll_offset.clamp(min, max);
        self.last_scroll_offset = self.scroll_offset;
    }

    fn on_drag_move(
        &mut self,
        event: &DragMoveEvent<DragInfo>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_dragging {
            return;
        }

        let delta_y = event.event.position.y - self.drag_start_y;
        let new_scroll_offset = self.last_scroll_offset + delta_y;
        let content_height = self.estimated_content_height();
        let (min_scroll, max_scroll) = self.calculate_scroll_bounds(cx, content_height);

        self.scroll_offset = new_scroll_offset.clamp(min_scroll, max_scroll);
        cx.notify();
    }

    fn on_drop(&mut self, _: &DragMoveEvent<DragInfo>, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
        self.last_scroll_offset = self.scroll_offset;
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
        let app_summary = format_notification_summary(&notif.summary);
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let time_ago: SharedString =
            time_ago(current_timestamp, notif.received_at.unwrap_or(0)).into();
        // Try to find an existing group for this app
        if let Some(group) = self
            .groups
            .iter_mut()
            .find(|g| g.app_summary == app_summary)
        {
            let item_has_thumbnail =
                notif.hints.contains_key("image-path") || notif.hints.contains_key("image_path");

            let mut new_item = NotificationItem {
                id: group.id + 1 + notif.id as u64, // Use group id + db_id as a base for unique UI id
                db_id: notif.id,
                preview: format_notification_body(&app_name_formatted, &notif.body),
                has_thumbnail: item_has_thumbnail,
                icon_path: None,
            };

            if let Some(image_path) = notif
                .hints
                .get("image-path")
                .or_else(|| notif.hints.get("image_path"))
            {
                new_item.icon_path = Some(std::path::PathBuf::from(image_path));
            }

            // Insert at index 0 (latest first)
            group.items.insert(0, new_item);
            group.count += 1;

            // Update group preview and thumbnail from the latest notification
            group.preview = format_notification_body(&app_name_formatted, &notif.body);
            group.has_thumbnail = item_has_thumbnail;
            if let Some(image_path) = notif
                .hints
                .get("image-path")
                .or_else(|| notif.hints.get("image_path"))
            {
                group.icon_path = Some(std::path::PathBuf::from(image_path));
            }
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

            let mut new_item = NotificationItem {
                id: next_id + notif.id as u64,
                db_id: notif.id,
                preview: format_notification_body(&notif.app_name, &notif.body),
                has_thumbnail: item_has_thumbnail,
                icon_path: None,
            };

            if let Some(image_path) = notif
                .hints
                .get("image-path")
                .or_else(|| notif.hints.get("image_path"))
            {
                new_item.icon_path = Some(std::path::PathBuf::from(image_path));
            }

            let mut new_group = NotificationGroupItem {
                id: next_id,
                app_name: app_name_formatted,
                app_summary,
                preview: format_notification_body(&notif.app_name, &notif.body),
                count: 1,
                has_thumbnail: item_has_thumbnail,
                items: vec![new_item],
                time_ago: format!("· {}", time_ago).into(),
                icon_path: None,
            };

            if let Some(image_path) = notif
                .hints
                .get("image-path")
                .or_else(|| notif.hints.get("image_path"))
            {
                new_group.icon_path = Some(std::path::PathBuf::from(image_path));
            }

            // Insert at the beginning of groups
            self.groups.insert(0, new_group);

            // Add a corresponding row state
            self.rows.insert(0, RowState::new(next_id));
            // Trigger entrance animation
            if let Some(row) = self.rows.first_mut() {
                row.anim_epoch = row.anim_epoch.wrapping_add(1);
            }
        }

        self.reclamp_scroll(cx);
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
            let app_summary = format_notification_summary(&notif.summary).to_string();
            grouped
                .entry(app_summary.clone())
                .or_insert_with(Vec::new)
                .push(notif);
        }

        // Convert to NotificationGroupItem
        let mut next_id = 1u64;
        self.groups = grouped
            .into_iter()
            .map(|(_app_name, mut notifs)| {
                // Sort by id descending (newest first)
                notifs.sort_by(|a, b| b.id.cmp(&a.id));

                let count = notifs.len();
                let first = notifs.first().unwrap();

                // Extract thumbnail from hints if available
                let has_thumbnail = first.hints.contains_key("image-path")
                    || first.hints.contains_key("image_path");

                // Format time ago (you'll need to calculate this based on timestamp)
                let time_ago = format!(
                    "· {}",
                    time_ago(current_timestamp, first.received_at.unwrap_or(0))
                );
                let app_summary = format_notification_summary(&first.summary);
                // Create items for the group
                let items: Vec<NotificationItem> = notifs
                    .iter()
                    .map(|n| {
                        let item_has_thumbnail = n.hints.contains_key("image-path")
                            || n.hints.contains_key("image_path");

                        let mut item = NotificationItem {
                            id: next_id + n.id as u64,
                            db_id: n.id,
                            preview: format_notification_body(&n.app_name, &n.body),
                            has_thumbnail: item_has_thumbnail,
                            icon_path: None,
                        };

                        if let Some(image_path) = n
                            .hints
                            .get("image-path")
                            .or_else(|| n.hints.get("image_path"))
                        {
                            item.icon_path = Some(std::path::PathBuf::from(image_path));
                        }
                        item
                    })
                    .collect();

                let group_id = next_id;
                next_id += 1000; // Leave space for item IDs

                let mut group = NotificationGroupItem {
                    id: group_id,
                    app_name: format_notification_name(&first.app_name),
                    app_summary,
                    time_ago: time_ago.into(),
                    preview: format_notification_body(&first.app_name, &first.body),
                    count: count as u32,
                    has_thumbnail,
                    items,
                    icon_path: None,
                };

                if let Some(image_path) = first
                    .hints
                    .get("image-path")
                    .or_else(|| first.hints.get("image_path"))
                {
                    group.icon_path = Some(std::path::PathBuf::from(image_path));
                }
                group
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
        self.reclamp_scroll(cx);
        cx.notify();
    }

    pub fn remove_db_notification(&mut self, db_id: u32, cx: &mut Context<Self>) {
        let mut group_to_remove = None;

        for (group_pos, group) in self.groups.iter_mut().enumerate() {
            if let Some(item_pos) = group.items.iter().position(|i| i.db_id == db_id) {
                let item = group.items.remove(item_pos);
                group.count = group.count.saturating_sub(1);
                self.item_states.remove(&item.id);

                if group.items.is_empty() {
                    group_to_remove = Some(group_pos);
                } else {
                    // Update group preview to the next available item
                    if let Some(first) = group.items.first() {
                        group.preview = first.preview.clone();
                        group.has_thumbnail = first.has_thumbnail;
                        group.icon_path = first.icon_path.clone();
                    }
                }
                break;
            }
        }

        if let Some(group_pos) = group_to_remove {
            let group = self.groups.remove(group_pos);
            self.rows.retain(|r| r.id != group.id);
            self.expanded_groups.remove(&group.id);
        }

        cx.notify();
    }

    pub fn closed_pos(cx: &Context<Self>) -> f32 {
        let settings = Settings::global(cx).notifications.clone();
        let size = settings.layer_shell.size;
        let navbar = settings.navbar_size;

        (size.height - navbar.height).into()
    }

    pub fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.position;
        let change = target - start;
        let start_time = std::time::Instant::now();
        let closed_pos = Self::closed_pos(cx);

        if target == 0.0 {
            self.is_visible = true;
        } else if target == closed_pos {
            self.is_visible = false;
        }

        cx.spawn(
            async move |this: WeakEntity<NotificationCenter>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    // Check if animation is done
                    if elapsed >= ANIMATION_DURATION_MS {
                        this.update(cx, |this, cx| {
                            this.position = target;
                            if target == 0.0 {
                                this.is_visible = true;
                            } else if target == closed_pos {
                                this.is_visible = false;
                            }
                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / ANIMATION_DURATION_MS).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3);
                    let current = start + (change * ease);

                    this.update(cx, |this, cx| {
                        this.position = current;
                        if current < (closed_pos / 2.0) {
                            this.is_visible = true;
                        } else {
                            this.is_visible = false;
                        }
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(ANIMATION_FRAME_MS))
                        .await;
                }
            },
        )
        .detach();
    }
}

impl Render for NotificationCenter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = Settings::global(cx).notifications.clone();
        let notifications_center_size = settings.layer_shell.size;
        let navbar_size = settings.navbar_size;
        let input_regions = settings.input_regions.clone();
        let primary_font = Fonts::global(cx).primary.clone();
        let colors = cx.theme().colors.clone();

        let open_y = 0.;
        let closed_y = Self::closed_pos(cx);

        let threshold_px = 40.;

        div()
            .w_full()
            .h_full()
            .font_family(primary_font)
            .on_mouse_move(
                cx.listener(move |this, event: &MouseMoveEvent, _window, cx| {
                    if let Some(offset) = this.drag_offset {
                        let new_y = event.position.y.to_f64() as f32 - offset;
                        this.position = new_y.clamp(open_y, closed_y);
                        cx.notify();
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.drag_offset.is_some() {
                        this.drag_offset = None;

                        let target;
                        let started_closed = this.drag_start_pos > (closed_y / 2.0);

                        if started_closed {
                            if this.position < (closed_y - threshold_px) {
                                target = open_y;
                            } else {
                                target = closed_y;
                            }
                        } else {
                            if this.position > (open_y + threshold_px) {
                                target = closed_y;
                            } else {
                                target = open_y;
                            }
                        }
                        this.snap_to(target, cx);
                        cx.notify();
                    }
                }),
            )
            .when(!self.is_visible, |this| {
                this.child(
                    div()
                        .id("input-region")
                        .absolute()
                        .top(input_regions.minimized.origin.y)
                        .left(input_regions.minimized.origin.x)
                        .w(input_regions.minimized.size.width)
                        .h(input_regions.minimized.size.height)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                                cx.stop_propagation();
                                this.drag_start_pos = this.position;
                                this.drag_offset =
                                    Some(event.position.y.to_f64() as f32 - this.position);
                                cx.notify();
                            }),
                        ),
                )
            })
            .child(
                div()
                    .w_full()
                    .h_full()
                    .absolute()
                    .top(px(self.position))
                    .child(div().id("left-wing").child({
                        let mut w = wing()
                            .w(notifications_center_size.width)
                            .h(notifications_center_size.height)
                            .border_color(colors.background_700)
                            .flex()
                            .flex_col()
                            .justify_end()
                            .items_end()
                            .bg(if self.is_visible {
                                colors.background_1000
                            } else {
                                colors.background_800
                            })
                            .child(self.render_content(window, cx));

                        w.upper_wing_size(size(navbar_size.width, navbar_size.height));
                        w.upper_wing_side(WingSide::Left);
                        w.border_width(px(1.0));
                        w.corner_radii(CornerRadii {
                            top_left: px(8.0),
                            top_right: px(8.0),
                            bottom_right: px(0.0),
                            bottom_left: px(0.0),
                        });
                        w
                    })),
            )
    }
}

pub struct NotificationWidget {
    pub center: Entity<NotificationCenter>,
    pub notification_list: Entity<NotificationList>,
}

impl NotificationWidget {
    pub fn new(
        center: Entity<NotificationCenter>,
        notification_list: Entity<NotificationList>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            center,
            notification_list,
        }
    }
}

impl Render for NotificationWidget {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = Settings::global(cx).notifications.clone();
        let notifications_center_size = settings.layer_shell.size;
        let navbar_size = settings.navbar_size;
        let input_regions = settings.input_regions.clone();

        let center_is_visible = self.center.read(cx).is_visible;
        let list_is_empty = self.notification_list.read(cx).notifications.is_empty();

        if center_is_visible && !list_is_empty {
            let notifications_to_move: Vec<DbNotification> = self
                .notification_list
                .read(cx)
                .notifications
                .iter()
                .filter_map(|n| n.read(cx).db_notification.clone())
                .collect();

            if !notifications_to_move.is_empty() {
                self.center.update(cx, |center, cx| {
                    for notif in notifications_to_move {
                        center.add_db_notification(notif, cx);
                    }

                    center.reclamp_scroll(cx);
                    cx.notify();
                });
                self.notification_list.update(cx, |list, cx| {
                    list.notifications.clear();
                    cx.notify();
                });
            }
        }

        let center = self.center.read(cx);
        let list = self.notification_list.read(cx);

        let mut regions = Vec::new();

        if center.is_visible || !list.notifications.is_empty() {
            regions.push(Bounds {
                origin: input_regions.maximized.origin,
                size: input_regions.maximized.size,
            });
        } else {
            regions.push(Bounds {
                origin: input_regions.minimized.origin,
                size: input_regions.minimized.size,
            });
        }
        window.set_input_regions(Some(regions));

        div()
            .relative()
            .size_full()
            .child(self.notification_list.clone())
            .child(self.center.clone())
    }
}

impl NotificationCenter {
    fn render_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let primary_font = Fonts::global(cx).primary.clone();

        let icons = Icons::global(cx).notifications.clone();
        let settings = Settings::global(cx).notifications.clone();
        let notifications_center_size = settings.layer_shell.size;
        let navbar_size = settings.navbar_size;
        // Header - updated styling
        let mut header = div()
            .flex()
            .w_full()
            .h_full()
            .flex_row()
            .items_center()
            .justify_between()
            .px_4()
            .pt_4()
            .pb_3()
            .font_family(primary_font)
            .child(
                div()
                    .text_color(colors.foreground_300)
                    .text_base()
                    .font_weight(FontWeight::MEDIUM)
                    .text_size(px(24.0))
                    .child("Notifications"),
            );

        if !self.groups.is_empty() {
            header = header.child(
                div()
                    .id("clear-all")
                    .text_size(px(16.0))
                    .text_color(colors.foreground_200)
                    .font_weight(FontWeight::MEDIUM)
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
        }

        // Groups list - updated styling
        let mut list = div().size_full().flex().flex_col().gap_2p5().px_3().pb_3();
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
            let mut row = div().relative().overflow_hidden().flex().flex_col();

            // Show multiple wings (only show when NOT expanded)
            if !is_expanded {
                // Back layer (third card hint) - only if count > 2
                if g.count > 2 {
                    let mut outer_wing = wing()
                        .relative()
                        .overflow_hidden()
                        .w(px(0.0))
                        .left(navbar_size.width)
                        .border_1()
                        .bottom(px(-4.0))
                        .border_color(colors.accent_200.with_alpha(0.6))
                        .bg(colors.background_1000);

                    outer_wing.upper_wing_size(Size::new(px(20.0), navbar_size.height));
                    outer_wing.include_upper_wing_in_bounds(true);
                    outer_wing.border_width(px(1.0));

                    let mut inner_wing = wing()
                        .relative()
                        .overflow_hidden()
                        .w_full()
                    .bg(if self.is_visible {
                            colors.accent_200.with_alpha(0.2)
                        } else {
                            colors.accent_200.with_alpha(0.1)
                        });

                    inner_wing.upper_wing_size(Size::new(px(20.0), navbar_size.height));
                    inner_wing.upper_wing_side(WingSide::Left);
                    inner_wing.include_upper_wing_in_bounds(true);
                    inner_wing.border_width(px(1.0));

                    let inner = inner_wing.into_any();
                    row = row.child(outer_wing.child(inner).into_any());
                }

                // Middle layer (second card hint) - only if count > 1
                if g.count > 1 {
                    let mut outer_wing = wing()
                        .relative()
                        .overflow_hidden()
                        .w(px(0.0))
                        .left(navbar_size.width - px(20.0))
                        .bottom(px(-2.0))
                        .border_1()
                        .border_color(colors.accent_200.with_alpha(0.6))
                        .bg(colors.background_1000);

                    outer_wing.upper_wing_size(Size::new(px(20.0), navbar_size.height));
                    outer_wing.include_upper_wing_in_bounds(true);
                    outer_wing.border_width(px(1.0));

                    let mut inner_wing = wing()
                        .relative()
                        .overflow_hidden()
                        .w_full()
                        .bg(if self.is_visible {
                            colors.accent_200.with_alpha(0.2)
                        } else {
                            colors.accent_200.with_alpha(0.1)
                        });

                    inner_wing.upper_wing_size(Size::new(px(20.0), navbar_size.height));
                    inner_wing.include_upper_wing_in_bounds(true);
                    inner_wing.border_width(px(1.0));

                    let inner = inner_wing.into_any();
                    row = row.child(outer_wing.child(inner).into_any());
                }
            }

            // Calculate top margin for main card based on stack count (only when not expanded)
            let card_top_offset = px(0.0);

            // Helper function to create a notification card
            let mut create_card = |item_preview: SharedString,
                                   item_has_thumbnail: bool,
                                   item_icon_path: Option<std::path::PathBuf>,
                                   show_header: bool,
                                   item_idx: usize,
                                   group_id: u64,
                                   item_id: u64| {
                let mut content = div().flex().flex_col().gap_2();
                let default_icon: SharedString =
                    icons.application.to_string_lossy().to_string().into();

                // Top line: icon + name · time (only for first card or when collapsed)
                if show_header {
                    let top = div()
                        .relative()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .pt(px(-10.0))
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
                                        .bg(colors.accent_200)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .when_some(
                                            item_icon_path.clone().or_else(|| g.icon_path.clone()),
                                            |this, path| {
                                                this.child(img(path).size_full().rounded(px(6.0)))
                                            },
                                        )
                                        .when(
                                            item_icon_path.is_none() && g.icon_path.is_none(),
                                            |this| {
                                                this.child(
                                                    svg()
                                                        .external_path(default_icon)
                                                        .w(px(16.))
                                                        .h(px(16.))
                                                        .text_color(colors.foreground_0),
                                                )
                                            },
                                        ),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(colors.foreground_500)
                                        .whitespace_normal()
                                        .child(format!("{}", g.app_summary,))
                                        .text_ellipsis()
                                        .w(if item_icon_path.is_none() {
                                            px(100.)
                                        } else {
                                            px(80.)
                                        }),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(colors.foreground_900)
                                        .whitespace_normal()
                                        .child(format!(" {}", g.time_ago.clone())),
                                ),
                        )
                        .when(g.count > 0, |this| {
                            if is_expanded {
                                this.child(
                                    div()
                                        .mt(px(-12.0))
                                        .left(px(10.0))
                                        .rounded(px(4.0))
                                        .bg(colors.accent_200.with_alpha(0.2))
                                        .text_color(colors.accent_200)
                                        .text_size(px(14.0))
                                        .px_2()
                                        .h(px(24.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("︿"),
                                )
                            } else {
                                this.child(
                                    div()
                                        .mt(px(-12.0))
                                        .left(px(10.0))
                                        .rounded(px(4.0))
                                        .bg(colors.accent_200.with_alpha(0.2))
                                        .text_color(colors.accent_200)
                                        .text_size(px(16.0))
                                        .px_2()
                                        // .py_0p3()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(if g.count > 10 {
                                            SharedString::from("10+")
                                        } else {
                                            g.count.to_string().into()
                                        }),
                                )
                            }
                        });
                    content = content.child(top);
                }

                // Body with preview text and optional thumbnail
                let mut body = div()
                    .text_size(px(16.0))
                    .text_color(colors.foreground_300)
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

                // let parsed = parse_markup(item_preview.as_ref());
                // body = body.child(
                //     div()
                //         .flex_1()
                //         .min_w(px(0.0))
                //         .child(render_markup(&parsed, cx))
                // );

                // if item_has_thumbnail {
                //     body = body.child(
                //         div()
                //             .w(px(44.0))
                //             .h(px(44.0))
                //             .rounded(px(8.0))
                //             .bg(rgb(0x404040))
                //             .when_some(
                //                 item_icon_path.clone().or_else(|| g.icon_path.clone()),
                //                 |this, path| this.child(img(path).size_full().rounded(px(8.0))),
                //             ),
                //     );
                // }

                content = content.child(body);

                let card_inner: AnyElement = if show_header {

                     let mut outer_wing = wing()
                        .w_128()
                        .group("")
                        .overflow_hidden()
                        .relative()
                        .border_1()
                        .border_color(colors.accent_200.with_alpha(0.6))                        
                        .bg(if is_expanded {
                            colors.background_900
                        } else {
                            colors.background_1000
                        })
                        .rounded(px(12.0));

                    outer_wing.upper_wing_size(Size::new(px(180.0), px(28.0)));
                    outer_wing.include_upper_wing_in_bounds(true);
                    outer_wing.border_radius(px(12.0));
                    outer_wing.border_width(px(1.0));

                    let mut inner_wing = wing()
                        .w_128()
                        .group("")
                        .relative()
                        .overflow_hidden()
                        .border_1()
                        .bg(if is_expanded {
                            colors.accent_200.with_alpha(0.0)
                        } else {
                            colors.accent_200.with_alpha(0.1)
                        })
                        .border(px(2.0))
                        .rounded(px(12.0))
                        .shadow_md()
                        .pt(px(2.0))
                        .px_4()
                        .py_3p5();

                    inner_wing.upper_wing_size(Size::new(px(180.0), px(28.0)));
                    inner_wing.include_upper_wing_in_bounds(true);
                    inner_wing.border_radius(px(12.0));
                    inner_wing.border_width(px(1.0));

                    let inner = inner_wing.child(content).into_any();
                        
                    outer_wing.child(inner).into_any()
                } else {
                    div()
                        .w_128()
                        .relative()
                        .rounded(px(12.0))
                        .border_1()
                        .border_color(colors.accent_200.with_alpha(0.2))
                        .bg(colors.background_900)
                        .px_4()
                        .py_3p5()
                        .child(content)
                        .into_any()
                };

                let mut card = div()
                    .id(("nc-card", item_id))
                    .overflow_hidden()
                    // .z_index(2)
                    .child(card_inner);

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
                        item.icon_path.clone(),
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
                                                Timer::after(Duration::from_millis(300)).await;
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
                                                                        group.icon_path =
                                                                            first.icon_path.clone();
                                                                    }

                                                                    // If group is now empty, remove it
                                                                    if group.items.is_empty() {
                                                                        this.groups.retain(|g| {
                                                                            g.id != target_group_id
                                                                        });
                                                                        this.rows.retain(|r| {
                                                                            r.id != target_group_id
                                                                        });
                                                                        this.expanded_groups
                                                                            .remove(
                                                                                &target_group_id,
                                                                            );
                                                                    } else if group.items.len() <= 1
                                                                    {
                                                                        // If only one item left, collapse the group
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
                    g.icon_path.clone(),
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
                                        Timer::after(Duration::from_millis(450)).await;
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
                                                    this.rows.retain(|s| s.id != removing_id);
                                                    this.expanded_groups.remove(&removing_id);
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
            list = list.child(
                row.id(("nc-row-container", st.id)).with_animation(
                    ElementId::NamedInteger(
                        "nc-row".into(),
                        if clearing {
                            1_000_000 + st.anim_epoch + st.id
                        } else {
                            st.anim_epoch + st.id
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
                            // Use st.id to make the stagger offset more stable, or just a constant
                            // since idx changes during removal.
                            let stagger_idx = (idx % 10) as f32;
                            let start = stagger_idx * step;
                            let denom = (1.0 - start).max(0.0001);
                            let local = ((delta - start) / denom).clamp(0.0, 1.0);
                            this.opacity(local)
                        }
                    },
                ),
            );
        }

        let entity = cx.entity();
        let entity2 = cx.entity();
        let viewport_entity = entity.clone();
        let content_entity = entity.clone();

        // Main container
        div()
            .relative()
            // .w(notifications_center_size.width)
            // .h(notifications_center_size.height)
            .w(notifications_center_size.width - px(1.5))
            .h(notifications_center_size.height - navbar_size.height - px(1.5))
            .bg(colors.background_1000)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.drag_start_pos = this.position;
                    this.drag_offset = Some(event.position.y.to_f64() as f32 - this.position);
                    cx.notify();
                }),
            )
            .child(
                div()
                    .id("nc-center-container")
                    .rounded(px(14.0))
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .child(header)
                    .on_drop(cx.listener(NotificationCenter::on_drop))
                    .on_drag_move(cx.listener(NotificationCenter::on_drag_move))
                    .child(
                        div()
                            .id("nc-center-viewport")
                            .flex_1()
                            .relative()
                            .overflow_hidden()
                            .child(
                                div()
                                    .id("nc-center-list")
                                    .relative()
                                    .top(self.scroll_offset)
                                    .on_drag(
                                        DragInfo::new(),
                                        move |_: &DragInfo, position, _, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.drag_start_y = position.y;
                                                this.last_scroll_offset = this.scroll_offset;
                                                this.is_dragging = true;
                                                cx.stop_propagation();
                                                cx.notify();
                                            });

                                            cx.new(|_| DragInfo::new().position(position))
                                        },
                                    )
                                    .child(list),
                            ),
                    ),
            )
    }
}
