import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

class GroupHeaderName extends SliverPersistentHeaderDelegate {
  final String title;

  GroupHeaderName(this.title);

  @override
  Widget build(
    BuildContext context,
    double shrinkOffset,
    bool overlapsContent,
  ) {
    return Container(
      margin: const EdgeInsets.only(left: 16, right: 16, bottom: 8, top: 12),
      alignment: Alignment.centerLeft,
      child: Text(
        title,
        style: const TextStyle(
          fontSize: 18,
          color: NotesColors.labelColor,
          fontWeight: FontWeight.w500,
          height: 1.35,
        ),
      ),
    );
  }

  @override
  double get maxExtent => 56;
  @override
  double get minExtent => 56;

  @override
  bool shouldRebuild(GroupHeaderName oldDelegate) =>
      oldDelegate.title != title;
}
