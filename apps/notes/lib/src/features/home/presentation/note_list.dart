import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/common_helper.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/mechanix.dart';

class GroupedNotes {
  final String label;
  final List<NoteHive> notes;
  final Widget? icon;
  GroupedNotes({required this.label, required this.notes, this.icon});
}

/// Updated NoteList that accepts grouped notes
class NoteList extends StatelessWidget {
  final bool isSelectionMode;
  final List<String> selectedNotes;
  final void Function(String id)? onSelect;
  final VoidCallback? onDeselect;
  final List<GroupedNotes> groupedNotes;

  const NoteList({
    super.key,
    required this.isSelectionMode,
    required this.selectedNotes,
    this.onSelect,
    this.onDeselect,
    required this.groupedNotes,
  });

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

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children:
          groupedNotes.map((group) {
            return Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    if (group.icon != null) group.icon!,
                    if (group.label.isNotEmpty)
                      Padding(
                        padding: const EdgeInsets.symmetric(vertical: 10),
                        child: Text(
                          group.label,
                          style: const TextStyle(
                            fontSize: 16,
                            color: NotesColors.secondaryTextColor,
                          ),
                        ),
                      ),
                  ],
                ),
                _buildNoteList(group.notes),
                const SizedBox(height: 20),
              ],
            );
          }).toList(),
    );
  }

  Widget _buildNoteList(List<NoteHive> notes) {
    return ListView.separated(
      physics: const NeverScrollableScrollPhysics(),
      itemCount: notes.length,
      shrinkWrap: true,
      separatorBuilder: (_, __) => const SizedBox(height: 8),
      itemBuilder: (context, index) {
        final note = notes[index];
        final isSelected = selectedNotes.contains(note.id);

        return SizedBox(
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
                        ? onSelect!(note.id)
                        : {_openNote(context, note)},
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
                  overflow: TextOverflow.ellipsis,
                  CommonHelper.formatDateTime(note.updatedAt),
                  style: const TextStyle(
                    color: NotesColors.secondaryTextColor,
                    fontSize: 14,
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

void _openNote(BuildContext context, NoteHive note) async {
  Navigator.push(
    context,
    MaterialPageRoute(builder: (_) => NotesEditor(note: note)),
  ).then((_) {
    if (context.mounted) {
      context.read<NotesBloc>().add(LoadNotes());
    }
  });
}
