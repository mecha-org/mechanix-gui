import 'dart:math' as math;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart' show TextSelectionPoint;
import 'package:flutter/services.dart' show SelectionChangedCause, ClipboardStatus;

class MyMaterialSelectionControls extends MaterialTextSelectionControls {
  @override
  Widget buildToolbar(
    BuildContext context,
    Rect globalEditableRegion,
    double textLineHeight,
    Offset selectionMidpoint,
    List<TextSelectionPoint> endpoints,
    TextSelectionDelegate delegate,
    ValueListenable<ClipboardStatus>? clipboardStatus,
    Offset? lastSecondaryTapDownPosition,
  ) {
    // Convert endpoints (which are relative to the RenderEditable) to global coords
    final endpointsGlobal = endpoints
        .map((p) => globalEditableRegion.topLeft + p.point)
        .toList(growable: false);

    final Offset mid = endpointsGlobal.length >= 2
        ? Offset((endpointsGlobal.first.dx + endpointsGlobal.last.dx) / 2,
            math.min(endpointsGlobal.first.dy, endpointsGlobal.last.dy))
        : (endpointsGlobal.isNotEmpty
            ? endpointsGlobal.first
            : globalEditableRegion.center);

    final anchorAbove = mid.translate(0, -textLineHeight);
    final anchorBelow = mid.translate(0, textLineHeight);

    return TextSelectionToolbar(
      anchorAbove: anchorAbove,
      anchorBelow: anchorBelow,
      children: [
        TextSelectionToolbarTextButton(
          padding: const EdgeInsets.symmetric(horizontal: 8),
          onPressed: canCut(delegate)
              ? () {
                  delegate.cutSelection(SelectionChangedCause.toolbar);
                }
              : null,
          child: const Text('Cut'),
        ),
        TextSelectionToolbarTextButton(
          padding: const EdgeInsets.symmetric(horizontal: 8),
          onPressed: canCopy(delegate)
              ? () {
                  delegate.copySelection(SelectionChangedCause.toolbar);
                }
              : null,
          child: const Text('Copy'),
        ),
        TextSelectionToolbarTextButton(
          padding: const EdgeInsets.symmetric(horizontal: 8),
          onPressed: canPaste(delegate)
              ? () async {
                  await delegate.pasteText(SelectionChangedCause.toolbar);
                }
              : null,
          child: const Text('Paste'),
        ),
      ],
    );
  }
}
