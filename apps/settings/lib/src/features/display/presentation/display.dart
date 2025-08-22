import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_row_item.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';

class Display extends StatefulWidget {
  const Display({super.key});

  @override
  State<Display> createState() => DisplayState();
}

class DisplayState extends State<Display> {
  void _backNavigation() {
    Navigator.pop(context);
  }

  double displayBrightnessValue = 0;
  String screenOffTime = "";
  bool isAutoBrightness = false;

  @override
  void initState() {
    super.initState();
    // Simulate API load or default init
    displayBrightnessValue = 0.5;
    screenOffTime =
        displayScreenOffTimeToString[DisplayScreenOffTime.thirtySeconds]!;
  }

  @override
  Widget build(BuildContext context) {
    Color activeTrackColor = Colors.white;
    Color inactiveTrackColor = Colors.grey;

    return Scaffold(
      appBar: CustomAppBar(
        title: "Display",
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: _backNavigation,
      ),
      body: ContainerWidget(
        child: Column(
          children: [
            Container(
              height: 60,
              child: GestureDetector(
                onPanUpdate: (details) {
                  double localPosition = details.localPosition.dx;
                  double containerWidth = MediaQuery.of(context).size.width -
                      32; // Account for padding
                  double newValue =
                      (localPosition / containerWidth).clamp(0.0, 1.0);
                  setState(() {
                    displayBrightnessValue = newValue;
                  });
                },
                onTapUp: (details) {
                  double localPosition = details.localPosition.dx;
                  double containerWidth = MediaQuery.of(context).size.width -
                      32; // Account for padding
                  double newValue =
                      (localPosition / containerWidth).clamp(0.0, 1.0);
                  setState(() {
                    displayBrightnessValue = newValue;
                  });
                },
                child: Stack(
                  children: [
                    // Background track
                    Container(
                      height: 60,
                      decoration: BoxDecoration(
                        color: inactiveTrackColor,
                        borderRadius: BorderRadius.circular(8.0),
                      ),
                    ),
                    // Active track
                    Container(
                      height: 60,
                      width: (MediaQuery.of(context).size.width - 32) *
                          displayBrightnessValue,
                      decoration: BoxDecoration(
                        color: activeTrackColor,
                        borderRadius: BorderRadius.circular(8.0),
                      ),
                    ),
                    // Brightness icon positioned on the left
                    Positioned(
                      left: 20,
                      top: 0,
                      bottom: 0,
                      child: Center(
                        child: Icon(
                          Icons.wb_sunny,
                          color: displayBrightnessValue < 0.02
                              ? Colors.white
                              : Colors.black,
                          size: 24,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 40),
            CustomRowItem(
                title: "Auto Brightness",
                child: CustomToggle(
                    value: isAutoBrightness,
                    onChanged: (val) => setState(() {
                          isAutoBrightness = !isAutoBrightness;
                        }))),
            const SizedBox(height: 40),
            CustomRowItem(
              title: 'Screen Off Time',
              child: Text(
                screenOffTime,
                style: secondaryHeaderStyle,
              ),
              onTap: () async {
                final result = await Navigator.pushNamed(
                  context,
                  AppRoutes.displayScreenOffTime,
                  arguments: {'screenOffTime': screenOffTime},
                );

                if (result != null && result is String) {
                  setState(() {
                    screenOffTime = result;
                  });
                }
              },
            ),
          ],
        ),
      ),
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
