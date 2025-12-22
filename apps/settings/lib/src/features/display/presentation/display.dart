import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/display/bloc/display_bloc.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
import 'package:widgets/widgets/slider/mechanix_slider_theme.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

class DisplayPage extends StatefulWidget {
  const DisplayPage({super.key});

  @override
  State<DisplayPage> createState() => _DisplayPageState();
}

class _DisplayPageState extends State<DisplayPage> {
  String screenOffTime = "";
  String lockScreenTime = "";

  @override
  void initState() {
    super.initState();

    context.read<DisplayBloc>().add(GetDefaultSettingsEvent());
  }

  void changeBrightnessAuto(bool value) {
    context.read<DisplayBloc>().add(SetAutoBrightnessEvent(value));
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DisplayBloc, DisplayState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(
            title: 'Display',
          ),
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        'Brightness',
                        style: context.textTheme.labelLarge,
                      ),
                      Text(
                        '${(state.brightness * 100).round()} %',
                        style: context.textTheme.labelLarge,
                      )
                    ],
                  ).padVertical(8),
                  BlocListener<DisplayBloc, DisplayState>(
                    listener: (context, state) {
                      // TODO: implement listener
                    },
                    listenWhen: (previous, current) =>
                        previous.brightness != current.brightness,
                    child: MechanixSlider.dot(
                      isDotSlider: true,
                      initialValue: state.brightness,
                      onChangeEnd: (value) => context
                          .read<DisplayBloc>()
                          .add(SetBrightnessEvent(value)),
                      onChanged: (value) => context
                          .read<DisplayBloc>()
                          .add(SetBrightnessChangeEvent(value)),
                      theme: MechanixSliderThemeData(
                          activeColor: Color(0xFFD9D9D9)),
                      leftIcon: IconWidget(
                        iconColor: Colors.white,
                        iconPath: Images.sunIcon,
                        iconWidth: 20,
                        iconHeight: 28,
                        boxHeight: 30,
                        boxWidth: 30,
                      ),
                    ),
                  ),
                  MechanixSectionList(
                      title: 'Display Options',
                      physics: const BouncingScrollPhysics(),
                      sectionListItems: [
                        SectionListItems(
                            title: 'Auto Brightness',
                            defaultTrailingIcon: false,
                            trailing: MechanixSwitch(
                              activeText: 'OFF',
                              inactiveText: 'ON',
                              style: MechanixSwitchStyle(
                                activeTrackColor: Color(0xFF141414),
                                activeThumbColor: Color(0xFF2D8AFF),
                              ),
                              value: state.isAutoBrightness,
                              onChanged: changeBrightnessAuto,
                            )),
                        SectionListItems(
                          title: 'Screen Off Time',
                          onTap: () async {
                            // final result = await
                            Navigator.pushNamed(
                              context,
                              AppRoutes.displayScreenOffTime,
                              arguments: {'screenOffTime': state.screenTimeout},
                            );
                          },
                          trailing: Text(displayLabel(state.screenTimeout))
                              .padRight(8),
                        ),
                        SectionListItems(
                          title: 'Lock Screen Time',
                          onTap: () async {
                            Navigator.pushNamed(
                                context, AppRoutes.lockScreenTimeout);
                          },
                          trailing: Text(displayLabel(state.lockScreenTimeout))
                              .padRight(8),
                        )
                      ]).padTop(40),
                ],
              ).padTop(8),
            ),
          ),
        );
      },
    );
  }
}

class RectangularRoundedSliderTrackShape extends SliderTrackShape {
  final double borderRadius;

  const RectangularRoundedSliderTrackShape({this.borderRadius = 8.0});

  @override
  Rect getPreferredRect({
    required RenderBox parentBox,
    Offset offset = Offset.zero,
    required SliderThemeData sliderTheme,
    bool isEnabled = false,
    bool isDiscrete = false,
  }) {
    final trackHeight = sliderTheme.trackHeight ?? 2.0;
    final trackLeft = offset.dx;
    final trackTop = offset.dy + (parentBox.size.height - trackHeight) / 2;
    final trackWidth = parentBox.size.width;
    return Rect.fromLTWH(trackLeft, trackTop, trackWidth, trackHeight);
  }

  @override
  void paint(
    PaintingContext context,
    Offset offset, {
    required RenderBox parentBox,
    required SliderThemeData sliderTheme,
    required Animation<double> enableAnimation,
    required Offset thumbCenter,
    Offset? secondaryOffset,
    bool isEnabled = false,
    bool isDiscrete = false,
    required TextDirection textDirection,
  }) {
    if (sliderTheme.trackHeight == null || sliderTheme.trackHeight! <= 0) {
      return;
    }

    final Rect trackRect = getPreferredRect(
      parentBox: parentBox,
      offset: offset,
      sliderTheme: sliderTheme,
      isEnabled: isEnabled,
      isDiscrete: isDiscrete,
    );

    final activeTrackColor = sliderTheme.activeTrackColor ?? Colors.blue;
    final inactiveTrackColor = sliderTheme.inactiveTrackColor ?? Colors.grey;

    final Paint activePaint = Paint()
      ..color = activeTrackColor
      ..style = PaintingStyle.fill;
    final Paint inactivePaint = Paint()
      ..color = inactiveTrackColor
      ..style = PaintingStyle.fill;

    final canvas = context.canvas;

    // Create rounded rectangle for the entire track
    final RRect trackRRect = RRect.fromRectAndRadius(
      trackRect,
      Radius.circular(borderRadius),
    );

    // Draw inactive track (full width)
    canvas.drawRRect(trackRRect, inactivePaint);

    // Calculate active track width based on thumb position
    final double activeTrackWidth = thumbCenter.dx - trackRect.left;

    if (activeTrackWidth > 0) {
      final Rect activeTrackRect = Rect.fromLTRB(
        trackRect.left,
        trackRect.top,
        thumbCenter.dx.clamp(trackRect.left, trackRect.right),
        trackRect.bottom,
      );

      // Only draw if the active track has positive width
      if (activeTrackRect.width > 0) {
        final RRect activeTrackRRect = RRect.fromRectAndRadius(
          activeTrackRect,
          Radius.circular(borderRadius),
        );

        canvas.drawRRect(activeTrackRRect, activePaint);
      }
    }
  }
}

class SunSliderThumbShape extends SliderComponentShape {
  final double thumbRadius;
  final IconData icon;

  const SunSliderThumbShape(
      {this.thumbRadius = 20.0, this.icon = Icons.wb_sunny});

  @override
  Size getPreferredSize(bool isEnabled, bool isDiscrete) {
    return Size.fromRadius(thumbRadius);
  }

  @override
  void paint(
    PaintingContext context,
    Offset center, {
    required Animation<double> activationAnimation,
    required Animation<double> enableAnimation,
    required bool isDiscrete,
    required TextPainter labelPainter,
    required RenderBox parentBox,
    required SliderThemeData sliderTheme,
    required TextDirection textDirection,
    required double value,
    required double textScaleFactor,
    required Size sizeWithOverflow,
  }) {
    final canvas = context.canvas;
    final iconPainter = TextPainter(
      text: TextSpan(
        text: String.fromCharCode(icon.codePoint),
        style: TextStyle(
          fontSize: thumbRadius * 1.2,
          fontFamily: icon.fontFamily,
          package: icon.fontPackage,
          color: sliderTheme.thumbColor ?? Colors.grey,
        ),
      ),
      textDirection: textDirection,
    );

    iconPainter.layout();
    iconPainter.paint(
      canvas,
      center - Offset(iconPainter.width / 2, iconPainter.height / 2),
    );
  }
}
