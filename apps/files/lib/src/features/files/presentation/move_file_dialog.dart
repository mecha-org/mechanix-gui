import 'dart:async';
import 'dart:io';
import 'dart:io' as io;

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/searchbar/mechanix_search_bar.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

final logger = Logger();

class MoveExplorerBottomSheet extends StatefulWidget {
  final String path;
  final String title;
  final FilesBloc filesBloc;
  final FilesBloc filesBlocMainContext;
  final VoidCallback onMoveCompleted;
  final BuildContext rootContext;

  const MoveExplorerBottomSheet(
      {super.key,
      required this.path,
      required this.title,
      required this.filesBloc,
      required this.filesBlocMainContext,
      required this.onMoveCompleted,
      required this.rootContext});

  @override
  State<MoveExplorerBottomSheet> createState() =>
      _MoveExplorerBottomSheetState();
}

class _MoveExplorerBottomSheetState extends State<MoveExplorerBottomSheet> {
  final FileManagerController controller = FileManagerController();
  late String currentPath;
  bool isSearching = false;
  final TextEditingController searchController = TextEditingController();
  final FocusNode searchFocusNode = FocusNode();

  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;
  final recentDir = AppConfig().recentDir;

  Future<void> _loadFiles() async {
    await controller.openDirectory(Directory(widget.path));
  }

  @override
  void initState() {
    super.initState();

    currentPath = widget.path;
    _loadFiles();

    controller.getPathNotifier.addListener(() {
      setState(() {
        currentPath = controller.getPathNotifier.value;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    final isAtRoot = currentPath == '/' || currentPath.isEmpty;

    final isDocumentsDir = currentPath == documentsDir;
    final isDownloadsDir = currentPath == downloadsDir;
    final isHomeDir = currentPath == homeDir;
    final isRecentDir = currentPath == recentDir;
    final isHomePageDir = isHomeDir ||
        isDownloadsDir ||
        isDocumentsDir ||
        isAtRoot ||
        isRecentDir;

    return ValueListenableBuilder<List<io.FileSystemEntity>>(
      valueListenable: controller.paginatedEntities,
      builder: (context, entities, _) {
        final foldersList = entities
            .where((file) =>
                file is io.Directory &&
                !p
                    .basename(file.path)
                    .startsWith('.')) // exclude hidden folders
            .toList();

        if (foldersList.length < pageSize) {
          controller.loadNextChunk();
        }

        final itemCount = foldersList.length;

        // Base height logic
        double minChildSize;
        double initialChildSize;
        double maxChildSize = 0.7;

        if (itemCount <= 2) {
          minChildSize = 0.35;
          initialChildSize = 0.4;
        } else if (itemCount <= 5) {
          minChildSize = 0.4;
          initialChildSize = 0.5;
        } else if (itemCount <= 10) {
          minChildSize = 0.5;
          initialChildSize = 0.6;
        } else {
          minChildSize = 0.6;
          initialChildSize = 0.7;
        }
        return DraggableScrollableSheet(
          expand: false,
          initialChildSize: initialChildSize,
          minChildSize: minChildSize,
          maxChildSize: maxChildSize,
          builder: (context, sheetController) {
            // Scroll controller listener
            sheetController.addListener(() {
              final maxScroll = sheetController.position.maxScrollExtent;
              final currentScroll = sheetController.position.pixels;

              if (currentScroll >= 0.8 * maxScroll) {
                controller.loadNextChunk();
              }
            });

            return Container(
              decoration: BoxDecoration(
                color: Colors.grey[850],
                borderRadius:
                    const BorderRadius.vertical(top: Radius.circular(16)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  MechanixNavigationBar(
                    backgroundColor: Colors.grey[850],
                    leadingWidget: IconButton(
                      icon: const Icon(
                        Icons.arrow_back_ios,
                        size: 20,
                        color: Colors.blue,
                      ),
                      onPressed: () {
                        if (isHomePageDir) {
                          Navigator.pop(context);
                          moveMainBottomSheet(
                              widget.onMoveCompleted, controller);
                        } else {
                          controller.goToParentDirectory();
                        }
                      },
                    ),
                    title:
                        isAtRoot ? "Root" : getCurrentFolderName(currentPath),
                    titleStyle: const TextStyle(
                      color: Colors.white,
                      fontSize: 18,
                    ),
                    actionWidgets: [
                      if (!isSearching)
                        IconButton(
                          icon: const Icon(Icons.search, color: Colors.white),
                          onPressed: () {
                            setState(() {
                              isSearching = true;
                              searchFocusNode.requestFocus();
                            });
                          },
                        ),
                    ],
                  ).padTop(8),
                  Expanded(
                    child: buildListViewMove(
                      foldersList,
                      context,
                      currentPath,
                      widget.filesBloc,
                      widget.onMoveCompleted,
                      sheetController,
                    ),
                  ),
                  if (isSearching)
                    Padding(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 12, vertical: 12),
                      child: SizedBox(
                        height: 48,
                        child: MechanixSearchBar(
                          controller: searchController,
                          autoFocus: true,
                          hintText: "Type here",
                          onChanged: (query) {
                            controller.search(query);
                          },
                          onCloseIconPress: () {
                            setState(() {
                              searchController.clear();
                              isSearching = false;
                            });

                            // Reload directory content when clearing search
                            controller.reload();
                          },
                        ),
                      ),
                    )
                  else
                    Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 12, vertical: 12),
                      decoration: BoxDecoration(
                        color: Colors.grey[850],
                        borderRadius: const BorderRadius.vertical(
                            bottom: Radius.circular(16)),
                      ),
                      child: Row(
                        children: [
                          Expanded(
                            child: OutlinedButton(
                              style: OutlinedButton.styleFrom(
                                side: const BorderSide(
                                    color: Colors.white70, width: 0.4),
                                backgroundColor: Colors.transparent,
                              ),
                              onPressed: () {
                                Navigator.pop(context);
                              },
                              child: const Icon(Icons.close,
                                  color: Colors.white70),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: ElevatedButton(
                              style: ElevatedButton.styleFrom(
                                backgroundColor: Colors.grey[800],
                              ),
                              onPressed: () {
                                handlePaste(context, widget.filesBloc.state);

                                Navigator.pop(context, true);
                              },
                              child: const Icon(Icons.check,
                                  color: Colors.white70),
                            ),
                          ),
                        ],
                      ),
                    ),
                ],
              ),
            );
          },
        );
      },
    );
  }

  void moveMainBottomSheet(onMoveCompleted, FileManagerController controller) {
    final filesBloc = BlocProvider.of<FilesBloc>(context); // get bloc

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.grey[850],
      isScrollControlled: true,
      builder: (context) {
        return SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'Select destination',
                style: TextStyle(
                  fontSize: 16,
                  color: Colors.grey,
                ),
              ).padLeft(18).padBottom(4).padTop(14),
              MechanixSectionListTheme(
                style: MechanixSectionListThemeData(
                  height: 42,
                  dividerPadding: EdgeInsets.zero,
                  widgetPadding: EdgeInsets.zero,
                  backgroundColor: WidgetStateProperty.all(Colors.grey[850]),
                ),
                child: MechanixSectionList(
                  sectionListItems: [
                    SectionListItems(
                        title: "Home directory",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, homeDir, "Home", filesBloc,
                            onMoveCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.home,
                          iconColor: Colors.blueAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Downloads",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, downloadsDir, "Downloads",
                            filesBloc, onMoveCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.downloads,
                          iconColor: Colors.deepPurpleAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Documents",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, documentsDir, "Documents",
                            filesBloc, onMoveCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.homeDocuments,
                          iconColor: Colors.orangeAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Root (/)",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(
                            context, "/", "Root", filesBloc, onMoveCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.hardDrive,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  Future<void> handlePaste(BuildContext context, FilesState state) async {
    final targetPath = currentPath;
    final bloc = BlocProvider.of<FilesBloc>(context);

    // Show SnackBar
    final movePathCount = state.movedPaths.length;
    final folderName = targetPath.split('/').last;

    // Check if any moved file/folder is being moved into its own parent folder
    final hasInvalidMove = state.movedPaths.any((sourcePath) {
      final sourceParent = p.dirname(sourcePath);
      return sourceParent == targetPath;
    });

    if (hasInvalidMove) {
      await Future.delayed(
          const Duration(milliseconds: 200)); // allow animation
      if (widget.rootContext.mounted) {
        await showInvalidMoveSheet(widget.rootContext, movePathCount);
      }
      return;
    }

    final completer = Completer<void>();
    bloc.add(Move(
      sourcePaths: state.movedPaths,
      destinationPath: targetPath,
      completer: completer,
    ));
    await completer.future;
    bloc.add(CancelMoveMode());

    // safe to reload
    widget.onMoveCompleted();

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
            "Moved $movePathCount item${movePathCount > 1 ? 's' : ''} to '$folderName'",
            style: const TextStyle(color: Colors.white)),
        duration: const Duration(seconds: 2),
        backgroundColor: Colors.grey[800],
      ),
    );
  }
}

Future<void> showInvalidMoveSheet(
    BuildContext context, int movePathCount) async {
  await showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    builder: (sheetContext) {
      return Container(
        decoration: BoxDecoration(
          color: Colors.grey[850],
          borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
        ),
        padding: const EdgeInsets.all(16),
        child: SafeArea(
          top: false,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                "You cannot move a file over itself",
                style: Theme.of(context).textTheme.titleMedium,
              ),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: MechanixOutlinedButton(
                      label: "Cancel",
                      textColor: Colors.white,
                      borderRadius: 50,
                      borderWidth: 0.5,
                      onPressed: () => Navigator.pop(sheetContext),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: MechanixElevatedButton(
                      label: movePathCount > 1 ? "Skip All" : "Skip",
                      backgroundColor: Colors.blue,
                      textColor: Colors.white,
                      borderRadius: 50,
                      onPressed: () => Navigator.pop(sheetContext),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      );
    },
  );
}

var totalMovedCount = 0;

void onTap(
  BuildContext context,
  String path,
  String title,
  FilesBloc filesBlocMain,
  VoidCallback onMoveCompleted,
) {
  final localBloc = FilesBloc(
    fileRepository: filesBlocMain.fileRepository,
    recentFilesManager: filesBlocMain.recentFilesManager,
  );

  // Clone movedPaths from main bloc
  localBloc.emit(
    localBloc.state.copyWith(
      movedPaths: List<String>.from(filesBlocMain.state.movedPaths),
    ),
  );

  // Close any existing sheet before opening new one
  Navigator.pop(context, true);

  showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
    ),
    builder: (_) {
      return BlocProvider.value(
        value: localBloc,
        child: BlocListener<FilesBloc, FilesState>(
          listenWhen: (prev, curr) =>
              prev.conflictingPaths != curr.conflictingPaths,
          listener: (context, state) async {
            if (!state.loading &&
                state.conflictingPaths.isNotEmpty &&
                state.isMoveMode) {
              final rootContext = Navigator.of(context).context;
              final conflicts = state.conflictingPaths.map((path) {
                return FileConflict(
                  path: path,
                  fileName: p.basename(path),
                  destination: state.conflictDestinationPath,
                );
              }).toList();

              totalMovedCount = state.movedPaths.length;

              await _handleConflictsSequentially(rootContext, conflicts);

              // After all conflicts are done, show snackbar + refresh
              if (rootContext.mounted) {
                final folderName = p.basename(state.conflictDestinationPath);

                ScaffoldMessenger.of(rootContext).showSnackBar(
                  SnackBar(
                    content: Text(
                      totalMovedCount > 0
                          ? "Moved $totalMovedCount item${totalMovedCount > 1 ? 's' : ''} to '$folderName'"
                          : "No items were moved",
                      style: const TextStyle(color: Colors.white),
                    ),
                    duration: const Duration(seconds: 2),
                    backgroundColor: Colors.grey[800],
                  ),
                );

                // Ensure MoveMode exits and trigger reload
                filesBlocMain.add(CancelMoveMode());
                onMoveCompleted();
              }
            }
          },
          child: MoveExplorerBottomSheet(
            path: path,
            title: title,
            filesBloc: localBloc,
            filesBlocMainContext: filesBlocMain,
            onMoveCompleted: onMoveCompleted,
            rootContext: Navigator.of(context, rootNavigator: true).context,
          ),
        ),
      );
    },
  );
}

Future<void> _handleConflictsSequentially(
  BuildContext context,
  List<FileConflict> conflicts,
) async {
  final filesBloc = context.read<FilesBloc>();

  for (final conflict in conflicts) {
    if (!context.mounted) return;
    final strategy = await showModalBottomSheet<ConflictResolutionStrategy>(
      context: context,
      useRootNavigator: true,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (sheetContext) {
        return Container(
          decoration: BoxDecoration(
            color: Colors.grey[850],
            borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
          ),
          padding: const EdgeInsets.all(16),
          child: SafeArea(
            top: false,
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  '“${conflict.fileName}” already exists',
                  style: Theme.of(context).textTheme.titleMedium,
                ),
                const SizedBox(height: 8),
                const Text('What would you like to do?'),
                const SizedBox(height: 16),
                Row(
                  children: [
                    Expanded(
                      child: MechanixOutlinedButton(
                          label: "Skip",
                          textColor: Colors.white,
                          borderRadius: 50,
                          borderWidth: 0.5,
                          onPressed: () {
                            totalMovedCount--;
                            Navigator.pop(
                              sheetContext,
                              ConflictResolutionStrategy.skip,
                            );
                          }),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: MechanixElevatedButton(
                        label: "Replace",
                        backgroundColor: Colors.blue,
                        textColor: Colors.white,
                        borderRadius: 50,
                        onPressed: () => Navigator.pop(
                          sheetContext,
                          ConflictResolutionStrategy.replace,
                        ),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        );
      },
    );

    if (strategy == null) continue;

    filesBloc.add(
      ContinueMoveWithConflictResolution(
        sourcePaths: [conflict.path],
        destinationPath: conflict.destination,
        strategy: strategy,
      ),
    );

    try {
      await filesBloc.stream
          .firstWhere(
            (s) =>
                (!s.conflictingPaths.contains(conflict.path) && !s.loading) ||
                (!s.isMoveMode),
          )
          .timeout(const Duration(seconds: 5));
    } catch (e) {
      logger.i(
          'Warning: waiting for conflict resolution timed out for ${conflict.fileName}: $e');
    }
  }
}

class FileConflict {
  final String path;
  final String fileName;
  final String destination;

  const FileConflict({
    required this.path,
    required this.fileName,
    required this.destination,
  });
}
