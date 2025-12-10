import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

class SelectionOptions extends StatelessWidget {
  const SelectionOptions({super.key});

  @override
  Widget build(BuildContext context) {
    return TextFieldTapRegion(
      child: Container(
        width: 252,
        height: 44,
        padding: const EdgeInsets.all(4),
        decoration: const BoxDecoration(
          color: NotesColors.floatingMenuColor,
          boxShadow: [
            BoxShadow(
              offset: Offset(0, 0),
              blurRadius: 8,
              spreadRadius: 0,
              color: Color(0x99000000), // #00000099
            ),
            BoxShadow(
              offset: Offset(0, 1),
              blurRadius: 5,
              spreadRadius: 0,
              color: Color(0x26424242), // #42424226
              blurStyle: BlurStyle.inner, // inset effect
            ),
            BoxShadow(
              offset: Offset(0, -1),
              blurRadius: 5,
              spreadRadius: 0,
              color: Color(0x40424242), // #42424240
              blurStyle: BlurStyle.inner, // inset effect
            ),
            BoxShadow(
              offset: Offset(0, 4),
              blurRadius: 4,
              spreadRadius: 0,
              color: Color(0x40000000), // #00000040
            ),
          ],
        ),
        child: Row(
          children: [
            IconButton(
              iconSize: 36,

              onPressed: () {
                print("Copy");
              },

              icon: Image.asset(height: 24, width: 24, NotesIcon.copyIcon),
            ),
            IconButton(
              iconSize: 36,

              onPressed: () {
                print("Cut");
              },
              icon: Image.asset(height: 24, width: 24, NotesIcon.cutIcon),
            ),
            IconButton(
              iconSize: 36,

              onPressed: () {
                print("Paste");
              },
              icon: Image.asset(height: 24, width: 24, NotesIcon.pasteIcon),
            ),
            IconButton(
              iconSize: 36,

              onPressed: () {
                print('Select All');
              },
              icon: Image.asset(
                height: 24,
                width: 24,
                NotesIcon.selectAllOptionIcon,
              ),
            ),
            IconButton(
              iconSize: 36,

              onPressed: () {
                print('Delete');
              },
              icon: Image.asset(height: 24, width: 24, NotesIcon.deleteIcon),
            ),
          ],
        ),
      ),
    );
  }
}
