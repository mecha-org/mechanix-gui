import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/camera_helper.dart';

class CameraFrameScope extends InheritedWidget {
  const CameraFrameScope({
    super.key,
    required this.frameRect,
    required super.child,
  });

  final Rect frameRect;

  static Rect of(BuildContext context) {
    final scope =
        context.dependOnInheritedWidgetOfExactType<CameraFrameScope>();
    assert(scope != null, 'No CameraFrameScope found in widget tree');
    return scope!.frameRect;
  }

  @override
  bool updateShouldNotify(CameraFrameScope oldWidget) =>
      oldWidget.frameRect != frameRect;
}

/// layout calculation once per constraint/aspect-ratio change.
class CameraFrameProvider extends StatelessWidget {
  const CameraFrameProvider({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, CameraAspectRatio>(
      selector: (state) => state.aspectRatio,
      builder: (context, aspectRatio) {
        return LayoutBuilder(
          builder: (context, constraints) {
            final frameRect = CameraHelper.calculate(
              screenWidth: constraints.maxWidth,
              screenHeight: constraints.maxHeight,
              aspectRatio: aspectRatio,
            );

            return CameraFrameScope(frameRect: frameRect, child: child);
          },
        );
      },
    );
  }
}
