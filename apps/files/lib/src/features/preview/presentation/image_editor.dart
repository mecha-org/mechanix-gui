import 'dart:io';

import 'package:flutter/material.dart';
import 'package:image/image.dart' as img;
import 'package:flutter/services.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/pressable_icon.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/notification/notification_type.dart';

class ImageEditorPage extends StatefulWidget {
  final String imagePath;
  final VoidCallback onClose;
  final FileExplorerPageState? state;

  const ImageEditorPage({
    super.key,
    required this.imagePath,
    required this.onClose,
    this.state,
  });
  @override
  State<ImageEditorPage> createState() => _ImageEditorPageState();
}

enum CropHandle { tl, tr, bl, br, left, right, top, bottom, move }

class _ImageEditorPageState extends State<ImageEditorPage> {
  img.Image? _image;
  Uint8List? _imageBytes;

  final List<Uint8List> _undoStack = [];
  final List<Uint8List> _redoStack = [];

  Rect? cropRect;
  Offset? _lastPos;
  CropHandle? _activeHandle;
  Rect? _displayedImageRect;
  bool _isCropping = false;
  final GlobalKey _imageKey = GlobalKey();
  static const double _cropMargin = 0.0; // space for handles
  static const double _outerMargin = 24.0;

  @override
  void initState() {
    super.initState();
    _loadImage();
  }

  void _pushToUndo() {
    if (_imageBytes == null) return;
    _undoStack.add(Uint8List.fromList(_imageBytes!));
    _redoStack.clear();
  }

  void undo() {
    if (_undoStack.isEmpty) return;

    _redoStack.add(Uint8List.fromList(_imageBytes!));
    final previous = _undoStack.removeLast();

    setState(() {
      _imageBytes = previous;
      _image = img.decodeImage(previous);
    });
  }

  void redo() {
    if (_redoStack.isEmpty) return;

    _undoStack.add(Uint8List.fromList(_imageBytes!));
    final next = _redoStack.removeLast();

    setState(() {
      _imageBytes = next;
      _image = img.decodeImage(next);
    });
  }

  void _commitImage(img.Image newImage) {
    setState(() {
      _image = newImage;
      _imageBytes = Uint8List.fromList(img.encodePng(newImage));
    });
  }

  Future<void> _loadImage() async {
    final file = File(widget.imagePath);
    final bytes = await file.readAsBytes();

    final decoded = img.decodeImage(bytes);
    if (decoded == null) return;

    setState(() {
      _image = decoded;
      _imageBytes = Uint8List.fromList(img.encodePng(decoded));
    });
  }

  Offset _normalizedTouch(Offset localPosition) {
    if (!_isCropping) return localPosition;
    return localPosition - const Offset(_outerMargin, _outerMargin);
  }

  void _exitCropMode() {
    setState(() {
      _isCropping = false;
      cropRect = null;
      _displayedImageRect = null;
      _activeHandle = null;
      _lastPos = null;
    });
  }

  void rotateRight() {
    if (_image == null) return;
    _commitCropIfAny();
    _pushToUndo();
    _commitImage(img.copyRotate(_image!, angle: 90));
  }

  void mirrorHorizontal() {
    if (_image == null) return;
    _commitCropIfAny();
    _pushToUndo();
    _commitImage(img.flipVertical(_image!));
  }

  void mirrorVertical() {
    if (_image == null) return;
    _commitCropIfAny();
    _pushToUndo();
    _commitImage(img.flipHorizontal(_image!));
  }

  void onCropPressed() {
    if (_image == null) return;

    if (!_isCropping) {
      final rect = _resolveDisplayedImageRect();
      if (rect == null || rect.size.isEmpty) return;

      setState(() {
        _displayedImageRect = rect;
        cropRect = Rect.fromLTRB(
          _cropMargin,
          _cropMargin,
          rect.width - _cropMargin,
          rect.height - _cropMargin,
        );
        _isCropping = true;
      });
      return;
    }

    // Apply crop
    _commitCropIfAny();
  }

  void _commitCropIfAny() {
    if (!_isCropping ||
        cropRect == null ||
        _displayedImageRect == null ||
        _image == null) return;

    final scaleX = _image!.width / _displayedImageRect!.width;
    final scaleY = _image!.height / _displayedImageRect!.height;

    final cropped = img.copyCrop(
      _image!,
      x: (cropRect!.left * scaleX).round(),
      y: (cropRect!.top * scaleY).round(),
      width: (cropRect!.width * scaleX).round(),
      height: (cropRect!.height * scaleY).round(),
    );

    _pushToUndo(); // history
    _commitImage(cropped); // new base image

    _exitCropMode(); // UI reset only
  }

  Future<void> saveImage() async {
    if (_image == null) return;

    // Ensure any active crop is committed
    _commitCropIfAny();

    final state = widget.state;

    final dir = p.dirname(widget.imagePath);
    final originalName = p.basenameWithoutExtension(widget.imagePath);

    final now = DateTime.now();
    final date =
        '${now.year}-${now.month.toString().padLeft(2, '0')}-${now.day.toString().padLeft(2, '0')}';
    final time = '${now.hour.toString().padLeft(2, '0')}:'
        '${now.minute.toString().padLeft(2, '0')}:'
        '${now.second.toString().padLeft(2, '0')}';

    final fileName = '$originalName – Edited $date $time.png';
    final file = File(p.join(dir, fileName));

    await file.writeAsBytes(img.encodePng(_image!));

    if (!mounted) return;

    MechanixNotification.show(
      context: context,
      notificationType: NotificationType.success,
      message: "Saved as '$fileName'",
    );

    state?.reload();
    widget.onClose();
  }

  CropHandle _detectHandle(Offset p) {
    if (cropRect == null) return CropHandle.move;
    final rect = cropRect!;
    const h = 30.0;

    // Corners
    if ((p - rect.topLeft).distance < h) return CropHandle.tl;
    if ((p - rect.topRight).distance < h) return CropHandle.tr;
    if ((p - rect.bottomLeft).distance < h) return CropHandle.bl;
    if ((p - rect.bottomRight).distance < h) return CropHandle.br;

    // Edges
    if ((p.dx - rect.left).abs() < h &&
        p.dy > rect.top + h &&
        p.dy < rect.bottom - h) {
      return CropHandle.left;
    }

    if ((p.dx - rect.right).abs() < h &&
        p.dy > rect.top + h &&
        p.dy < rect.bottom - h) {
      return CropHandle.right;
    }

    if ((p.dy - rect.top).abs() < h &&
        p.dx > rect.left + h &&
        p.dx < rect.right - h) {
      return CropHandle.top;
    }

    if ((p.dy - rect.bottom).abs() < h &&
        p.dx > rect.left + h &&
        p.dx < rect.right - h) {
      return CropHandle.bottom;
    }

    // Move (inside area)
    if (rect.contains(p)) return CropHandle.move;

    return CropHandle.move;
  }

  void _onPanStart(DragStartDetails d) {
    final p = _normalizedTouch(d.localPosition);
    _lastPos = p;
    _activeHandle = _detectHandle(p);
  }

  void _onPanUpdate(DragUpdateDetails d) {
    if (_displayedImageRect == null || cropRect == null) return;

    final dx = d.delta.dx;
    final dy = d.delta.dy;

    Rect r = cropRect!;
    switch (_activeHandle) {
      case CropHandle.tl:
        r = Rect.fromLTRB(r.left + dx, r.top + dy, r.right, r.bottom);
        break;

      case CropHandle.tr:
        r = Rect.fromLTRB(r.left, r.top + dy, r.right + dx, r.bottom);
        break;

      case CropHandle.bl:
        r = Rect.fromLTRB(r.left + dx, r.top, r.right, r.bottom + dy);
        break;

      case CropHandle.br:
        r = Rect.fromLTRB(r.left, r.top, r.right + dx, r.bottom + dy);
        break;

      case CropHandle.left:
        r = Rect.fromLTRB(r.left + dx, r.top, r.right, r.bottom);
        break;

      case CropHandle.right:
        r = Rect.fromLTRB(r.left, r.top, r.right + dx, r.bottom);
        break;

      case CropHandle.top:
        r = Rect.fromLTRB(r.left, r.top + dy, r.right, r.bottom);
        break;

      case CropHandle.bottom:
        r = Rect.fromLTRB(r.left, r.top, r.right, r.bottom + dy);
        break;

      case CropHandle.move:
        r = r.translate(dx, dy);
        break;

      default:
        return;
    }

    setState(() {
      cropRect = _clampRect(r);
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_imageBytes == null) {
      return Scaffold(
        body: Center(
          child: CircularProgressIndicator(
              color: context.colorScheme.surfaceContainerLowest),
        ),
      );
    }

    return Scaffold(
      body: LayoutBuilder(
        builder: (context, constraints) {
          return Center(
            child: FittedBox(
              fit: BoxFit.contain,
              child: Builder(
                builder: (context) {
                  final imageWidget = Image.memory(
                    _imageBytes!,
                    key: _imageKey,
                  );

                  WidgetsBinding.instance.addPostFrameCallback((_) {
                    if (!mounted || !_isCropping) return;

                    final box = context.findRenderObject() as RenderBox?;
                    if (box == null || box.size.isEmpty) return;

                    if (_displayedImageRect == null) {
                      setState(() {
                        _displayedImageRect = Offset.zero & box.size;
                        cropRect ??= Rect.fromLTRB(
                          _cropMargin,
                          _cropMargin,
                          box.size.width - _cropMargin,
                          box.size.height - _cropMargin,
                        );
                      });
                    }
                  });

                  return GestureDetector(
                    onPanStart: _isCropping ? _onPanStart : null,
                    onPanUpdate: _isCropping ? _onPanUpdate : null,
                    child: Padding(
                      padding: EdgeInsets.all(_isCropping ? _outerMargin : 0),
                      child: Stack(
                        children: [
                          imageWidget,
                          if (_isCropping &&
                              cropRect != null &&
                              _displayedImageRect != null)
                            CustomPaint(
                              painter: AdvancedCropPainter(cropRect!),
                              size: _displayedImageRect!.size,
                            ),
                        ],
                      ),
                    ),
                  );
                },
              ),
            ),
          );
        },
      ),
      bottomNavigationBar: _buildEditorBar(context),
    );
  }

  Widget _buildEditorBar(BuildContext context) {
    const double iconGap = 38;
    const double sidePadding = 16;

    return Container(
      height: 44,
      decoration: BoxDecoration(
        color: context.colorScheme.tertiary,
        borderRadius: const BorderRadius.only(
            topLeft: Radius.circular(8), topRight: Radius.circular(8)),
      ),
      child: Stack(
        children: [
          Align(
            alignment: Alignment.centerLeft,
            child: Padding(
              padding: const EdgeInsets.only(left: sidePadding),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  PressableIcon(
                    iconPath: Images.undo,
                    isDisabled: _undoStack.isEmpty,
                    onTap: _undoStack.isEmpty ? null : undo,
                  ),
                  const SizedBox(width: iconGap),
                  PressableIcon(
                    iconPath: Images.redo,
                    isDisabled: _redoStack.isEmpty,
                    onTap: _redoStack.isEmpty ? null : redo,
                  ),
                ],
              ),
            ),
          ),
          Align(
            alignment: Alignment.center,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 24),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  PressableIcon(
                    iconPath: Images.rotateRight,
                    onTap: rotateRight,
                  ),
                  const SizedBox(width: iconGap),
                  PressableIcon(
                    iconPath: Images.mirrorVertical,
                    onTap: mirrorVertical,
                  ),
                  const SizedBox(width: iconGap),
                  PressableIcon(
                    iconPath: Images.mirrorHorizontal,
                    onTap: mirrorHorizontal,
                  ),
                  const SizedBox(width: iconGap),
                  PressableIcon(
                    iconPath: Images.crop,
                    onTap: onCropPressed,
                  ),
                ],
              ),
            ),
          ),
          Align(
            alignment: Alignment.centerRight,
            child: Padding(
              padding: const EdgeInsets.only(right: sidePadding),
              child: PressableIcon(
                iconPath: Images.check,
                onTap: saveImage,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Rect? _resolveDisplayedImageRect() {
    final ctx = _imageKey.currentContext;
    if (ctx == null) return null;

    final box = ctx.findRenderObject() as RenderBox?;
    if (box == null || box.size.isEmpty) return null;

    return Offset.zero & box.size;
  }

  Rect _clampRect(Rect r) {
    const minSize = 60.0;
    final bounds = _displayedImageRect!;

    final safeBounds = Rect.fromLTRB(
      _cropMargin,
      _cropMargin,
      bounds.width - _cropMargin,
      bounds.height - _cropMargin,
    );

    return Rect.fromLTRB(
      r.left.clamp(safeBounds.left, safeBounds.right - minSize),
      r.top.clamp(safeBounds.top, safeBounds.bottom - minSize),
      r.right.clamp(r.left + minSize, safeBounds.right),
      r.bottom.clamp(r.top + minSize, safeBounds.bottom),
    );
  }
}

// Crop painter
class AdvancedCropPainter extends CustomPainter {
  final Rect rect;
  AdvancedCropPainter(this.rect);

  @override
  void paint(Canvas canvas, Size size) {
    final overlayPaint = Paint()..color = Colors.black.withOpacity(0.65);

    final overlayPath = Path()
      ..addRect(Rect.fromLTWH(0, 0, size.width, size.height))
      ..addRect(rect)
      ..fillType = PathFillType.evenOdd;

    canvas.drawPath(overlayPath, overlayPaint);

    // Border
    final borderPaint = Paint()
      ..color = Colors.white
      ..strokeWidth = 2
      ..style = PaintingStyle.stroke;

    canvas.drawRect(rect, borderPaint);

    // Handle settings
    const inset = -2.0; // keeps handles inside
    const handleLength = 18.0;

    final handlePaint = Paint()
      ..color = Colors.white
      ..strokeWidth = 4
      ..strokeCap = StrokeCap.square
      ..style = PaintingStyle.stroke;

    Rect r = rect.deflate(inset);

    // Corner Handles (L-shape)
    void corner(Offset c, bool left, bool top) {
      canvas.drawLine(
        c,
        c.translate(left ? handleLength : -handleLength, 0),
        handlePaint,
      );
      canvas.drawLine(
        c,
        c.translate(0, top ? handleLength : -handleLength),
        handlePaint,
      );
    }

    corner(r.topLeft, true, true);
    corner(r.topRight, false, true);
    corner(r.bottomLeft, true, false);
    corner(r.bottomRight, false, false);

    // Side Handles
    final sidePaint = Paint()..color = Colors.white;

    const sideLong = 22.0;
    const sideShort = 4.0;

    void horizontalHandle(Offset center) {
      canvas.drawRRect(
        RRect.fromRectAndRadius(
          Rect.fromCenter(center: center, width: sideLong, height: sideShort),
          const Radius.circular(2),
        ),
        sidePaint,
      );
    }

    void verticalHandle(Offset center) {
      canvas.drawRRect(
        RRect.fromRectAndRadius(
          Rect.fromCenter(center: center, width: sideShort, height: sideLong),
          const Radius.circular(2),
        ),
        sidePaint,
      );
    }

    verticalHandle(r.centerLeft);
    verticalHandle(r.centerRight);
    horizontalHandle(r.topCenter);
    horizontalHandle(r.bottomCenter);
  }

  @override
  bool shouldRepaint(covariant AdvancedCropPainter oldDelegate) {
    return oldDelegate.rect != rect;
  }
}
