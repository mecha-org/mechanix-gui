import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/extensions/color.dart';
import 'package:widgets/widgets/icon_widget.dart';

class TimerLabel extends StatelessWidget {
  const TimerLabel({super.key});

  /// Top Timer Label
  @override
  Widget build(BuildContext context) {
    return BlocSelector<
      CameraBloc,
      CameraState,
      Tuple2<CameraTimer, CaptureMode>
    >(
      selector: (state) => Tuple2(state.timer, state.captureMode),
      builder: (context, state) {
        final timer = state.item1;
        final captureMode = state.item2;

        return timer.seconds > 0 && captureMode == CaptureMode.photo
            ? Positioned(
              top: 15,
              left: 0,
              right: 0,
              child: Center(
                child: Container(
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(8),
                    color: context.surfaceContainerHigh.withValues(alpha: 0.8),
                  ),
                  padding: const EdgeInsets.symmetric(
                    horizontal: 8,
                    vertical: 4,
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    spacing: 10,
                    children: [
                      IconWidget(
                        iconPath: CameraIcons.timerIcon,
                        boxWidth: 20,
                        boxHeight: 20,
                      ),
                      Text(
                        timer.label,
                        style: TextStyle(
                          fontSize: 18,
                          color: context.onSecondaryFixedVariant,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            )
            : const SizedBox.shrink();
      },
    );
  }
}
