import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class EditorMenu extends StatelessWidget {
  final String? noteId;
  final VoidCallback onTapFocus;
  const EditorMenu({super.key, this.noteId, required this.onTapFocus});

  @override
  Widget build(BuildContext context) {
    return MechanixMenu(
      animationDuration: const Duration(milliseconds: 100),
      topTabWidth: 10,
      topTabRightSideShiftLength: 50,
      dropdownPosition: DropdownPosition.topRight,
      padding: const EdgeInsets.only(top: 0),
      theme: const MechanixMenuThemeData(
        dropdownWidth: 135,
        buttonMargin: EdgeInsets.only(right: 12),
      ),
      offset: const Offset(-5, -15),
      openMenu: () => onTapFocus(),
      buttonIcon: const IconWidget(
        boxHeight: 28,
        boxWidth: 28,
        iconHeight: 28,
        iconWidth: 28,
        iconColor: Colors.white,
        iconPath: NotesIcon.threeDotIcon,
        activeIconColor: NotesColors.secondaryTextColor,
      ),
      items: [
        const MechanixMenuItemsType(
          title: "Share",
          leading: IconWidget(
            iconPath: NotesIcon.shareIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () {
            if (noteId != null) {
              context.read<NotesBloc>().add(DeleteNotes(deleteIds: [noteId!]));
            }
            onTapFocus();
            Navigator.pop(context);
          },
          title: "Delete",
          leading: const IconWidget(
            iconPath: NotesIcon.deleteIcon,
            iconColor: Colors.white,
          ),
        ),
      ],
    );
  }
}
