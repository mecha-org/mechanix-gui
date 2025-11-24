import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets/icon_widget.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class MenuOptions extends StatelessWidget {
  final NoteHive? note;
  final bool isPinned;

  const MenuOptions({super.key, required this.isPinned, this.note});

  @override
  Widget build(BuildContext context) {
    void onDelete() {
      if (note?.id != null) {
        context.read<NotesBloc>().add(DeleteNotes(deleteIds: [note!.id]));
      }
      Navigator.pop(context);
    }

    void onPin(bool isPinned) {
      context.read<EditorBloc>().add(PinnedUpdate(isPinned: !isPinned));
      if (note?.id != null) {
        context.read<NotesBloc>().add(
          PinnedNotes(isPinned: !isPinned, noteIds: [note!.id]),
        );
      }
    }

    return MechanixMenu(
      offset: const Offset(-16, 0),
      dropdownPosition: DropdownPosition.bottomRight,
      buttonIcon: const IconWidget(
        iconHeight: 20,
        iconWidth: 20,
        boxHeight: 20,
        boxWidth: 20,
        iconPath: NotesIcon.threeDotIcon,
        iconColor: Colors.white,
      ),
      items: [
        MechanixMenuItemsType(
          title: isPinned ? "Unpin" : "Pin",
          onTap: () => onPin(isPinned),
          trailing: SizedBox(
            child: Image.asset(
              isPinned ? NotesIcon.unPinnedIcon : NotesIcon.pinIcon,
              width: 18,
              height: 18,
              color: const Color(0xFFF0F0F0),
            ),
          ),
        ),

        MechanixMenuItemsType(
          trailing: SizedBox(
            child: Image.asset(
              height: 18,
              NotesIcon.deleteIcon,
              width: 18,
              color: const Color(0xFFFF4949),
            ),
          ),
          title: "Delete",
          onTap: () => onDelete(),
        ),
      ],
    );
  }
}
