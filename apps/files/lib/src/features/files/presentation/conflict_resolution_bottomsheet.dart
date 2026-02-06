import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/mechanix.dart';

class ConflictResolutionBottomSheet extends StatelessWidget {
  final List<String> conflictingPaths;
  final String destinationPath;
  final FileManagerController controller;

  const ConflictResolutionBottomSheet({
    super.key,
    required this.conflictingPaths,
    required this.destinationPath,
    required this.controller,
  });

  @override
  Widget build(BuildContext context) {
    final fileName = p.basename(conflictingPaths.first);
    final double sheetWidth = MediaQuery.of(context).size.width;

    return ClipPath(
      clipper: TabClipper(shift: 400),
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
              LayoutBuilder(
                builder: (context, constraints) {
                  const suffixText = ' already exists';

                  final suffixStyle = confirmationDialogRegularStyle(context);
                  final nameStyle = confirmationDialogBoldStyle(context);

                  final suffixWidth = textWidth(suffixText, suffixStyle);

                  final availableForName = constraints.maxWidth - suffixWidth;

                  final truncatedName = middleEllipsisString(
                    fileName,
                    availableForName,
                    nameStyle,
                  );

                  return RichText(
                    maxLines: 1,
                    overflow: TextOverflow.clip,
                    text: TextSpan(
                      children: [
                        TextSpan(
                          text: truncatedName,
                          style: nameStyle,
                        ),
                        TextSpan(
                          text: suffixText,
                          style: suffixStyle,
                        ),
                      ],
                    ),
                  );
                },
              ),
              const SizedBox(height: 8),
              Text(
                'Would you like to replace?',
                style: TextStyle(
                    color: context.colorScheme.onSurface,
                    fontSize: 20,
                    fontWeight: FontWeight.w400),
              ),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: MechanixFilledButton(
                      theme: buttonThemeData(context,
                          type: MechanixButtonType.cancel),
                      label: "Cancel",
                      onPressed: () {
                        context.read<FilesBloc>().add(
                              ContinueCopyWithConflictResolution(
                                sourcePaths: conflictingPaths,
                                destinationPath: destinationPath,
                                strategy: ConflictResolutionStrategy.skip,
                                controller: controller,
                              ),
                            );
                        Navigator.pop(context);
                      },
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: MechanixFilledButton(
                      theme: buttonThemeData(context,
                          type: MechanixButtonType.action),
                      label: "Replace",
                      onPressed: () {
                        context.read<FilesBloc>().add(
                              ContinueCopyWithConflictResolution(
                                sourcePaths: conflictingPaths,
                                destinationPath: destinationPath,
                                strategy: ConflictResolutionStrategy.replace,
                                controller: controller,
                              ),
                            );
                        Navigator.pop(context);
                      },
                    ),
                  ),
                ],
              ).padBottom(30),
            ],
          ),
        ),
      ),
    );
  }
}
