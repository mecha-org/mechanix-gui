import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';

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

  void onDeleteRemoveSelection() {
    context.read<NotesBloc>().add(DeleteNotes(deleteIds: const []));
    clearSelection();
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
          onPressed: clearSelection,
          iconPath: NotesIcon.backIcon,
        ),
      ],
      centerWidget: [
        BottomBarButton(
          onPressed: selectAll,
          iconPath: NotesIcon.selectAllIcon,
        ),

        BottomBarButton(onPressed: () => {}, iconPath: NotesIcon.shareIcon),

        BottomBarButton(
          onPressed: onDeleteRemoveSelection,
          iconPath: NotesIcon.deleteIcon,
        ),
      ],
      anchorWidget: [
        BottomBarButton(
          onPressed: onDeleteRemoveSelection,
          iconPath: NotesIcon.deleteIcon,
        ),
      ],
    );
  }
}
