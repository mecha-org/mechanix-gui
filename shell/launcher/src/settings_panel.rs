use bevy::{
    asset::meta::Settings, ecs::system::SystemId, prelude::*, render::settings,
    window::CompositeAlphaMode,
};
use bevy_styled_widgets::prelude::{StyledText, StyledTextPlugin, ThemeManager, ThemeMode};

use crate::{
    StyledWidgetsPlugin,
    components::AssetsLoadingState,
    utils::{FontAssets, Icon},
    widgets::{
        button::{ButtonSize, ButtonVariant, StyledButton},
        slider::StyledSlider,
    },
};
use bevy_asset_loader::prelude::*;

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

pub fn run_settings_panel() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Settings Panel".to_string(),
                    resolution: (540.0, 620.0).into(),
                    ..default()
                }),
                ..default()
            }),
            StyledWidgetsPlugin,
            StyledTextPlugin,
        ))
        .insert_resource(ThemeManager::default())
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<FontAssets>(),
        )
        .add_event::<SettingsPanelBackgroudEvent>()
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup)
        .add_systems(Update, update_root_background)
        .add_systems(Update, update_popup_background)
        .add_systems(Update, animate_background)
        .add_systems(Update, on_animation_background_completed)
        .add_systems(Update, animate_settings_item)
        .add_systems(Update, animate_settings_item_text)
        .run();
}

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
) {
    if *is_completed {
        // If the animation is completed, do nothing
        return;
    }

    for mut node in settings_query.iter_mut() {
        // Target size
        let target_width = 100.0;
        let target_height = 100.0;
        let start_width = 60.0;
        let start_height = 60.0;

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
        let new_width = start_width + (target_width - start_width) * eased_progress;
        let new_height = start_height + (target_height - start_height) * eased_progress;
        // Update the node dimensions
        node.width = Val::Percent(new_width);
        node.height = Val::Percent(new_height);
    }
}

fn toggle_mode(mut theme_manager: ResMut<ThemeManager>) {
    let current_mode = theme_manager.current_mode;
    let new_mode = match current_mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
    theme_manager.set_theme_mode(new_mode);
}

fn on_animation_background_completed(
    mut commands: Commands,
    mut event_reader: EventReader<SettingsPanelBackgroudEvent>,
    mut bg_query: Query<Entity, With<SettingsPanelBackgroud>>,
    font_assets: Option<Res<FontAssets>>,
) {
    let on_toogle_theme_mode = commands.register_system(toggle_mode);

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
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    children![
                        (
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            children![
                                (
                                    Node {
                                        width: Val::Px(34.0),
                                        height: Val::Px(34.0),
                                        margin: UiRect::right(Val::Px(24.0)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    children![
                                        StyledButton::builder()
                                            .icon(Icon::Settings)
                                            .font_size(16.)
                                            .font(font_assets.as_ref().unwrap().font_icons.clone())
                                            .build()
                                    ]
                                ),
                                (
                                    Node {
                                        width: Val::Px(34.0),
                                        height: Val::Px(34.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    children![
                                        StyledButton::builder()
                                            .icon(Icon::Edit)
                                            .font_size(16.)
                                            .font(font_assets.as_ref().unwrap().font_icons.clone())
                                            .build()
                                    ]
                                )
                            ]
                        ),
                        (
                            Node {
                                width: Val::Px(34.0),
                                height: Val::Px(34.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            children![
                                StyledButton::builder()
                                    .icon(Icon::Power)
                                    .font_size(16.)
                                    .font(font_assets.as_ref().unwrap().font_icons.clone())
                                    .build()
                            ]
                        ),
                    ],
                ));

                parent.spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        margin: UiRect::top(Val::Px(16.0)),
                        ..default()
                    },
                    children![(
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        children![
                            (StyledText::builder()
                                .content("11 April 12:30")
                                .font(font_assets.as_ref().unwrap().primary_500.clone())
                                .build(),),
                            // (
                            //     Text::new("11 April 12:30"),
                            //     TextFont {
                            //         font: font_assets.as_ref().unwrap().primary_500.clone(),
                            //         font_size: 16.0,
                            //         ..Default::default()
                            //     },
                            //     TextColor(Color::linear_rgba(0.24, 0.24, 0.24, 1.)),
                            // ),
                            (
                                Node { ..default() },
                                children![
                                    (
                                        StyledText::builder()
                                            .content(Icon::SignalBarsFull.to_string())
                                            .font(font_assets.as_ref().unwrap().primary_500.clone())
                                            .build(),
                                        Node {
                                            margin: UiRect::right(Val::Px(4.)),
                                            ..default()
                                        }
                                    ),
                                    (
                                        StyledText::builder()
                                            .content("Airtel")
                                            .font(font_assets.as_ref().unwrap().primary_500.clone())
                                            .build(),
                                        Node {
                                            margin: UiRect::right(Val::Px(21.)),
                                            ..default()
                                        }
                                    ),
                                    (
                                        StyledText::builder()
                                            .content("65%")
                                            .font(font_assets.as_ref().unwrap().primary_500.clone())
                                            .build(),
                                        Node {
                                            margin: UiRect::right(Val::Px(4.)),
                                            ..default()
                                        }
                                    ),
                                    (StyledText::builder()
                                        .content(Icon::Battery.to_string())
                                        .font(font_assets.as_ref().unwrap().primary_500.clone())
                                        .build(),),
                                ]
                            )
                        ]
                    ),],
                ));

                //spawn settings items

                parent
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(284.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        row_gap: Val::Px(14.0),
                        column_gap: Val::Px(14.0),
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::top(Val::Px(16.0)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        // Spawn the settings items
                        for control_name in [
                            "airplane_mode",
                            "auto_rotation",
                            "external_display",
                            "screen_record",
                            "wifi",
                            "bluetooth",
                            "camera",
                            "battery",
                            "terminal",
                            "voice_record",
                            "calc",
                            "theme",
                        ] {
                            spawn_menu_widget(
                                parent,
                                &font_assets.as_ref().unwrap(),
                                control_name,
                                on_toogle_theme_mode,
                            );
                        }
                    });

                //spawn brightness
                // parent.spawn((
                //     Node {
                //         width: Val::Percent(60.0),
                //         height: Val::Percent(60.0),
                //         max_height: Val::Px(60.),
                //         display: Display::Flex,
                //         flex_direction: FlexDirection::Column,
                //         justify_content: JustifyContent::Center,
                //         align_items: AlignItems::Start,
                //         margin: UiRect::top(Val::Px(14.0)),
                //         padding: UiRect::left(Val::Px(16.)),
                //         ..default()
                //     },
                //     BackgroundColor(Color::linear_rgb(0.85, 0.85, 0.85)),
                //     BorderRadius::all(Val::Px(12.0)),
                //     SettingsItem,
                //     Children::spawn(Spawn((
                //         Text::new(Icon::Brightness),
                //         TextFont {
                //             font: font_assets.as_ref().unwrap().font_icons.clone(),
                //             font_size: 16.0,
                //             ..Default::default()
                //         },
                //         TextColor(Color::linear_rgba(0.24, 0.24, 0.24, 1.)),
                //         SettingsItemText,
                //     ))),
                // ));

                // parent.spawn((
                //     Node {
                //         width: Val::Percent(60.0),
                //         height: Val::Percent(60.0),
                //         max_height: Val::Px(60.),
                //         display: Display::Flex,
                //         flex_direction: FlexDirection::Column,
                //         justify_content: JustifyContent::Center,
                //         align_items: AlignItems::Start,
                //         margin: UiRect::top(Val::Px(14.0)),
                //         padding: UiRect::left(Val::Px(16.)),
                //         ..default()
                //     },
                //     BackgroundColor(Color::linear_rgb(0.85, 0.85, 0.85)),
                //     BorderRadius::all(Val::Px(12.0)),
                //     SettingsItem,
                //     children![
                //         (
                //             Node {
                //                 display: Display::Flex,
                //                 position_type: PositionType::Absolute,
                //                 left: Val::Px(0.0),
                //                 height: Val::Percent(100.0),
                //                 width: Val::Percent(50.), // Indicator width based on value
                //                 ..default()
                //             },
                //             BackgroundColor(Color::linear_rgba(1., 1., 1., 0.8)),
                //             BorderRadius {
                //                 top_left: Val::Px(12.),
                //                 top_right: Val::Auto,
                //                 bottom_left: Val::Px(12.),
                //                 bottom_right: Val::Auto
                //             },
                //         ),
                //         (
                //             Text::new(Icon::VolumeOn),
                //             TextFont {
                //                 font: font_assets.as_ref().unwrap().font_icons.clone(),
                //                 font_size: 16.0,
                //                 ..Default::default()
                //             },
                //             TextColor(Color::linear_rgba(0.24, 0.24, 0.24, 1.)),
                //             SettingsItemText,
                //         ),
                //     ],
                // ));

                parent.spawn(
                    (StyledSlider::builder()
                        .icon(Icon::Brightness.to_string())
                        .max(100.)
                        .min(0.)
                        .value(80.)
                        .build()),
                );

                parent.spawn(
                    (StyledSlider::builder()
                        .icon(Icon::VolumeOn.to_string())
                        .max(100.)
                        .min(0.)
                        .value(80.)
                        .build()),
                );

                //spwan brightness and volume

                // parent.spawn(
                // (
                //     Node {
                //         width: Val::Percent(60.0),
                //         height: Val::Percent(60.0),
                //         display: Display::Flex,
                //         flex_direction: FlexDirection::Column,
                //         justify_content: JustifyContent::Center,
                //         align_items: AlignItems::Center,
                //         ..default()
                //     },
                //     BackgroundColor(Color::WHITE),
                //     BorderRadius::all(Val::Px(12.0)),
                //     SettingsItem
                // )
                // );

                //    parent.spawn(
                // (
                //     Node {
                //         width: Val::Percent(60.0),
                //         height: Val::Percent(60.0),
                //         display: Display::Flex,
                //         flex_direction: FlexDirection::Column,
                //         justify_content: JustifyContent::Center,
                //         align_items: AlignItems::Center,
                //         ..default()
                //     },
                //     BackgroundColor(Color::WHITE),
                //     BorderRadius::all(Val::Px(12.0)),
                //     SettingsItem
                // )
                // );
            });
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

fn setup(mut commands: Commands, theme_manager: Res<ThemeManager>) {
    commands.spawn(Camera2d);

    // Create a root node
    commands.spawn((
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
    ));

    // Add more UI elements here as needed
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

    let icon = match control_name {
        "airplane_mode" => Icon::AirplaneMode,
        "auto_rotation" => Icon::AutoRotation,
        "external_display" => Icon::Monitor,
        "screen_record" => Icon::ScreenRecord,
        "wifi" => Icon::WifiConnectedStrong,
        "bluetooth" => Icon::Bluetooth,
        "camera" => Icon::Camera,
        "battery" => Icon::Battery,
        "terminal" => Icon::Terminal,
        "voice_record" => Icon::Mic,
        "calc" => Icon::Calc,
        "theme" => Icon::Moon,
        "brightness" => Icon::Brightness,
        "volume" => Icon::VolumeOn,
        _ => Icon::Moon,
    };

    parent.spawn((StyledButton::builder()
        .icon(icon)
        .font(font_icons.clone())
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
