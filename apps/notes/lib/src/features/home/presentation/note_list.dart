import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/src/commons/common_helper.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc_provider.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:widgets/mechanix.dart';

class NoteList extends StatelessWidget {
  final bool isSelectionMode;
  final List<String> selectedNotes;
  final void Function(String id)? onSelect;
  final List<GroupedNotes> groupedNotes;
  final ScrollController? controller;

  const NoteList({
    super.key,
    required this.isSelectionMode,
    required this.selectedNotes,
    this.onSelect,
    required this.groupedNotes,
    this.controller,
  });

  @override
  Widget build(BuildContext context) {
    if (groupedNotes.isEmpty) {
      return SizedBox(
        height: 64,
        child: MechanixSelectableList(
          // itemPadding: const EdgeInsets.all(10),
          onTap: () => Navigator.pushNamed(context, AppRoutes.createEditNotes),
          leadingIcon: Image.asset(NotesIcon.addIcon, height: 18, width: 18),
          isSelected: true,
          title: "Add a new Note",
          // titleTextStyle: const TextStyle(
          //   color: NotesColors.secondaryTextColor,
          //   fontSize: 16,
          // ),
        ),
      );
    }

    return ScrollConfiguration(
      behavior: const ScrollBehavior().copyWith(
        overscroll: false,
        dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
      ),
      child: CustomScrollView(
        controller: controller,
        slivers:
            groupedNotes.map((group) {
              return SliverMainAxisGroup(
                slivers: [
                  // Section Header
                  SliverPersistentHeader(
                    pinned: true,
                    delegate: _GroupHeaderDelegate(group.label),
                  ),

                  // Lazy-built list of notes for this group
                  SliverPrototypeExtentList(
                    prototypeItem: const SizedBox(height: 64),
                    delegate: SliverChildBuilderDelegate(
                      (context, index) {
                        final note = group.notes[index];
                        final isSelected = selectedNotes.contains(note.id);

                        return Padding(
                          padding: const EdgeInsets.only(bottom: 8.0),
                          child: SizedBox(
                            height: 64,
                            child: MechanixSelectableList(
                              onLongPress: () => onSelect?.call(note.id),
                              onTap:
                                  () =>
                                      isSelectionMode
                                          ? onSelect?.call(note.id)
                                          : _openNote(context, note),
                              title: note.title,
                              selectionMode: isSelectionMode,
                              isSelected: isSelected,
                              trailingWidget: Row(
                                children: [
                                  Text(
                                    CommonHelper.formatDateTime(note.updatedAt),
                                    overflow: TextOverflow.ellipsis,
                                    style: const TextStyle(
                                      color: NotesColors.secondaryTextColor,
                                      fontSize: 14,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        );
                      },
                      childCount: group.notes.length,
                      addAutomaticKeepAlives: false,
                      addRepaintBoundaries: true,
                    ),
                  ),

                  const SliverToBoxAdapter(child: SizedBox(height: 16)),
                ],
              );
            }).toList(),
      ),
    ).padAll(16);
  }
}

class _GroupHeaderDelegate extends SliverPersistentHeaderDelegate {
  final String title;

  _GroupHeaderDelegate(this.title);

  @override
  Widget build(
    BuildContext context,
    double shrinkOffset,
    bool overlapsContent,
  ) {
    return Container(
      color: Colors.black.withOpacity(0.05),
      alignment: Alignment.centerLeft,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
      child: Row(
        children: [
          if (title == 'Pinned Notes')
            Image.asset(
              NotesIcon.pinnedFilledIcon,
              color: NotesColors.secondaryTextColor,
              height: 18,
              width: 18,
            ),
          if (title.isNotEmpty)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              child: Text(
                title,
                style: const TextStyle(
                  fontSize: 16,
                  color: NotesColors.secondaryTextColor,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
        ],
      ),
    );
  }

  @override
  double get maxExtent => 40;

  @override
  double get minExtent => 40;

  @override
  bool shouldRebuild(_GroupHeaderDelegate oldDelegate) =>
      oldDelegate.title != title;
}

void _openNote(BuildContext context, NoteMetaData note) async {
  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (_) => EditorBlocProvider(child: NotesEditor(note: note)),
    ),
  );
}
