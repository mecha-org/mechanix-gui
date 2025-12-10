import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

class GroupHeaderName extends SliverPersistentHeaderDelegate {
  final String title;
  final bool isFirst;
  GroupHeaderName({required this.title, this.isFirst = false});

  @override
  Widget build(
    BuildContext context,
    double shrinkOffset,
    bool overlapsContent,
  ) {
    return Container(
      margin: EdgeInsets.only(
        left: 16,
        right: 16,
        bottom: 8,
        top: isFirst ? 12 : 36,
      ),
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
  double get maxExtent => isFirst ? 44 : 68;
  @override
  double get minExtent => isFirst ? 44 : 68;

  @override
  bool shouldRebuild(GroupHeaderName oldDelegate) => oldDelegate.title != title;
}
