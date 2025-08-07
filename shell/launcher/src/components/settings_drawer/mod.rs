use bevy::{
    asset::meta::Settings, ecs::system::SystemId, prelude::*, render::settings, scene::ron::de,
    window::CompositeAlphaMode,
};
use bevy_core_widgets::CoreButton;
use bevy_styled_widgets::prelude::{StyledText, StyledTextPlugin, ThemeManager, ThemeMode};

use crate::{
    components::{AssetsLoadingState, Clock, get_current_datetime},
    styled_card::StyledCard,
    utils::{FontAssets, Icon},
    widgets::{
        LauncherStyledWidgetsPlugin,
        button::{ButtonSize, ButtonVariant, StyledButton},
        slider::StyledSlider,
    },
};
use bevy_asset_loader::prelude::*;

#[derive(Debug, States, Hash, Clone, Eq, PartialEq)]
pub enum Screens {
    Homescreen,
    SettingsDrawerAnimating,
    SettingsDrawer,
}

#[derive(Component)]
pub struct SettingsDrawerRoot;

#[derive(Component)]
pub struct SettingsDrawerPopup;

#[derive(Component)]
pub struct Wireless;

#[derive(Component)]
pub struct AutoRotation;

#[derive(Component)]
pub struct AirplaneMode;

#[derive(Component)]
pub struct Bluetooth;

#[derive(Component)]
pub struct Microphone;

#[derive(Component)]
pub struct ScreenRecording;

#[derive(Resource, Default, Debug, Clone)]
pub struct WirelessEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct BluetoothEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct RotationEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct AirplaneModeEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct PowerSavingEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct MicrophoneEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct ScreenRecordingEnabled(pub bool);

#[derive(Event)]
pub struct SettingsPanelBackgroudEvent;

#[derive(Component)]
pub struct SettingsPanelBackgroud;

#[derive(Component)]
pub struct SettingsItem;

#[derive(Component)]
pub struct SettingsItemText {
    pub font_size: f32,
}

#[derive(Component)]
pub struct StyledPopup;

pub struct SettingsDrawerPlugin;

impl Plugin for SettingsDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SettingsPanelBackgroudEvent>();

        app.insert_state(Screens::Homescreen);
        app.insert_resource(WirelessEnabled(true));
        app.init_resource::<BluetoothEnabled>();
        app.insert_resource(RotationEnabled(true));
        app.insert_resource(AirplaneModeEnabled(true));
        app.init_resource::<PowerSavingEnabled>();
        app.init_resource::<MicrophoneEnabled>();
        app.init_resource::<ScreenRecordingEnabled>();

        app.add_systems(
            Update,
            (
                update_popup_background,
                animate_background,
                on_animation_background_completed,
            ),
        );

        app.add_systems(
            Update,
            (
                animate_settings_item.run_if(in_state(Screens::SettingsDrawer)),
                animate_settings_item_text.run_if(in_state(Screens::SettingsDrawer)),
            ),
        );

        app.add_systems(
            OnEnter(Screens::SettingsDrawer),
            (
                update_wireless_state,
                update_airplane_mode_state,
                update_auto_rotation_state,
                update_bluetooth_state,
                update_microphone_state,
                update_screen_recording_state,
            ),
        );

        app.add_systems(
            Update,
            (
                update_wireless_state
                    .run_if(resource_changed::<WirelessEnabled>)
                    .run_if(resource_exists::<FontAssets>),
                update_airplane_mode_state
                    .run_if(resource_changed::<AirplaneModeEnabled>)
                    .run_if(resource_exists::<FontAssets>),
                update_auto_rotation_state
                    .run_if(resource_changed::<RotationEnabled>)
                    .run_if(resource_exists::<FontAssets>),
                update_bluetooth_state
                    .run_if(resource_changed::<BluetoothEnabled>)
                    .run_if(resource_exists::<FontAssets>),
                update_microphone_state
                    .run_if(resource_changed::<MicrophoneEnabled>)
                    .run_if(resource_exists::<FontAssets>),
                update_screen_recording_state
                    .run_if(resource_changed::<ScreenRecordingEnabled>)
                    .run_if(resource_exists::<FontAssets>),
            ),
        );
    }
}

fn update_bluetooth_state(
    mut query: Query<&mut StyledButton, With<Bluetooth>>,
    bluetooth_state: Res<BluetoothEnabled>,
    font_assets: Res<FontAssets>,
) {
    for mut styled_button in &mut query {
        info!("BluetoothEnabled is updated :{:?}", bluetooth_state);

        if bluetooth_state.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.bluetooth_on.clone());
            styled_button.layout = Some(font_assets.layout_bluetooth.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.bluetooth_off.clone());
            styled_button.layout = Some(font_assets.layout_bluetooth.clone());
        }
    }
}

fn update_auto_rotation_state(
    mut query: Query<&mut StyledButton, With<AutoRotation>>,
    auto_rotation: Res<RotationEnabled>,
    font_assets: Res<FontAssets>,
) {
    for mut styled_button in &mut query {
        info!("RotationEnabled is updated :{:?}", auto_rotation);

        if auto_rotation.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.rotation_on.clone());
            styled_button.layout = Some(font_assets.layout_rotation.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.rotation_off.clone());
            styled_button.layout = Some(font_assets.layout_rotation.clone());
        }
    }
}

fn update_wireless_state(
    mut query: Query<&mut StyledButton, With<Wireless>>,
    wifi_state: Res<WirelessEnabled>,
    font_assets: Res<FontAssets>,
) {
    for mut styled_button in &mut query {
        info!("WirelessEnabled is updated :{:?}", wifi_state);

        if wifi_state.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.wireless_none.clone());
            styled_button.layout = Some(font_assets.layout_wireless.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.wireless_off.clone());
            styled_button.layout = Some(font_assets.layout_wireless.clone());
        }
    }
}

fn update_microphone_state(
    mut query: Query<&mut StyledButton, With<Microphone>>,
    state: Res<MicrophoneEnabled>,
    font_assets: Res<FontAssets>,
) {
    info!("MicrophoneEnabled is updated :{:?}", state);
    for mut styled_button in &mut query {
        if state.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.microphone_on.clone());
            styled_button.layout = Some(font_assets.layout_microphone.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.microphone_off.clone());
            styled_button.layout = Some(font_assets.layout_microphone.clone());
        }
    }
}

fn update_screen_recording_state(
    mut query: Query<&mut StyledButton, With<ScreenRecording>>,
    state: Res<ScreenRecordingEnabled>,
    font_assets: Res<FontAssets>,
) {
    info!("ScreenRecordingEnabled is updated :{:?}", state);
    for mut styled_button in &mut query {
        if state.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.screen_recording_on.clone());
            styled_button.layout = Some(font_assets.layout_screen_recording.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.screen_recording_off.clone());
            styled_button.layout = Some(font_assets.layout_screen_recording.clone());
        }
    }
}

fn update_airplane_mode_state(
    mut query: Query<&mut StyledButton, With<AirplaneMode>>,
    airplane_enabled: Res<AirplaneModeEnabled>,
    font_assets: Res<FontAssets>,
) {
    info!("AirplaneModeEnabled is updated :{:?}", airplane_enabled);
    for mut styled_button in &mut query {
        if airplane_enabled.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.airplane_on.clone());
            styled_button.layout = Some(font_assets.layout_airplane.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.airplane_off.clone());
            styled_button.layout = Some(font_assets.layout_airplane.clone());
        }
    }
}

// pub fn run_settings_drawer() {
//     App::new()
//         .insert_resource(ThemeManager::default())
//         .init_state::<AssetsLoadingState>()
//         .add_loading_state(
//             LoadingState::new(AssetsLoadingState::Loading)
//                 .continue_to_state(AssetsLoadingState::Loaded)
//                 .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
//                 .load_collection::<FontAssets>(),
//         )
//         .add_systems(OnEnter(AssetsLoadingState::Loaded), settings_drawer)
//         // .add_systems(Update, update_root_background)
//         .run();
// }

fn update_popup_background(
    theme_manager: Res<ThemeManager>,
    mut query: Query<&mut BackgroundColor, With<StyledPopup>>,
) {
    for mut bg_color in query.iter_mut() {
        let theme_styles = theme_manager.styles.clone();
        let color = theme_styles.popup.background_color;
        bg_color.0 = color;
    }
}

fn animate_settings_item_text(
    mut text_font_query: Query<(&mut TextFont, &SettingsItemText), With<SettingsItemText>>,
    time: Res<Time>,
    mut is_completed: Local<bool>,
) {
    if *is_completed {
        // If the animation is completed, do nothing
        return;
    }

    for (mut text_font, styled_item_text) in text_font_query.iter_mut() {
        // Target size
        let start_font_size = 0.60 * styled_item_text.font_size;
        let target_font_size = styled_item_text.font_size;

        // Animation duration in seconds
        let animation_duration = 0.80;
        let elapsed = time.elapsed_secs();

        // Calculate progress (0.0 to 1.0)
        let progress = (elapsed / animation_duration).min(1.0);

        if progress >= 1.0 {
            // Animation completed, set the flag to true
            *is_completed = true;
        }

        // Ease-in cubic function: t^3
        let eased_progress = progress * progress * progress;

        // Interpolate between start and target using eased progress
        let new_font_size = start_font_size + (target_font_size - start_font_size) * eased_progress;

        println!("Progress 2: {}", new_font_size);

        // Update the font size
        text_font.font_size = new_font_size;
    }
}

fn animate_settings_item(
    mut settings_query: Query<&mut Node, With<SettingsItem>>,
    time: Res<Time>,
    mut is_completed: Local<bool>,
    mut anim_state: Local<Option<f32>>, // Option to store animation start time
) {
    if *is_completed {
        // If the animation is completed, do nothing
        return;
    }

    // On first run (in this state), store the start time
    let start_time = anim_state.get_or_insert(time.elapsed_secs());

    for mut node in settings_query.iter_mut() {
        // Target size
        let target_width = 100.0;
        let target_height = 100.0;
        let start_width = 60.0;
        let start_height = 60.0;

        // Animation duration in seconds
        let animation_duration = 0.10;
        let elapsed = time.elapsed_secs() - *start_time;

        // Calculate progress (0.0 to 1.0)
        let progress = (elapsed / animation_duration).min(1.0);

        if progress >= 1.0 {
            // Animation completed, set the flag to true
            *is_completed = true;
        }

        // Ease-in cubic function: t^3
        let eased_progress = progress * progress * progress;

        // Interpolate between start and target using eased progress
        let new_width = start_width + (target_width - start_width) * eased_progress;
        let new_height = start_height + (target_height - start_height) * eased_progress;
        // Update the node dimensions
        node.width = Val::Percent(new_width);
        node.height = Val::Percent(new_height);
    }
}

fn toggle_mode(mut theme_manager: ResMut<ThemeManager>) {
    // let current_mode = theme_manager.current_mode;
    // let new_mode = match current_mode {
    //     ThemeMode::Light => ThemeMode::Dark,
    //     ThemeMode::Dark => ThemeMode::Light,
    // };
    // theme_manager.set_theme_mode(new_mode);
}

fn toggle_auto_rotation(mut enabled: ResMut<RotationEnabled>) {
    enabled.0 = !enabled.0;
}

fn toggle_wireless(mut enabled: ResMut<WirelessEnabled>) {
    enabled.0 = !enabled.0;
}

fn long_press_wireless(
    mut commands: Commands,
    mut q_settings_drawer: Query<Entity, With<SettingsDrawerRoot>>,
) {
    println!("long press wireless");
    for entity in q_settings_drawer.iter_mut() {
        let popup_id = commands.spawn_empty().id();
        let popup = wireless_list_popup(&mut commands);
        commands.entity(popup_id).insert(popup);
        commands.entity(entity).add_child(popup_id);
    }
}

fn toggle_bluetooth(mut enabled: ResMut<BluetoothEnabled>) {
    enabled.0 = !enabled.0;
}

fn toggle_airplane_mode(mut enabled: ResMut<AirplaneModeEnabled>) {
    enabled.0 = !enabled.0;
}

fn toggle_screen_recording(mut enabled: ResMut<ScreenRecordingEnabled>) {
    enabled.0 = !enabled.0;
}

fn toggle_microphone(mut enabled: ResMut<MicrophoneEnabled>) {
    enabled.0 = !enabled.0;
}

fn on_animation_background_completed(
    mut commands: Commands,
    mut event_reader: EventReader<SettingsPanelBackgroudEvent>,
    mut bg_query: Query<Entity, With<SettingsPanelBackgroud>>,
    font_assets: Option<Res<FontAssets>>,
    mut screens_state: ResMut<NextState<Screens>>,
) {
    let on_toogle_theme_mode = commands.register_system(toggle_mode);
    let on_toggle_auto_rotation = commands.register_system(toggle_auto_rotation);
    let on_toggle_wireless = commands.register_system(toggle_wireless);
    let on_long_press_wireless = commands.register_system(long_press_wireless);
    let on_toggle_bluetooth = commands.register_system(toggle_bluetooth);
    let on_toggle_airplane_mode = commands.register_system(toggle_airplane_mode);
    let on_toggle_screen_recording = commands.register_system(toggle_screen_recording);
    let on_toggle_microphone = commands.register_system(toggle_microphone);

    if font_assets.is_none() {
        return;
    }

    for _event in event_reader.read() {
        // Remove the background component when the animation is completed
        println!("Animation completed");
        if let Ok(entity) = bg_query.single_mut() {
            println!("Entity: {:?}", entity);
            // Add new childs
            commands.entity(entity).with_children(|parent| {
                //spawn settings edit and power

                parent.spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    children![(
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        children![
                            (
                                StyledText::builder()
                                    .content(get_current_datetime())
                                    .font(font_assets.as_ref().unwrap().primary_500.clone())
                                    .font_size(16.)
                                    .build(),
                                Clock
                            ),
                            (
                                Node { ..default() },
                                children![
                                    (StyledText::builder()
                                        .content("65%")
                                        .font(font_assets.as_ref().unwrap().primary_500.clone())
                                        .font_size(16.)
                                        .build(),),
                                    (Node {
                                        margin: UiRect::right(Val::Px(1.)),
                                        ..default()
                                    }),
                                    (StyledText::builder()
                                        .content(Icon::BatteryFull)
                                        .font(font_assets.as_ref().unwrap().primary_500.clone())
                                        .font_size(16.)
                                        .build(),)
                                ]
                            ),
                            (
                                Node { ..default() },
                                children![
                                    (
                                        StyledText::builder()
                                            .content(Icon::Power.to_string())
                                            .font(font_assets.as_ref().unwrap().font_icons.clone())
                                            .font_size(24.)
                                            .build(),
                                        Node {
                                            margin: UiRect::right(Val::Px(16.)),
                                            ..default()
                                        }
                                    ),
                                    (
                                        StyledText::builder()
                                            .content(Icon::Settings.to_string())
                                            .font(font_assets.as_ref().unwrap().font_icons.clone())
                                            .font_size(24.)
                                            .build(),
                                        Node { ..default() }
                                    ),
                                ]
                            )
                        ]
                    ),],
                ));

                let FontAssets {
                    airplane_off,
                    airplane_on,
                    bluetooth_connected,
                    bluetooth_none,
                    bluetooth_off,
                    bluetooth_on,
                    bluetooth_warning,
                    brightness_low,
                    calculator,
                    calculator_pressed,
                    camera,
                    camera_pressed,
                    cell_signal_high,
                    microphone_off,
                    microphone_on,
                    power_saving_off,
                    power_saving_on,
                    rotation_off,
                    rotation_on,
                    screen_recording_off,
                    screen_recording_on,
                    sound_low,
                    terminal,
                    wireless_high,
                    wireless_low,
                    wireless_medium,
                    wireless_off,
                    wireless_warning,
                    extend_screen_none,
                    layout_airplane,
                    layout_bluetooth,
                    layout_brightness,
                    layout_calculator,
                    layout_camera,
                    layout_cell_signal,
                    layout_microphone,
                    layout_power_saving,
                    layout_rotation,
                    layout_screen_recording,
                    layout_sound,
                    layout_terminal,
                    layout_wireless,
                    layout_extend_screen,
                    ..
                } = &**font_assets.as_ref().unwrap();

                //spawn settings items
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(262.0),
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                            row_gap: Val::Px(28.0),
                            column_gap: Val::Px(32.0),
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect {
                                left: Val::Px(21.),
                                right: Val::Px(21.),
                                top: Val::Px(22.),
                                bottom: Val::Px(22.),
                            },
                            margin: UiRect::top(Val::Px(14.5)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(12.0)),
                        BackgroundColor(Color::oklcha(0.2221, 0., 0., 0.8)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            AutoRotation,
                            StyledButton::builder()
                                .icon(rotation_off.clone())
                                .layout(layout_rotation.clone())
                                .on_click(on_toggle_auto_rotation)
                                .build(),
                        ));

                        parent.spawn((
                            AirplaneMode,
                            StyledButton::builder()
                                .icon(airplane_off.clone())
                                .layout(layout_airplane.clone())
                                .active_background_color(Color::oklcha(0.7878, 0.1643, 75.13, 0.90))
                                .on_click(on_toggle_airplane_mode)
                                .build(),
                        ));

                        parent.spawn((StyledButton::builder()
                            .icon(extend_screen_none.clone())
                            .layout(layout_extend_screen.clone())
                            // .on_click(on_click)
                            .build(),));

                        parent.spawn((StyledButton::builder()
                            .icon(power_saving_off.clone())
                            .layout(layout_power_saving.clone())
                            // .on_click(on_click)
                            .build(),));

                        parent.spawn((StyledButton::builder()
                            .icon(calculator.clone())
                            .layout(layout_calculator.clone())
                            // .on_click(on_click)
                            .build(),));

                        parent.spawn((
                            Microphone,
                            StyledButton::builder()
                                .icon(microphone_off.clone())
                                .layout(layout_microphone.clone())
                                .on_click(on_toggle_microphone)
                                .build(),
                        ));

                        parent.spawn((
                            ScreenRecording,
                            StyledButton::builder()
                                .icon(screen_recording_off.clone())
                                .layout(layout_screen_recording.clone())
                                .on_click(on_toggle_screen_recording)
                                .build(),
                        ));

                        parent.spawn((StyledButton::builder()
                            .icon(camera.clone())
                            .layout(layout_camera.clone())
                            // .on_click(on_click)
                            .build(),));
                    });

                parent
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(74.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                        row_gap: Val::Px(0.0),
                        column_gap: Val::Px(32.0),
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::top(Val::Px(20.)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        // Spawn the settings items
                        parent.spawn(
                            (StyledSlider::builder()
                                .icon(brightness_low.clone())
                                .layout(layout_brightness.clone())
                                .max(100.)
                                .min(0.)
                                .value(30.)
                                .build()),
                        );
                        parent.spawn(
                            (StyledSlider::builder()
                                .icon(sound_low.clone())
                                .layout(layout_sound.clone())
                                .max(100.)
                                .min(0.)
                                .value(80.)
                                .build()),
                        );
                    });

                parent
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(89.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                        row_gap: Val::Px(0.0),
                        column_gap: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::top(Val::Px(20.)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        // Spawn the settings items
                        parent.spawn((
                            Wireless,
                            StyledButton::builder()
                                .icon(wireless_off.clone())
                                .layout(layout_wireless.clone())
                                .on_click(on_toggle_wireless)
                                .on_long_press(on_long_press_wireless)
                                .build(),
                        ));

                        parent.spawn((
                            Bluetooth,
                            StyledButton::builder()
                                .icon(bluetooth_off.clone())
                                .layout(layout_bluetooth.clone())
                                .on_click(on_toggle_bluetooth)
                                .build(),
                        ));

                        parent.spawn((StyledButton::builder()
                            .icon(terminal.clone())
                            .layout(layout_terminal.clone())
                            // .on_click(on_toggle_bluetooth)
                            .build(),));

                        parent.spawn((StyledButton::builder()
                            .icon(cell_signal_high.clone())
                            .layout(layout_cell_signal.clone())
                            // .on_click(on_toggle_bluetooth)
                            .build(),));
                    });
            });
            screens_state.set(Screens::SettingsDrawer);
        }
    }
}

fn animate_background(
    mut bg_query: Query<&mut Node, With<SettingsPanelBackgroud>>,
    time: Res<Time>,
    mut is_completed: Local<bool>,
    mut event_writer: EventWriter<SettingsPanelBackgroudEvent>,
) {
    if *is_completed {
        // If the animation is completed, do nothing
        return;
    }

    if let Ok(mut node) = bg_query.single_mut() {
        // Target size
        let target_width = 100.0;
        let target_height = 100.0;
        let start_width = 10.0;
        let start_height = 10.0;

        // Animation duration in seconds
        let animation_duration = 0.6;
        let elapsed = time.elapsed_secs();

        // Calculate progress (0.0 to 1.0)
        let progress = (elapsed / animation_duration).min(1.0);

        println!("Progress: {}", progress);

        if progress >= 1.0 {
            // Animation completed, set the flag to true
            *is_completed = true;
            event_writer.write(SettingsPanelBackgroudEvent);
        }

        // Ease-in cubic function: t^3
        let eased_progress = progress * progress * progress;

        // Interpolate between start and target using eased progress
        let new_width = start_width + (target_width - start_width) * eased_progress;
        let new_height = start_height + (target_height - start_height) * eased_progress;

        // Update the node dimensions
        node.width = Val::Percent(new_width);
        node.height = Val::Percent(new_height);
    }
}

fn setup2(mut commands: Commands, theme_manager: Res<ThemeManager>) {
    commands.spawn((
        Camera2d,
        Camera {
            ..Default::default()
        },
    ));
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
        children![
            (Text::new("Bevy"), TextColor(Color::linear_rgb(0., 0., 0.))),
            (
                Node {
                    width: Val::Px(100.),
                    height: Val::Px(100.),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(40.),
                    left: Val::Percent(40.),
                    ..Default::default()
                },
                BackgroundColor(Color::linear_rgba(0., 0., 0., 0.8))
            ),
        ],
    ));
}

pub fn settings_drawer(mut commands: &Commands, theme_manager: &ThemeManager) -> impl Bundle {
    (
        //make this transparent
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::End,
            ..default()
        },
        SettingsDrawerRoot,
        children![
            (
                Node {
                    width: Val::Percent(10.0),
                    height: Val::Percent(10.0),
                    padding: UiRect {
                        left: Val::Px(32.),
                        right: Val::Px(32.),
                        top: Val::Px(30.),
                        bottom: Val::Px(30.)
                    },
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme_manager.styles.panel.background_color),
                StyledCard,
                SettingsPanelBackgroud,
            ),
            // (
            //     Node {
            //         width: Val::Percent(50.0),
            //         height: Val::Percent(50.0),
            //         padding: UiRect {
            //             left: Val::Px(32.),
            //             right: Val::Px(32.),
            //             top: Val::Px(30.),
            //             bottom: Val::Px(30.)
            //         },
            //         display: Display::Flex,
            //         flex_direction: FlexDirection::Column,
            //         align_items: AlignItems::Center,
            //         align_self: AlignSelf::Center,
            //         justify_self: JustifySelf::Center,
            //         position_type: PositionType::Absolute,
            //         left: Val::Percent(50.),
            //         ..default()
            //     },
            //     BackgroundColor(theme_manager.styles.popup.background_color),
            //     StyledPopup,
            //     ZIndex(999)
            // )
        ],
    )
}

fn spawn_menu_widget(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    font_assets: &FontAssets,
    control_name: &str,
    on_click: SystemId,
) {
    let mut grid_column = GridPlacement::span(1);

    if control_name == "brightness" || control_name == "volume" {
        grid_column = GridPlacement::span(4);
    };

    let FontAssets { font_icons, .. } = font_assets;

    // let click_system_id = parent
    //     .commands()
    //     .register_system(control_click_system(control_name.to_string()));

    let (icon, layout, on_press_icon, on_press_layout) = match control_name {
        "airplane_mode" => (
            font_assets.airplane_off.clone(),
            font_assets.layout_airplane.clone(),
            font_assets.airplane_on.clone(),
            font_assets.layout_airplane.clone(),
        ),

        "auto_rotation" => (
            font_assets.rotation_off.clone(),
            font_assets.layout_rotation.clone(),
            font_assets.rotation_on.clone(),
            font_assets.layout_rotation.clone(),
        ),
        "screen_record" => (
            font_assets.screen_recording_off.clone(),
            font_assets.layout_screen_recording.clone(),
            font_assets.screen_recording_on.clone(),
            font_assets.layout_screen_recording.clone(),
        ),
        "wifi" => (
            font_assets.wireless_medium.clone(),
            font_assets.layout_wireless.clone(),
            font_assets.wireless_off.clone(),
            font_assets.layout_wireless.clone(),
        ),
        "bluetooth" => (
            font_assets.bluetooth_off.clone(),
            font_assets.layout_bluetooth.clone(),
            font_assets.bluetooth_on.clone(),
            font_assets.layout_bluetooth.clone(),
        ),
        "camera" => (
            font_assets.camera.clone(),
            font_assets.layout_camera.clone(),
            font_assets.camera_pressed.clone(),
            font_assets.layout_camera.clone(),
        ),
        "battery" => (
            font_assets.power_saving_off.clone(),
            font_assets.layout_power_saving.clone(),
            font_assets.power_saving_on.clone(),
            font_assets.layout_power_saving.clone(),
        ),
        "terminal" => (
            font_assets.terminal.clone(),
            font_assets.layout_terminal.clone(),
            font_assets.terminal.clone(),
            font_assets.layout_terminal.clone(),
        ),
        "voice_record" => (
            font_assets.microphone_off.clone(),
            font_assets.layout_microphone.clone(),
            font_assets.microphone_on.clone(),
            font_assets.layout_microphone.clone(),
        ),
        "brightness" => (
            font_assets.brightness_low.clone(),
            font_assets.layout_brightness.clone(),
            font_assets.brightness_low.clone(),
            font_assets.layout_brightness.clone(),
        ),
        "volume" => (
            font_assets.sound_low.clone(),
            font_assets.layout_sound.clone(),
            font_assets.sound_low.clone(),
            font_assets.layout_sound.clone(),
        ),
        // "screen_mirror" => (
        //     font_assets.screen_mirroring_off.clone(),
        //     font_assets.screen_mirroring_off_layout.clone(),
        // ),
        "calculator" => (
            font_assets.calculator.clone(),
            font_assets.layout_calculator.clone(),
            font_assets.calculator.clone(),
            font_assets.layout_calculator.clone(),
        ),
        "cellular" => (
            font_assets.cell_signal_high.clone(),
            font_assets.layout_cell_signal.clone(),
            font_assets.cell_signal_high.clone(),
            font_assets.layout_cell_signal.clone(),
        ),
        _ => (
            font_assets.wireless_medium.clone(),
            font_assets.layout_wireless.clone(),
            font_assets.wireless_off.clone(),
            font_assets.layout_wireless.clone(),
        ),
    };

    parent.spawn((StyledButton::builder()
        .icon(icon)
        .layout(layout)
        .font(font_icons.clone())
        .active(false)
        .on_press_icon(on_press_icon)
        .on_press_layout(on_press_layout)
        .active_background_color(Color::oklcha(0.7878, 0.1643, 75.13, 0.90))
        .on_click(on_click)
        .build(),));

    // parent.spawn((
    //     Node {
    //         width: Val::Percent(60.0),
    //         height: Val::Percent(60.0),
    //         display: Display::Flex,
    //         flex_direction: FlexDirection::Column,
    //         justify_content: JustifyContent::Center,
    //         align_items: AlignItems::Center,
    //         grid_column,
    //         grid_row: GridPlacement::span(1),
    //         ..default()
    //     },
    //     BackgroundColor(Color::linear_rgb(0.85, 0.85, 0.85)),
    //     BorderRadius::all(Val::Px(12.0)),
    //     SettingsItem,
    //     Children::spawn(Spawn((
    //         Text::new(icon),
    //         TextFont {
    //             font: font_assets.font_icons.clone(),
    //             font_size: 16.0,
    //             ..Default::default()
    //         },
    //         TextColor(Color::linear_rgba(0.24, 0.24, 0.24, 1.)),
    //         SettingsItemText,
    //     ))),
    // ));
}

fn popup_click(
    mut commands: Commands,
    mut q_settings_drawer: Query<Entity, With<SettingsDrawerPopup>>,
) {
    for entity in q_settings_drawer.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub fn wireless_list_popup(commands: &mut Commands) -> impl Bundle {
    let on_popup_click = commands.register_system(popup_click);

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            right: Val::Px(0.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        SettingsDrawerPopup,
        CoreButton {
            on_click: Some(on_popup_click),
            on_long_press: None,
        },
        BackgroundColor(Color::oklch(0.173, 0., 0.)),
        children![(
            Node {
                width: Val::Px(432.),
                height: Val::Px(333.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(24.)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius {
                        top_left: Val::Px(12.0),
                        top_right: Val::Px(12.0),
                        bottom_left: Val::Px(0.0),
                        bottom_right: Val::Px(0.0),
                    },
                    BackgroundColor(Color::oklch(0.4313, 0., 0.)),
                    children![
                        (StyledText::new("Wireless")),
                        (bevy_styled_widgets::prelude::StyledButton::builder()
                            .icon(Icon::Settings)
                            .text_color(Color::WHITE)
                            .size(bevy_styled_widgets::prelude::ButtonSize::Small)
                            .background_color(Color::oklcha(0.2221, 0., 0., 0.90))
                            .build(),)
                    ]
                ),
                (
                    Node {
                        width: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        padding: UiRect {
                            left: Val::Px(24.),
                            right: Val::Px(24.),
                            top: Val::Px(1.),
                            bottom: Val::Px(1.),
                        },
                        ..default()
                    },
                    BackgroundColor(Color::oklcha(0.2221, 0., 0., 0.90)),
                    BorderRadius {
                        top_left: Val::Px(0.0),
                        top_right: Val::Px(0.0),
                        bottom_left: Val::Px(12.0),
                        bottom_right: Val::Px(12.0),
                    },
                    children![
                        wireless("Mecha"),
                        divider(),
                        wireless("Actonate"),
                        divider(),
                        wireless("Mecha 5g"),
                        divider(),
                        wireless("Actonate 5g"),
                    ]
                )
            ]
        )],
    )
}

fn wireless(name: &str) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(26.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(20.)),
            ..default()
        },
        children![
            (StyledText::new("Icon")),
            (StyledText::new(name),),
            (StyledText::new("Connected"))
        ],
    )
}

fn divider() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            ..default()
        },
        BackgroundColor(Color::oklcha(0.4054, 0., 0., 0.80)),
    )
}
