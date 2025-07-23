use bevy::prelude::*;

use crate::{launcher::NavigationEvents, styled_card::StyledCard};

const BAR_BOTTOM_DISTANCE: f32 = 12.0;
const ANIMATION_DURATION: f32 = 0.8;
const LEFT_RIGHT_BAR_WIDTH: f32 = 74.;
const CENTER_BAR_WIDTH: f32 = 134.;
const LEFT_RIGHT_BAR_CONTAINER_WIDTH: f32 = 96.;
const CENTER_BAR_CONTAINER_WIDTH: f32 = 288.;

const SHORT_SWIPE_THRESHOLD: f32 = 30.0;
const LONG_SWIPE_THRESHOLD: f32 = 80.0;
const SWIPE_VELOCITY_THRESHOLD: f32 = 100.0;
const MIN_SWIPE_DISTANCE: f32 = 15.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Bar {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeUpType {
    Short,
    Long,
}

impl Bar {
    const fn default_width(self) -> f32 {
        match self {
            Self::Left | Self::Right => LEFT_RIGHT_BAR_WIDTH,
            Self::Center => CENTER_BAR_WIDTH,
        }
    }

    const fn container_width(self) -> f32 {
        match self {
            Self::Left | Self::Right => LEFT_RIGHT_BAR_CONTAINER_WIDTH,
            Self::Center => CENTER_BAR_CONTAINER_WIDTH,
        }
    }

    const fn justify_content(self) -> JustifyContent {
        match self {
            Self::Left => JustifyContent::Start,
            Self::Center => JustifyContent::Center,
            Self::Right => JustifyContent::End,
        }
    }

    const fn align_self(self) -> AlignSelf {
        match self {
            Self::Left => AlignSelf::Start,
            Self::Center => AlignSelf::Center,
            Self::Right => AlignSelf::Start,
        }
    }
}

#[derive(Component)]
struct BarAnimation {
    start_bottom: f32,
    start_width: f32,
    target_bottom: f32,
    target_width: f32,
    start_time: f32,
}

impl BarAnimation {
    fn new(current_bottom: f32, current_width: f32, target_width: f32, start_time: f32) -> Self {
        Self {
            start_bottom: current_bottom,
            start_width: current_width,
            target_bottom: BAR_BOTTOM_DISTANCE,
            target_width,
            start_time,
        }
    }

    fn interpolate(&self, current_time: f32) -> (f32, f32) {
        let elapsed = current_time - self.start_time;
        let progress = (elapsed / ANIMATION_DURATION).clamp(0.0, 1.0);

        let eased_progress = 1.0 - (1.0 - progress).powi(3);

        let bottom = self.start_bottom + (self.target_bottom - self.start_bottom) * eased_progress;
        let width = self.start_width + (self.target_width - self.start_width) * eased_progress;

        (bottom, width)
    }

    fn is_complete(&self, current_time: f32) -> bool {
        (current_time - self.start_time) >= ANIMATION_DURATION
    }
}

#[derive(Component)]
struct SwipeTracker {
    start_pos: Vec2,
    start_time: f32,
    last_pos: Vec2,
    last_time: f32,
    is_active: bool,
}

impl SwipeTracker {
    fn new(pos: Vec2, time: f32) -> Self {
        Self {
            start_pos: pos,
            start_time: time,
            last_pos: pos,
            last_time: time,
            is_active: true,
        }
    }

    fn update(&mut self, pos: Vec2, time: f32) {
        self.last_pos = pos;
        self.last_time = time;
    }

    fn get_swipe_data(&self) -> (Vec2, f32, f32) {
        let distance_vector = self.last_pos - self.start_pos;
        let total_distance = distance_vector.length();
        let total_time = self.last_time - self.start_time;
        let velocity = if total_time > 0.0 {
            total_distance / total_time
        } else {
            0.0
        };

        (distance_vector, total_distance, velocity)
    }

    fn determine_swipe_up(&self) -> Option<SwipeUpType> {
        let (distance_vector, total_distance, velocity) = self.get_swipe_data();

        if total_distance < MIN_SWIPE_DISTANCE || velocity < SWIPE_VELOCITY_THRESHOLD {
            return None;
        }

        if distance_vector.y >= 0.0 {
            return None;
        }

        if distance_vector.x.abs() > distance_vector.y.abs() {
            return None;
        }

        if total_distance >= LONG_SWIPE_THRESHOLD {
            Some(SwipeUpType::Long)
        } else if total_distance >= SHORT_SWIPE_THRESHOLD {
            Some(SwipeUpType::Short)
        } else {
            None
        }
    }
}

pub struct NavigationBarPlugin;

impl Plugin for NavigationBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_bar_click)
            .add_observer(on_bar_drag_start)
            .add_observer(on_bar_drag)
            .add_observer(on_bar_drag_end)
            .add_systems(Update, animate_bars);
    }
}

pub fn navigation_bar() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(30.0)),
            ..Default::default()
        },
        StyledCard,
        children![
            create_bar_section(Bar::Left),
            create_bar_section(Bar::Center),
            create_bar_section(Bar::Right),
        ],
    )
}

fn create_bar_section(bar: Bar) -> impl Bundle {
    (
        Node {
            width: Val::Px(bar.container_width()),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(1.0)),
            justify_content: bar.justify_content(),
            ..Default::default()
        },
        BorderColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.2)),
        children![create_bar(bar)],
    )
}

fn create_bar(bar: Bar) -> impl Bundle {
    (
        Node {
            width: Val::Px(bar.default_width()),
            height: Val::Px(8.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(BAR_BOTTOM_DISTANCE),
            align_self: bar.align_self(),
            ..Default::default()
        },
        BorderRadius::all(Val::Px(8.0)),
        BackgroundColor(Color::linear_rgb(0.74, 0.74, 0.74)),
        bar,
    )
}

fn on_bar_click(trigger: Trigger<Pointer<Click>>, q_bars: Query<&Bar>) {
    if let Ok(bar) = q_bars.get(trigger.target) {
        info!("Bar clicked: {:?}", bar);
    }
}

fn on_bar_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    q_bars: Query<&Bar>,
    time: Res<Time>,
) {
    if let Ok(bar) = q_bars.get(trigger.target) {
        trigger.propagate(false);
        info!("Bar drag started: {:?}", bar);

        if matches!(bar, Bar::Center) {
            let tracker = SwipeTracker::new(trigger.pointer_location.position, time.elapsed_secs());
            commands.entity(trigger.target).insert(tracker);
        }
    }
}

fn on_bar_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut q_bars: Query<(&mut Node, &Bar)>,
    mut q_swipe_trackers: Query<&mut SwipeTracker>,
    time: Res<Time>,
) {
    let Ok((mut node, bar)) = q_bars.get_mut(trigger.target) else {
        return;
    };

    trigger.propagate(false);

    if matches!(bar, Bar::Center) {
        if let Ok(mut tracker) = q_swipe_trackers.get_mut(trigger.target) {
            tracker.update(trigger.pointer_location.position, time.elapsed_secs());
        }
    }

    let distance = trigger.distance.y.abs();
    node.bottom = Val::Px(BAR_BOTTOM_DISTANCE + distance);

    let new_width = calculate_drag_width(*bar, distance);
    node.width = Val::Px(new_width);
}

fn calculate_drag_width(bar: Bar, distance: f32) -> f32 {
    match bar {
        Bar::Left | Bar::Right => (bar.default_width() + distance).min(bar.container_width()),
        Bar::Center => (bar.default_width() - distance).max(96.0),
    }
}

fn on_bar_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    q_bars: Query<&Node, With<Bar>>,
    q_bar_pos: Query<&Bar>,
    q_swipe_trackers: Query<&SwipeTracker>,
    time: Res<Time>,
) {
    let Ok(node) = q_bars.get(trigger.target) else {
        return;
    };

    trigger.propagate(false);

    let Ok(bar) = q_bar_pos.get(trigger.target) else {
        return;
    };

    if matches!(bar, Bar::Center) {
        if let Ok(tracker) = q_swipe_trackers.get(trigger.target) {
            if let Some(swipe_type) = tracker.determine_swipe_up() {
                handle_center_bar_swipe_up(swipe_type);
                match swipe_type {
                    SwipeUpType::Short => {
                        commands.trigger(NavigationEvents::OpenHomescreen);
                    }
                    SwipeUpType::Long => {
                        commands.trigger(NavigationEvents::AppSwitcher);
                    }
                }
            }
        }

        commands.entity(trigger.target).remove::<SwipeTracker>();
    } else if matches!(bar, Bar::Left) {
        commands.trigger(NavigationEvents::OpenSearch);
    } else if matches!(bar, Bar::Right) {
        commands.trigger(NavigationEvents::OpenSettingDrawer);
    }

    let current_bottom = match node.bottom {
        Val::Px(val) => val,
        _ => BAR_BOTTOM_DISTANCE,
    };

    let current_width = match node.width {
        Val::Px(val) => val,
        _ => bar.default_width(),
    };

    let animation = BarAnimation::new(
        current_bottom,
        current_width,
        bar.default_width(),
        time.elapsed_secs(),
    );

    commands.entity(trigger.target).insert(animation);
}

fn handle_center_bar_swipe_up(swipe_type: SwipeUpType) {
    match swipe_type {
        SwipeUpType::Short => {
            info!("Center bar: Short swipe up");
        }
        SwipeUpType::Long => {
            info!("Center bar: Long swipe up");
        }
    }
}

fn animate_bars(
    mut commands: Commands,
    mut q_animated_bars: Query<(Entity, &mut Node, &BarAnimation)>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (entity, mut node, animation) in q_animated_bars.iter_mut() {
        let (bottom, width) = animation.interpolate(current_time);

        node.bottom = Val::Px(bottom);
        node.width = Val::Px(width);

        if animation.is_complete(current_time) {
            commands.entity(entity).remove::<BarAnimation>();
        }
    }
}
