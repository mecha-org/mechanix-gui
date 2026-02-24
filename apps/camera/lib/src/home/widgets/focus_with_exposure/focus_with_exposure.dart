import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mechanix_camera/src/home/widgets/camera_frame_scope.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:widgets/mechanix.dart';

class FocusWithExposure extends StatefulWidget {
  final Offset position;
  final double scale;
  final double opacity;
  final double exposureOffset;
  final double minExposure;
  final double maxExposure;
  final bool isDragging;
  final bool isFocusing;
  final Future<void> Function(double) onExposureChanged;
  final void Function(bool) onDragStateChanged;
  final Future<void> Function(bool locked) onLockChanged;

  const FocusWithExposure({
    super.key,
    required this.position,
    required this.scale,
    required this.opacity,
    required this.exposureOffset,
    required this.minExposure,
    required this.maxExposure,
    required this.isDragging,
    required this.isFocusing,
    required this.onExposureChanged,
    required this.onDragStateChanged,
    required this.onLockChanged,
  });

  @override
  State<FocusWithExposure> createState() => FocusWithExposureState();
}

class FocusWithExposureState extends State<FocusWithExposure>
    with SingleTickerProviderStateMixin {
  final GlobalKey _trackKey = GlobalKey();
  double _trackHeight = 80.0;

  double _dragStartY = 0.0;
  double _exposureAtDragStart = 0.0;

  bool _isLocked = false;

  late AnimationController _lockAnim;
  late Animation<double> _lockScale;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _measureTrack());

    _lockAnim = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 200),
    );
    _lockScale = Tween<double>(
      begin: 1.0,
      end: 1.35,
    ).animate(CurvedAnimation(parent: _lockAnim, curve: Curves.easeOut));
  }

  @override
  void dispose() {
    _lockAnim.dispose();
    super.dispose();
  }

  void _measureTrack() {
    final ctx = _trackKey.currentContext;
    if (ctx == null) return;
    final box = ctx.findRenderObject() as RenderBox?;
    if (box != null) setState(() => _trackHeight = box.size.height);
  }

  void _onDragStart(DragStartDetails details) {
    _dragStartY = details.localPosition.dy;
    _exposureAtDragStart = widget.exposureOffset;
    widget.onDragStateChanged(true);
  }

  void _onDragUpdate(DragUpdateDetails details) {
    final delta = _dragStartY - details.localPosition.dy;
    final range = widget.maxExposure - widget.minExposure;
    final sensitivity = range / (_trackHeight * 0.9);
    final newOffset = (_exposureAtDragStart + delta * sensitivity).clamp(
      widget.minExposure,
      widget.maxExposure,
    );
    widget.onExposureChanged(newOffset);
  }

  void _onDragEnd(DragEndDetails _) => widget.onDragStateChanged(false);

  Future<void> _onLongPress() async {
    if (_isLocked) return;
    HapticFeedback.mediumImpact();
    setState(() => _isLocked = true);
    await _lockAnim.forward();
    await _lockAnim.reverse();
    await widget.onLockChanged(true);
  }

  Future<void> _onUnlock() async {
    HapticFeedback.lightImpact();
    setState(() => _isLocked = false);
    await widget.onLockChanged(false);
  }

  @override
  Widget build(BuildContext context) {
    const double outerD = 116.0;
    const double innerD = 40.0;
    const double barHeight = 160.0;
    const double barWidth = 24.0;
    const double gap = 10.0;
    const double iconSize = 16.0;
    const double totalW = outerD + gap + barWidth;
    const double totalH = barHeight;
    const double vPad = 28.0;
    const double iconGap = 2.0;

    final Rect frameRect = CameraFrameScope.of(context);
    double left = widget.position.dx - outerD / 2;
    double top = widget.position.dy - totalH / 2;
    left = left.clamp(frameRect.left, frameRect.right - totalW);
    top = top.clamp(frameRect.top, frameRect.bottom - totalH);

    final double range = widget.maxExposure - widget.minExposure;
    final double normalized =
        range == 0
            ? 0.5
            : ((widget.exposureOffset - widget.minExposure) / range).clamp(
              0.0,
              1.0,
            );

    final Color ringColor =
        _isLocked
            ? context.primaryContainer
            : context.onSurface.withValues(alpha: 0.9);

    final Color iconColor =
        (widget.isDragging || _isLocked)
            ? context.primaryContainer
            : context.onSurface;

    final Color lineColor =
        (widget.isDragging || _isLocked)
            ? context.primaryContainer
            : context.onSurface;

    return Positioned(
      left: left,
      top: top,
      child: Opacity(
        opacity: widget.opacity.clamp(0.0, 1.0),
        child: Transform.scale(
          scale: widget.scale,
          alignment: Alignment.centerLeft,
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            mainAxisSize: MainAxisSize.min,
            children: [
              SizedBox(
                width: outerD,
                height: totalH,
                child: Center(
                  child: GestureDetector(
                    onLongPress: _onLongPress,
                    child: SizedBox(
                      width: outerD,
                      height: outerD,
                      child: Stack(
                        alignment: Alignment.center,
                        children: [
                          _Ring(diameter: outerD, color: ringColor),
                          _Ring(diameter: innerD, color: ringColor),

                          if (widget.isFocusing && !_isLocked)
                            SizedBox(
                              width: outerD - 6,
                              height: outerD - 6,
                              child: CircularProgressIndicator(
                                strokeWidth: 1.0,
                                color: context.onSurface.withValues(
                                  alpha: 0.35,
                                ),
                              ),
                            ),

                          if (_isLocked)
                            GestureDetector(
                              onTap: _onUnlock,
                              child: ScaleTransition(
                                scale: _lockScale,
                                child: Container(
                                  width: innerD,
                                  height: innerD,
                                  decoration: BoxDecoration(
                                    shape: BoxShape.circle,
                                    color: context.primaryContainer.withValues(
                                      alpha: 0.15,
                                    ),
                                  ),
                                  child: Icon(
                                    Icons.lock,
                                    color: context.primaryContainer,
                                    size: 18,
                                  ),
                                ),
                              ),
                            ),

                          if (!_isLocked && !widget.isFocusing)
                            Container(
                              width: 4,
                              height: 4,
                              decoration: BoxDecoration(
                                shape: BoxShape.circle,
                                color: context.onSurface.withValues(alpha: 0.5),
                              ),
                            ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),

              const SizedBox(width: gap),

              GestureDetector(
                onVerticalDragStart: _onDragStart,
                onVerticalDragUpdate: _onDragUpdate,
                onVerticalDragEnd: _onDragEnd,
                behavior: HitTestBehavior.opaque,
                child: SizedBox(
                  width: barWidth,
                  height: totalH,
                  child: LayoutBuilder(
                    builder: (ctx, bc) {
                      final double trackTop = vPad;
                      final double trackBottom = bc.maxHeight - vPad;
                      final double trackH = trackBottom - trackTop;

                      final double travelH = trackH - iconSize - iconGap * 2;
                      final double iconTop = (trackTop +
                              iconGap +
                              (1.0 - normalized) * travelH)
                          .clamp(trackTop, trackBottom - iconSize);
                      final double iconBottom = iconTop + iconSize;

                      final double seg1Top = trackTop;
                      final double seg1Bottom = iconTop - iconGap;
                      final double seg1H = (seg1Bottom - seg1Top).clamp(
                        0,
                        trackH,
                      );

                      final double seg2Top = iconBottom + iconGap;
                      final double seg2Bottom = trackBottom;
                      final double seg2H = (seg2Bottom - seg2Top).clamp(
                        0,
                        trackH,
                      );

                      return Stack(
                        key: _trackKey,
                        fit: StackFit.expand,
                        clipBehavior: Clip.none,
                        children: [
                          if (seg1H > 0)
                            Positioned(
                              top: seg1Top,
                              left: 0,
                              right: 0,
                              height: seg1H,
                              child: Center(
                                child: Container(
                                  width: 1,
                                  height: seg1H,
                                  decoration: BoxDecoration(
                                    color: lineColor,
                                    borderRadius: BorderRadius.circular(1),
                                  ),
                                ),
                              ),
                            ),

                          if (seg2H > 0)
                            Positioned(
                              top: seg2Top,
                              left: 0,
                              right: 0,
                              height: seg2H,
                              child: Center(
                                child: Container(
                                  width: 1,
                                  height: seg2H,
                                  decoration: BoxDecoration(
                                    color: lineColor,
                                    borderRadius: BorderRadius.circular(1),
                                  ),
                                ),
                              ),
                            ),

                          Positioned(
                            top: iconTop,
                            left: 0,
                            right: 0,
                            child: Center(
                              child: IconWidget(
                                iconPath: CameraIcons.brightnessIcon,
                                iconHeight: iconSize,
                                iconWidth: iconSize,
                                iconColor: iconColor,
                                boxHeight: iconSize,
                                boxWidth: iconSize,
                              ),
                            ),
                          ),
                        ],
                      );
                    },
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _Ring extends StatelessWidget {
  final double diameter;
  final Color color;
  const _Ring({required this.diameter, required this.color});

  @override
  Widget build(BuildContext context) => Container(
    width: diameter,
    height: diameter,
    decoration: BoxDecoration(
      shape: BoxShape.circle,
      border: Border.all(color: color, width: 1.5),
    ),
  );
}
