import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:mechanix_settings/src/features/battery/widgets/battery_painter.dart';
import 'package:widgets/mechanix.dart';

import '../models/types.dart';

class BatteryIndicator extends StatefulWidget {
  final double height;
  final bool isCharging;
  final double tipHeight;
  final double tipWidth;

  const BatteryIndicator({
    super.key,
    required this.isCharging,
    this.height = 68,
    this.tipHeight = 32.0,
    this.tipWidth = 13.0,
  });

  @override
  State<BatteryIndicator> createState() => _BatteryIndicatorState();
}

class _BatteryIndicatorState extends State<BatteryIndicator> {
  ui.Image? _chargingIcon;
  bool _isLoadingImage = false;
  String? _lastPerformanceMode;

  @override
  void initState() {
    super.initState();
    _loadChargingIcon();
  }

  Future<void> _loadChargingIcon() async {
    if (_isLoadingImage) return;

    _isLoadingImage = true;
    try {
      final ByteData data =
          await rootBundle.load('assets/images/charging_icon.png');
      final bytes = data.buffer.asUint8List();
      final codec = await ui.instantiateImageCodec(bytes);
      final frame = await codec.getNextFrame();
      setState(() {
        _chargingIcon = frame.image;
      });
    } catch (e) {
      print('Error loading charging icon: $e');
    } finally {
      _isLoadingImage = false;
    }
  }

  List<Color> _getBatteryColor(BatteryState state) {
    if (state.batteryPercentage > 20) {
      return getModeDetails(state.performanceMode ?? '').colors;
    }
    return const [Color.fromRGBO(255, 0, 0, 1), Color.fromRGBO(240, 0, 0, 0.2)];
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BatteryBloc, BatteryState>(
      builder: (context, state) {
        final bool performanceModeChanged =
            _lastPerformanceMode != state.performanceMode;

        _lastPerformanceMode = state.performanceMode;

        final animationKey = ValueKey<String>(
            '${state.batteryPercentage}_${state.performanceMode}');
        return TweenAnimationBuilder<double>(
          key: animationKey,
          tween: Tween<double>(
            begin: performanceModeChanged ? 0 : state.batteryPercentage,
            end: state.batteryPercentage,
          ),
          duration: const Duration(milliseconds: 700),
          curve: Curves.easeInOut,
          builder: (context, animatedPercentage, child) {
            return LayoutBuilder(
              builder: (context, constraints) {
                final double availableWidth = constraints.maxWidth;

                return CustomPaint(
                  size: Size(availableWidth, widget.height),
                  painter: BatteryPainter(
                    batteryPercentage: animatedPercentage,
                    isCharging: widget.isCharging,
                    colors: _getBatteryColor(state),
                    borderColor: context.surfaceContainerHighest,
                    backgroundColor: context.colorScheme.secondary,
                    chargingIcon: _chargingIcon,
                    height: widget.height,
                    tipHeight: widget.tipHeight,
                    tipWidth: widget.tipWidth,
                    padding: 8,
                    borderRadius: 8,
                  ),
                );
              },
            );
          },
        );
      },
    );
  }
}
