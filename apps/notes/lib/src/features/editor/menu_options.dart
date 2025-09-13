import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/mechanix.dart';

class MenuOptions extends StatelessWidget {
  final LayerLink menuLink;
  final OverlayEntry? entry;
  final NoteHive? note;
  final bool? isPinned;
  final VoidCallback? togglePinned;

  const MenuOptions({
    super.key,
    required this.menuLink,
    this.entry,
    this.note,
    this.isPinned = false,
    this.togglePinned,
  });

  @override
  Widget build(BuildContext context) {
    void onDelete() {
      entry!.remove();
      if (note?.id != null) {
        context.read<NotesBloc>().add(DeleteNotes(deleteIds: [note!.id]));
      }
      Navigator.pop(context);
    }

    void onPin() {
      entry!.remove();
      if (note?.id != null) {
        context.read<NotesBloc>().add(
          PinnedNotes(isPinned: isPinned!, noteIds: [note!.id]),
        );
      }
      togglePinned!();
    }

    return Positioned.fill(
      child: Stack(
        children: [
          GestureDetector(
            behavior: HitTestBehavior.translucent,
            onTap: () => entry?.remove(),
          ),
          CompositedTransformFollower(
            link: menuLink,
            showWhenUnlinked: false,
            targetAnchor: Alignment.topRight,
            followerAnchor: Alignment.topRight,
            offset: const Offset(-20, 40),
            child: SizedBox(
              width: 185,
              child: MechanixMenu(
                backgroundColor: Color.fromRGBO(68, 68, 68, 0.95),
                items: [
                  MechanixMenuItem(
                    label: isPinned! ? "Unpin" : "Pin",
                    textStyle: TextStyle(
                      fontWeight: FontWeight.w500,
                      color: Color(0xFFF0F0F0),
                    ),
                    trailingWidget: SizedBox(
                      child: Image.asset(
                        isPinned! ? NotesIcon.unPinnedIcon : NotesIcon.pinIcon,
                        width: 18,
                        height: 18,
                        color: Color(0xFFF0F0F0),
                      ),
                    ),

                    onTap: () => onPin(),
                    layout: MenuItemLayout.iconRight,
                  ),

                  MechanixMenuDivider(thickness: 1, color: Color(0xFF333333)),
                  MechanixMenuItem(
                    trailingWidget: SizedBox(
                      child: Image.asset(
                        NotesIcon.deleteIcon,
                        height: 18,
                        width: 18,
                        color: Color(0xFFFF4949),
                      ),
                    ),
                    label: "Delete",
                    textStyle: TextStyle(
                      color: Color(0xFFFF4949),
                      fontWeight: FontWeight.w500,
                    ),
                    onTap: () => onDelete(),
                    layout: MenuItemLayout.iconRight,
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
