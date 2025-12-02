import 'package:flutter/rendering.dart';
import 'package:mechanix_notes/src/features/home/widgets/staggered_grid_layout.dart';

class SliverStaggeredGridDelegateWithFixedCrossAxisCount
    extends SliverGridDelegate {
  final int crossAxisCount;
  final double mainAxisSpacing;
  final double crossAxisSpacing;
  final List<double> itemHeights;

  const SliverStaggeredGridDelegateWithFixedCrossAxisCount({
    required this.crossAxisCount,
    required this.mainAxisSpacing,
    required this.crossAxisSpacing,
    required this.itemHeights,
  });

  @override
  SliverGridLayout getLayout(SliverConstraints constraints) {
    final double usableCrossAxisExtent =
        constraints.crossAxisExtent - crossAxisSpacing * (crossAxisCount - 1);
    final double childCrossAxisExtent = usableCrossAxisExtent / crossAxisCount;

    return StaggeredGridLayout(
      crossAxisCount: crossAxisCount,
      mainAxisSpacing: mainAxisSpacing,
      crossAxisSpacing: crossAxisSpacing,
      childCrossAxisExtent: childCrossAxisExtent,
      itemHeights: itemHeights,
    );
  }

  @override
  bool shouldRelayout(
    SliverStaggeredGridDelegateWithFixedCrossAxisCount oldDelegate,
  ) {
    return oldDelegate.crossAxisCount != crossAxisCount ||
        oldDelegate.mainAxisSpacing != mainAxisSpacing ||
        oldDelegate.crossAxisSpacing != crossAxisSpacing ||
        oldDelegate.itemHeights.length != itemHeights.length;
  }
}
