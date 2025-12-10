import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/delete_bottom_sheet.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';

class BottomMenu extends StatefulWidget {
  final bool isSelectionMode;

  const BottomMenu({super.key, required this.isSelectionMode});

  @override
  State<BottomMenu> createState() => _BottomMenuState();
}

class _BottomMenuState extends State<BottomMenu> {
  @override
  void initState() {
    super.initState();
  }

  void confirmDelete(BuildContext context) {
    final notesToDelete = context.read<NotesBloc>().state.selectedNoteIds;

    final isSingle = notesToDelete.length == 1;
    final title =
        "Delete ${isSingle ? "this Note?" : "${notesToDelete.length} Notes ?"}";

    final message =
        "This action will delete the note${isSingle ? "" : "s"} permanently";

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return DeleteBottomSheet(
          title: title,
          message: message,
          bottomSheetContext: bottomSheetContext,
        );
      },
    );
  }

  void onDeleteRemoveSelection(BuildContext context) {
    confirmDelete(context);
  }

  void clearSelection() {
    context.read<NotesBloc>().add(ClearSelection());
  }

  @override
  void dispose() {
    super.dispose();
  }

  void selectAll() {
    context.read<NotesBloc>().add(SelectAllNotes());
  }

  @override
  Widget build(BuildContext context) {
    return MechanixBottomBar(
      leadingWidget: [
        BottomBarButton(
          iconTheme: const MechanixBottomBarIconThemeData(
            iconSize: Size(28, 28),
            iconBoxSize: Size(44, 44),
          ),
          onPressed: clearSelection,
          iconPath: NotesIcon.backIcon,
        ),
      ],
      centerWidget: [
        BottomBarButton(
          iconTheme: const MechanixBottomBarIconThemeData(
            iconSize: Size(28, 28),
            iconBoxSize: Size(44, 44),
          ),

          onPressed: selectAll,
          iconPath: NotesIcon.selectAllIcon,
        ),

        BottomBarButton(
          iconTheme: const MechanixBottomBarIconThemeData(
            iconSize: Size(28, 28),
            iconBoxSize: Size(44, 44),
          ),

          onPressed: () => {},
          iconPath: NotesIcon.shareIcon,
        ),

        BottomBarButton(
          iconTheme: const MechanixBottomBarIconThemeData(
            iconSize: Size(28, 28),
            iconBoxSize: Size(44, 44),
          ),

          onPressed: () => onDeleteRemoveSelection(context),
          iconPath: NotesIcon.deleteIcon,
        ),
      ],
    );
  }
}
