import 'dart:math' as math;

import 'package:flutter/rendering.dart';

class StaggeredGridLayout extends SliverGridLayout {
  final int crossAxisCount;
  final double mainAxisSpacing;
  final double crossAxisSpacing;
  final double childCrossAxisExtent;
  final List<double> itemHeights;

  late final List<double> _columnHeights;
  late final List<Offset> _itemOffsets;
  late final double _maxColumnHeight;

   StaggeredGridLayout({
    required this.crossAxisCount,
    required this.mainAxisSpacing,
    required this.crossAxisSpacing,
    required this.childCrossAxisExtent,
    required this.itemHeights,
  }) {
    _calculateLayout();
  }

  void _calculateLayout() {
    _columnHeights = List.filled(crossAxisCount, 0.0);
    _itemOffsets = [];

    for (int i = 0; i < itemHeights.length; i++) {
      // Find shortest column
      int shortestColumn = 0;
      double shortestHeight = _columnHeights[0];
      for (int j = 1; j < crossAxisCount; j++) {
        if (_columnHeights[j] < shortestHeight) {
          shortestHeight = _columnHeights[j];
          shortestColumn = j;
        }
      }

      // Place item in shortest column
      final double x =
          shortestColumn * (childCrossAxisExtent + crossAxisSpacing);
      final double y = _columnHeights[shortestColumn];
      _itemOffsets.add(Offset(x, y));

      // Update column height
      _columnHeights[shortestColumn] += itemHeights[i] + mainAxisSpacing;
    }

    _maxColumnHeight = _columnHeights.reduce(math.max);
  }

  @override
  int getMinChildIndexForScrollOffset(double scrollOffset) {
    if (_itemOffsets.isEmpty) return 0;

    // Binary search for first visible item
    int left = 0;
    int right = _itemOffsets.length - 1;

    while (left < right) {
      int mid = (left + right) ~/ 2;
      if (_itemOffsets[mid].dy + itemHeights[mid] < scrollOffset) {
        left = mid + 1;
      } else {
        right = mid;
      }
    }

    return math.max(0, left - crossAxisCount);
  }

  @override
  int getMaxChildIndexForScrollOffset(double scrollOffset) {
    if (_itemOffsets.isEmpty) return 0;

    // Find last visible item
    for (int i = _itemOffsets.length - 1; i >= 0; i--) {
      if (_itemOffsets[i].dy <= scrollOffset) {
        return math.min(i + crossAxisCount * 4, _itemOffsets.length - 1);
      }
    }
    return _itemOffsets.length - 1;
  }

  @override
  SliverGridGeometry getGeometryForChildIndex(int index) {
    if (index >= _itemOffsets.length || index < 0) {
      return const SliverGridGeometry(
        scrollOffset: 0,
        crossAxisOffset: 0,
        mainAxisExtent: 0,
        crossAxisExtent: 0,
      );
    }

    final offset = _itemOffsets[index];
    final height = itemHeights[index];

    return SliverGridGeometry(
      scrollOffset: offset.dy,
      crossAxisOffset: offset.dx,
      mainAxisExtent: height,
      crossAxisExtent: childCrossAxisExtent,
    );
  }

  @override
  double computeMaxScrollOffset(int childCount) {
    return _maxColumnHeight;
  }
}