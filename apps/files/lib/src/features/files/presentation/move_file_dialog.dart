import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/notification/notification_type.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

final logger = Logger();
var totalMovedCount = 0;

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

class MoveBottomSheetContent extends StatefulWidget {
  final FilesBloc filesBloc;
  final int selectedCount;
  final VoidCallback reload;
  final String currentPath;
  final BuildContext rootContext;

  const MoveBottomSheetContent({
    required this.filesBloc,
    required this.selectedCount,
    required this.reload,
    required this.currentPath,
    required this.rootContext,
    super.key,
  });

  @override
  State<MoveBottomSheetContent> createState() => MoveBottomSheetContentState();
}

class MoveBottomSheetContentState extends State<MoveBottomSheetContent> {
  final FileManagerController controller = FileManagerController();
  final ScrollController _scrollController = ScrollController();
  String currentPath = "";
  final ValueNotifier<String> searchQuery = ValueNotifier("");
  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;

  bool showHomeView = false;
  bool isSearching = false;
  bool isCreateFolder = false;

  bool showRenameBar = false;
  String renameText = "";
  String createdFolderPath = "";
  String originalFolderName = "";

  @override
  void initState() {
    super.initState();

    currentPath = widget.currentPath;

    _scrollController.addListener(_onScroll);

    controller.openDirectory(Directory(widget.currentPath));

    controller.getPathNotifier.addListener(() {
      setState(() {
        currentPath = controller.getPathNotifier.value;
      });
    });

    searchQuery.addListener(() {
      if (searchQuery.value.isEmpty) {
        controller.reload();
      }
    });
  }

  void _onScroll() {
    if (!_scrollController.hasClients) return;
    final position = _scrollController.position;
    if (!position.hasPixels || !position.hasContentDimensions) return;

    final maxScroll = position.maxScrollExtent;
    final currentScroll = position.pixels;

    // Trigger near bottom
    if (currentScroll >= 0.8 * maxScroll) {
      controller.loadNextChunk();
    }
  }

  @override
  void dispose() {
    searchQuery.dispose();
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void handleBack() {
    controller.goToParentDirectory();
  }

  void homeNavigation() {
    setState(() {
      showHomeView = true;
    });
  }

  TextStyle listItemTitleTextStyle(BuildContext context) {
    return TextStyle(
      fontSize: 18,
      color: context.colorScheme.onSurface,
      fontWeight: FontWeight.w500,
    );
  }

  @override
  Widget build(BuildContext context) {
    final isAtRoot = currentPath == '/' || currentPath.isEmpty;

    final isDocumentsDir = currentPath == documentsDir;
    final isDownloadsDir = currentPath == downloadsDir;
    final isHomeDir = currentPath == homeDir;
    final isHomePageDir =
        isHomeDir || isDownloadsDir || isDocumentsDir || isAtRoot;

    final selectedPaths = widget.filesBloc.state.movedPaths;

    final selectedCount = selectedPaths.length;
    final label = selectedCount > 1
        ? " $selectedCount items"
        : " '${selectedPaths.first.split('/').last}'";

    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const SizedBox(height: 18),

        // Title
        if (!showHomeView)
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            child: Text(
              currentPath == '/' || currentPath.isEmpty
                  ? "Root"
                  : getCurrentFolderName(currentPath),
              style: TextStyle(
                color: context.colorScheme.onSurface,
                fontSize: 24,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),

        // Folder list
        Expanded(
          child: showHomeView
              ? buildHomeView(context)
              : buildListViewMoveAndExtract(
                  context, _scrollController, controller),
        ),

        const SizedBox(height: 18),
        Divider(
          height: 1,
          color: context.colorScheme.surfaceContainerLow,
        ), // Navigation bar
        SizedBox(
          height: 60,
          child: Row(
            children: [
              if (isSearching) ...[
                Expanded(
                  child: MechanixTextInput.search(
                    cursorColor: context.colorScheme.primaryFixed,
                    prefixIcon: IconWidget(
                      iconPath: Images.search,
                      iconColor: context.colorScheme.onSurface,
                      iconHeight: 24,
                      iconWidth: 24,
                    ),
                    hintText: "Search here",
                    onChanged: (query) {
                      searchQuery.value = query;

                      if (query.trim().length > 2) {
                        controller.search(query.trim());
                      }
                    },
                    onClear: () {
                      setState(() {
                        isSearching = false;
                        searchQuery.value = "";
                      });
                      controller.search('');
                    },
                  ),
                ),
              ] else if (showRenameBar) ...[
                _buildRenameDialog()
              ] else ...[
                // Entire MechanixBottomBar must be inside Row children
                Expanded(
                  child: MechanixBottomBar(
                    leadingWidget: [
                      BottomBarButton(
                        iconPath: Images.back,
                        onPressed: () {
                          (isHomePageDir ? homeNavigation() : handleBack());
                        },
                      ),
                    ],
                    anchorWidget: [
                      BottomBarButton(
                        iconWidget: IconWidget(
                          iconPath: Images.search,
                          iconColor: context.colorScheme.onSurface,
                          iconHeight: 28,
                          iconWidth: 28,
                        ),
                        onPressed: () {
                          setState(() => isSearching = true);
                        },
                      ),
                      BottomBarButton(
                        iconWidget: IconWidget(
                          iconPath: Images.home,
                          iconColor: context.colorScheme.onSurface,
                          iconHeight: 24,
                          iconWidth: 24,
                        ),
                        onPressed: () {
                          setState(() => showHomeView = true);
                        },
                      ),
                      BottomBarButton(
                        iconWidget: IconWidget(
                          iconPath: Images.createFolder,
                          iconColor: context.colorScheme.onSurface,
                          iconHeight: 28,
                          iconWidth: 28,
                        ),
                        onPressed: () async {
                          await createFolderAndRename();
                        },
                      ),
                    ],
                  ).padLeft(16).padRight(16),
                ),
              ]
            ],
          ),
        ),

        // Bottom bar
        Container(
          height: 60,
          padding: const EdgeInsets.symmetric(horizontal: 16),
          decoration: BoxDecoration(
            color: context.colorScheme.tertiary,
            borderRadius:
                const BorderRadius.vertical(bottom: Radius.circular(12)),
          ),
          child: Row(
            children: [
              Expanded(
                child: LayoutBuilder(
                  builder: (context, constraints) {
                    final prefix = 'Moving ';
                    final prefixStyle = regularStyle(context);
                    final labelStyle = boldStyle(context);

                    // Measure prefix width
                    final prefixWidth = textWidth(prefix, prefixStyle);

                    // Remaining width for label
                    final availableWidth = (constraints.maxWidth - prefixWidth)
                        .clamp(0.0, double.infinity);

                    final truncatedLabel = middleEllipsisString(
                      label,
                      availableWidth,
                      labelStyle,
                    );

                    return RichText(
                      maxLines: 1,
                      text: TextSpan(
                        children: [
                          TextSpan(
                            text: prefix,
                            style: prefixStyle,
                          ),
                          TextSpan(
                            text: truncatedLabel,
                            style: labelStyle,
                          ),
                        ],
                      ),
                    );
                  },
                ),
              ),
              const SizedBox(width: 8),
              MechanixFilledButton(
                theme: buttonThemeData(context,
                    type: MechanixButtonType.cancel, size: const Size(94, 40)),
                label: "Cancel",
                onPressed: () {
                  widget.filesBloc.add(CancelMoveMode());
                  Navigator.pop(context);
                },
              ),
              const SizedBox(width: 10),
              MechanixFilledButton(
                theme: buttonThemeData(context,
                    type: showHomeView
                        ? MechanixButtonType.disable
                        : MechanixButtonType.action,
                    size: const Size(94, 40)),
                label: "Move",
                onPressed: showHomeView
                    ? null
                    : () {
                        handlePaste(context, widget.filesBloc.state);
                        Navigator.pop(context, true);
                      },
              )
            ],
          ),
        ),
      ],
    );
  }

  Future<void> createFolderAndRename() async {
    final path = controller.getCurrentPath;
    final bloc = context.read<FilesBloc>();

    // Generate safe name
    final folderName = await generateUniqueFolderName(path);
    final newPath = p.join(path, folderName);

    bloc.add(CreateFolder(
      path: path,
      folderName: folderName,
      controller: controller,
    ));

    // Wait for folder to appear in UI (optional small delay)
    await Future.delayed(const Duration(milliseconds: 200));

    // Store rename target + default rename text
    setState(() {
      createdFolderPath = newPath;
      renameText = folderName;
      originalFolderName = folderName;
      showRenameBar = true;
      isCreateFolder = false;
    });
  }

  Widget _buildRenameDialog() {
    final bool isEmpty = renameText.trim().isEmpty;
    final bool isSame = renameText.trim() == originalFolderName.trim();
    final bool showCheck = !isEmpty && !isSame; // valid new name

    return Expanded(
      child: MechanixTextInput.textInput(
        autofocus: true,
        cursorColor: context.colorScheme.primaryFixed,
        initialValue: renameText,
        onChanged: (v) => setState(() => renameText = v),
        anchorWidget: showCheck
            ? IconButton(
                icon: Icon(Icons.check,
                    color: context.colorScheme.surfaceContainerLowest),
                onPressed: () {
                  final filesBloc = context.read<FilesBloc>();
                  filesBloc.add(
                    Rename(
                      oldPath: createdFolderPath,
                      newName: renameText,
                      controller: controller,
                    ),
                  );
                  setState(() => showRenameBar = false);
                  controller.clearNewFolder();
                },
              )
            : IconButton(
                icon: Icon(Icons.close,
                    color: context.colorScheme.surfaceContainerLowest),
                onPressed: () {
                  setState(() => showRenameBar = false);
                  controller.clearNewFolder();
                },
              ),
      ),
    );
  }

  Widget buildHomeView(BuildContext context) {
    return SingleChildScrollView(
      child: Container(
        margin: const EdgeInsets.all(12),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Title
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 8),
              child: Text(
                "Files",
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.w600,
                  color: context.colorScheme.onSurface,
                ),
              ),
            ),

            // Home directory
            MechanixSectionList(
                theme: MechanixSectionListThemeData(
                  backgroundColor: WidgetStateProperty.all(Colors.transparent),
                ),
                sectionListItems: [
                  SectionListItems(
                    title: "Home directory",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory(homeDir));
                    },
                    leading: Image.asset(Images.home, height: 20, width: 20),
                  ),

                  // Downloads
                  SectionListItems(
                    title: "Downloads",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory(downloadsDir));
                    },
                    leading:
                        Image.asset(Images.downloads, height: 24, width: 24),
                  ),

                  // Documents
                  SectionListItems(
                    title: "Documents",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory(documentsDir));
                    },
                    leading: Image.asset(Images.homeDocuments,
                        height: 24, width: 24),
                  ),
                ]),

            // Root dir
            MechanixSectionList(
                title: 'Hard Drive',
                theme: MechanixSectionListThemeData(
                  backgroundColor: WidgetStateProperty.all(Colors.transparent),
                  titleTextStyle: TextStyle(
                    fontSize: 18,
                    color: context.colorScheme.onSurfaceVariant,
                    fontWeight: FontWeight.w500,
                  ),
                ),
                sectionListItems: [
                  SectionListItems(
                    title: "Root (/)",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory("/"));
                    },
                    leading:
                        Image.asset(Images.hardDrive, height: 24, width: 24),
                  ),
                ]),
          ],
        ),
      ),
    );
  }

  Future<void> handlePaste(BuildContext context, FilesState state) async {
    final targetPath = currentPath;
    final bloc = BlocProvider.of<FilesBloc>(context);

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
    widget.reload();

    MechanixNotification.show(
      context: context,
      notificationType: NotificationType.success,
      message:
          "Moved $movePathCount item${movePathCount > 1 ? 's' : ''} to '$folderName'",
    );
  }
}

Future<void> handleConflictsSequentially(
  BuildContext context,
  List<FileConflict> conflicts,
) async {
  final filesBloc = context.read<FilesBloc>();

  for (final conflict in conflicts) {
    if (!context.mounted) return;

    final double sheetWidth = MediaQuery.of(context).size.width;

    final strategy = await showModalBottomSheet<ConflictResolutionStrategy>(
      context: context,
      useRootNavigator: true,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (sheetContext) {
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
                  LayoutBuilder(
                    builder: (context, constraints) {
                      final fileStyle = boldStyle(context);
                      final suffixStyle = regularStyle(context);
                      const suffix = ' already exists';

                      // Measure suffix width
                      final suffixWidth = textWidth(suffix, suffixStyle);

                      // Remaining width for filename
                      final availableWidth =
                          (constraints.maxWidth - suffixWidth)
                              .clamp(0.0, double.infinity);

                      final truncatedName = middleEllipsisString(
                        conflict.fileName,
                        availableWidth,
                        fileStyle,
                      );

                      return RichText(
                        maxLines: 1,
                        text: TextSpan(
                          children: [
                            TextSpan(
                              text: truncatedName,
                              style: fileStyle,
                            ),
                            TextSpan(
                              text: suffix,
                              style: suffixStyle,
                            ),
                          ],
                        ),
                      );
                    },
                  ),
                  const SizedBox(height: 8),
                  Text(
                    'What would you like to do?',
                    style: TextStyle(
                        color: context.colorScheme.onSurface, fontSize: 16),
                  ),
                  const SizedBox(height: 16),
                  Row(
                    children: [
                      Expanded(
                        child: MechanixFilledButton(
                          onPressed: () {
                            totalMovedCount--;
                            Navigator.pop(
                              sheetContext,
                              ConflictResolutionStrategy.skip,
                            );
                          },
                          theme: buttonThemeData(context,
                              type: MechanixButtonType.cancel),
                          label: "Cancel",
                        ),
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: MechanixFilledButton(
                          theme: buttonThemeData(context,
                              type: MechanixButtonType.action),
                          label: "Replace",
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
        'Warning: waiting for conflict resolution timed out for ${conflict.fileName}: $e',
      );
    }
  }
}

Future<void> showInvalidMoveSheet(
  BuildContext context,
  int movePathCount,
) async {
  final double sheetWidth = MediaQuery.of(context).size.width;

  await showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    builder: (sheetContext) {
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
                  "You cannot move a file over itself",
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
                          final bloc = BlocProvider.of<FilesBloc>(context);
                          bloc.add(CancelMoveMode());
                          Navigator.pop(sheetContext);
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
                        label: movePathCount > 1 ? "Skip All" : "Skip",
                        onPressed: () => Navigator.pop(sheetContext),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      );
    },
  );
}
