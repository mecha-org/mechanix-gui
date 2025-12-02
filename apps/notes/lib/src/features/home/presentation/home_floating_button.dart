import 'package:flutter/material.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

class HomeFloatingButton extends StatelessWidget {
  const HomeFloatingButton({super.key});
  @override
  Widget build(BuildContext context) {
    void onSearch() {
      Navigator.pushNamed(context, AppRoutes.searchNotes);
    }

    void createNote() {
      Navigator.pushNamed(context, AppRoutes.createEditNotes);
    }

    return Column(
      mainAxisAlignment: MainAxisAlignment.end,
      crossAxisAlignment: CrossAxisAlignment.center,
      spacing: 16,
      children: [
        Container(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(8),
            color: NotesColors.secondaryButtonColor,
          ),
          child: IconButton(
            hoverColor: Colors.transparent,
            splashColor: Colors.transparent,
            highlightColor: Colors.transparent,
            padding: const EdgeInsets.all(10),
            onPressed: onSearch,
            icon: Image.asset(NotesIcon.searchIcon, height: 24, width: 24),
          ),
        ),
        Container(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(8),
            color: NotesColors.secondaryCardColor,
          ),
          child: IconButton(
            hoverColor: Colors.transparent,
            splashColor: Colors.transparent,
            highlightColor: Colors.transparent,
            iconSize: 56,
            padding: const EdgeInsets.all(15),
            onPressed: createNote,
            icon: Image.asset(NotesIcon.addIcon, height: 24, width: 24),
          ),
        ),
      ],
    );
  }
}
