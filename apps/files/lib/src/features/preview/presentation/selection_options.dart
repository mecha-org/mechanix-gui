import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';

class SelectionOptions extends StatelessWidget {
  const SelectionOptions({super.key});

  @override
  Widget build(BuildContext context) {
    return TextFieldTapRegion(
      child: Container(
        width: 252,
        height: 44,
        padding: const EdgeInsets.all(4),
        decoration: BoxDecoration(
          color: context.colorScheme.tertiary,
          boxShadow: const [
            BoxShadow(
              offset: Offset(0, 0),
              blurRadius: 8,
              spreadRadius: 0,
              color: Color(0x99000000),
            ),
            BoxShadow(
              offset: Offset(0, 1),
              blurRadius: 5,
              spreadRadius: 0,
              color: Color(0x26424242),
              blurStyle: BlurStyle.inner,
            ),
            BoxShadow(
              offset: Offset(0, -1),
              blurRadius: 5,
              spreadRadius: 0,
              color: Color(0x40424242),
              blurStyle: BlurStyle.inner,
            ),
            BoxShadow(
              offset: Offset(0, 4),
              blurRadius: 4,
              spreadRadius: 0,
              color: Color(0x40000000),
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
              icon: Image.asset(height: 24, width: 24, Images.copy),
            ),
            IconButton(
              iconSize: 36,
              onPressed: () {
                print("Cut");
              },
              icon: Image.asset(height: 24, width: 24, Images.cut),
            ),
            IconButton(
              iconSize: 36,
              onPressed: () {
                print("Paste");
              },
              icon: Image.asset(height: 24, width: 24, Images.paste),
            ),
            IconButton(
              iconSize: 36,
              onPressed: () {
                print('Select All');
              },
              icon: Image.asset(
                height: 24,
                width: 24,
                Images.listChecks,
              ),
            ),
            IconButton(
              iconSize: 36,
              onPressed: () {
                print('Delete');
              },
              icon: Image.asset(height: 24, width: 24, Images.delete),
            ),
          ],
        ),
      ),
    );
  }
}
