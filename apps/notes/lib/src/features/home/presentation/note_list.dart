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

// Model for flattened list items
abstract class _ListItem {}

class _HeaderItem extends _ListItem {
  final String title;
  _HeaderItem(this.title);
}

class _NoteItem extends _ListItem {
  final NoteMetaData note;
  _NoteItem(this.note);
}

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

  // Flatten groups for ListView
  List<_ListItem> _buildListItems() {
    final items = <_ListItem>[];
    for (final group in groupedNotes) {
      items.add(_HeaderItem(group.label));
      for (final note in group.notes) {
        items.add(_NoteItem(note));
      }
    }
    return items;
  }

  @override
  Widget build(BuildContext context) {
    if (groupedNotes.isEmpty) {
      return SizedBox(
        height: 64,
        child: MechanixPressableList(
          itemPadding: const EdgeInsets.all(10),
          onTap: () => Navigator.pushNamed(context, AppRoutes.createEditNotes),
          leadingIcon: Image.asset(NotesIcon.addIcon, height: 18, width: 18),
          isSelected: true,
          title: "Add a new Note",
          titleTextStyle: const TextStyle(
            color: NotesColors.secondaryTextColor,
            fontSize: 16,
          ),
        ),
      );
    }

    final items = _buildListItems();

    return ListView.builder(
      controller: controller,
      physics: const BouncingScrollPhysics(),
      prototypeItem: const SizedBox(height: 64),
      itemCount: items.length,
      itemBuilder: (context, index) {
        final item = items[index];
        if (item is _HeaderItem) {
          // Section Header
          return Container(
            height: 40,
            color: Colors.black.withOpacity(0.05),
            alignment: Alignment.centerLeft,
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
            child: Row(
              children: [
                if (item.title == 'Pinned Notes')
                  Image.asset(
                    NotesIcon.pinnedFilledIcon,
                    color: NotesColors.secondaryTextColor,
                    height: 18,
                    width: 18,
                  ),
                if (item.title.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 8),
                    child: Text(
                      item.title,
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
        } else if (item is _NoteItem) {
          final note = item.note;
          final isSelected = selectedNotes.contains(note.id);

          // Updated prototype item for note row
          return Padding(
            padding: const EdgeInsets.only(bottom: 8.0),
            child: SizedBox(
              height: 64,
              child: MechanixPressableList(
                leadingIconPadding: EdgeInsets.zero,
                checkboxSpacing: const EdgeInsets.only(right: 16, left: 6),
                itemPadding: const EdgeInsets.only(
                  left: 16,
                  right: 12,
                  top: 10,
                  bottom: 10,
                ),
                onLongPress: () => onSelect?.call(note.id),
                onTap:
                    () =>
                        isSelectionMode
                            ? onSelect?.call(note.id)
                            : _openNote(context, note),
                title: note.title,
                titleTextStyle: const TextStyle(
                  fontSize: 16,
                  color: NotesColors.titleTextColor,
                ),
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
        }
        return const SizedBox.shrink();
      },
    );
  }
}

void _openNote(BuildContext context, NoteMetaData note) async {
  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (_) => EditorBlocProvider(child: NotesEditor(note: note)),
    ),
  );
}
