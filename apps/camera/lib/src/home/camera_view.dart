import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/circular_frame_painter.dart';
import 'package:mechanix_camera/src/home/widgets/camera_frame_scope.dart';
import 'package:mechanix_camera/src/home/widgets/focus_with_exposure/focus_with_exposure.dart';
import 'package:mechanix_camera/src/home/widgets/grid_layout.dart';
import 'package:mechanix_camera/src/home/widgets/reframe_size.dart';
import 'package:widgets/extensions/build_context.dart';

class CameraView extends StatefulWidget {
  final CameraController? cameraController;
  final bool isCameraInitialized;

  const CameraView({
    super.key,
    this.cameraController,
    required this.isCameraInitialized,
  });

  @override
  State<CameraView> createState() => _CameraViewState();
}

class _CameraViewState extends State<CameraView>
    with SingleTickerProviderStateMixin {
  Offset? _focusPoint;
  bool _showFocusIndicator = false;

  double _exposureOffset = 0.0;
  double _minExposure = -2.0;
  double _maxExposure = 2.0;

  bool _isDraggingExposure = false;
  bool _isFocusing = false;
  bool _isLocked = false;

  late AnimationController _animController;
  late Animation<double> _scaleAnim;
  late Animation<double> _fadeAnim;

  @override
  void initState() {
    super.initState();

    _animController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
    );
    _scaleAnim = Tween<double>(
      begin: 1.5,
      end: 1.0,
    ).animate(CurvedAnimation(parent: _animController, curve: Curves.easeOut));
    _fadeAnim = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(
        parent: _animController,
        curve: const Interval(0.0, 0.4, curve: Curves.easeIn),
      ),
    );

    _initExposureRange();
  }

  Future<void> _initExposureRange() async {
    final c = widget.cameraController;
    if (c == null || !c.value.isInitialized) return;
    try {
      _minExposure = await c.getMinExposureOffset();
      _maxExposure = await c.getMaxExposureOffset();
    } catch (_) {}
  }

  @override
  void dispose() {
    _animController.dispose();
    super.dispose();
  }

  Future<void> _onTapFocus(
    TapUpDetails details,
    BoxConstraints constraints,
    Rect frameRect,
  ) async {
    if (_isLocked) return;

    final tapPos = details.localPosition;

    if (!frameRect.contains(tapPos)) return;

    final c = widget.cameraController;
    if (c == null || !c.value.isInitialized) return;

    final norm = Offset(
      ((tapPos.dx - frameRect.left) / frameRect.width).clamp(0.0, 1.0),
      ((tapPos.dy - frameRect.top) / frameRect.height).clamp(0.0, 1.0),
    );

    setState(() {
      _focusPoint = tapPos;
      _showFocusIndicator = true;
      _isFocusing = true;
      _isDraggingExposure = false;
      _exposureOffset = 0.0;
    });

    _animController.forward(from: 0);

    try {
      await c.setExposureMode(ExposureMode.auto);
      await c.setExposureOffset(0.0);
      await c.setExposurePoint(norm);
      await c.setFocusPoint(norm);
      await c.setFocusMode(FocusMode.auto);
      await Future.delayed(const Duration(milliseconds: 500));
      await c.setFocusMode(FocusMode.locked);
    } catch (_) {}

    if (mounted) setState(() => _isFocusing = false);
  }

  Future<void> _onExposureChanged(double newOffset) async {
    setState(() => _exposureOffset = newOffset);
    final c = widget.cameraController;
    if (c == null) return;
    try {
      if (_isLocked) {
        await c.setExposureOffset(newOffset);
      } else {
        await c.setExposureMode(ExposureMode.locked);
        await c.setExposureOffset(newOffset);
      }
    } catch (_) {}
  }

  void _onExposureDragStateChanged(bool dragging) {
    setState(() => _isDraggingExposure = dragging);
  }

  Future<void> _onLockChanged(bool locked) async {
    setState(() => _isLocked = locked);
    final c = widget.cameraController;
    if (c == null) return;
    try {
      if (locked) {
        await c.setFocusMode(FocusMode.locked);
        await c.setExposureMode(ExposureMode.locked);
        await c.setExposureOffset(_exposureOffset);
      } else {
        await c.setFocusMode(FocusMode.auto);
        await c.setExposureMode(ExposureMode.auto);
        await c.setExposureOffset(0.0);
        if (mounted) setState(() => _exposureOffset = 0.0);
      }
    } catch (_) {}
  }

  @override
  Widget build(BuildContext context) {
    if (!widget.isCameraInitialized || widget.cameraController == null) {
      return const Center(child: CircularProgressIndicator());
    }

    return CameraFrameProvider(
      child: LayoutBuilder(
        builder: (context, constraints) {
          final Rect frameRect = CameraFrameScope.of(context);

          return GestureDetector(
            onTapUp: (d) => _onTapFocus(d, constraints, frameRect),
            child: Stack(
              fit: StackFit.expand,
              children: [
                CameraPreview(widget.cameraController!),
                Positioned.fill(
                  child: Stack(
                    fit: StackFit.expand,
                    children: [
                      const ReframeSize(),
                      const GridLayout(),
                      BlocSelector<CameraBloc, CameraState, bool>(
                        selector: (state) => state.isSettingsOpen,
                        builder:
                            (context, isSettingsOpen) =>
                                !isSettingsOpen
                                    ? Center(
                                      child: CustomPaint(
                                        size: const Size(480, 480),
                                        painter: CircularFramePainter(
                                          colorScheme: context.colorScheme,
                                        ),
                                      ),
                                    )
                                    : const SizedBox.shrink(),
                      ),
                      if (_showFocusIndicator && _focusPoint != null)
                        AnimatedBuilder(
                          animation: _animController,
                          builder: (context, _) {
                            return FocusWithExposure(
                              position: _focusPoint!,
                              scale: _scaleAnim.value,
                              opacity: _fadeAnim.value,
                              exposureOffset: _exposureOffset,
                              minExposure: _minExposure,
                              maxExposure: _maxExposure,
                              isDragging: _isDraggingExposure,
                              isFocusing: _isFocusing,
                              onExposureChanged: _onExposureChanged,
                              onDragStateChanged: _onExposureDragStateChanged,
                              onLockChanged: _onLockChanged,
                            );
                          },
                        ),
                    ],
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
