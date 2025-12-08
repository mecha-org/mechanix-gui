import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
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
      clipper: TabClipper(shift: sheetWidth * 0.65),
      child: Container(
        decoration: BoxDecoration(
          color: Colors.grey[850],
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
                '‘$fileName’ already exists',
                style: const TextStyle(
                  fontWeight: FontWeight.bold,
                  fontSize: 16,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 8),
              const Text(
                'What would you like to do?',
                style: TextStyle(color: Colors.white70),
              ),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: MechanixElevatedButton(
                      padding: const EdgeInsets.symmetric(
                          vertical: 16, horizontal: 12),
                      label: "Cancel",
                      textColor: Colors.white,
                      borderRadius: 8,
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
                    child: MechanixElevatedButton(
                      padding: const EdgeInsets.symmetric(
                          vertical: 16, horizontal: 12),
                      label: "Replace",
                      backgroundColor: Theme.of(context)
                          .extension<FilesTheme>()!
                          .primaryColor,
                      textColor: Colors.white,
                      borderRadius: 8,
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
              ),
            ],
          ),
        ),
      ),
    );
  }
}
