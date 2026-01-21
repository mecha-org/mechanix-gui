import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:widgets/mechanix.dart';

class ConfirmationBottomSheet extends StatelessWidget {
  final String filePath;

  const ConfirmationBottomSheet({
    super.key,
    required this.filePath,
  });

  @override
  Widget build(BuildContext context) {
    final double sheetWidth = MediaQuery.of(context).size.width;

    return ClipPath(
      clipper: TabClipper(shift: sheetWidth * 0.65),
      child: Container(
        decoration: BoxDecoration(
          color: context.colorScheme.surfaceContainerHigh,
        ),
        padding: const EdgeInsets.only(
          left: 16,
          right: 16,
          bottom: 16,
          top: 32,
        ),
        child: SafeArea(
          top: false,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Do you want to save changes?',
                style: regularStyle(context),
              ),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: MechanixFilledButton(
                      theme: buttonThemeData(
                        context,
                        type: MechanixButtonType.cancel,
                      ),
                      label: "Cancel",
                      onPressed: () {
                        Navigator.pop(context);
                      },
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: MechanixFilledButton(
                      theme: buttonThemeData(
                        context,
                        type: MechanixButtonType.action,
                      ),
                      label: "Save",
                      onPressed: () {
                        Navigator.pop(context, true);
                      },
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
