import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_button.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_loading_dialog.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:mechanix_files/src/features/files/presentation/move_file_dialog.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/widgets/bottomSheetModals/mechanix_bottom_sheet_theme.dart';
import 'package:widgets/widgets/floatingActionButton/mechanix_fab_items.dart';
import 'package:widgets/widgets/listItems/mechanix_simple_list_theme.dart';
import 'package:widgets/widgets/menu/mechanix_menu_item_theme.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/textInput/mechanix_text_input_theme.dart';
import 'view_mode_notifier.dart';
import 'grid_view.dart';
import 'list_view.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:file/file.dart';
import 'package:path/path.dart' as p;
import 'dart:math' as Math;
import 'package:widgets/mechanix.dart';

class FileExplorerPage extends StatefulWidget {
  final String title;
  final List<FileItem> path;

  const FileExplorerPage({
    super.key,
    this.title = "My Files",
    this.path = const [],
  });

  @override
  State<FileExplorerPage> createState() => FileExplorerPageState();
}

class FileExplorerPageState extends State<FileExplorerPage> {
  List<FileItem> displayedFiles = [];
  bool selectionMode = false;
  Set<String> selectedPaths = {};
  bool showHiddenFiles = false;
  bool isLoadingDialogShown = false;
  bool isCopyMode = false;
  bool isMoveMode = false;
  bool isExtractMode = false;
  String zipFilePath = '';
  List<String> copiedPaths = [];
  List<String> movedPaths = [];
  final LayerLink _menuLink = LayerLink();
  OverlayEntry? _menuEntry;
  bool _isMenuOpen = false;

  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;

  @override
  Widget build(BuildContext context) {
    final isAtRoot = widget.path.isEmpty;
    final backIndex = widget.path.length - 1;
    final currentTitle = backIndex < -1
        ? "Files"
        : (backIndex == -1 ? "Root" : widget.path[backIndex].name);

    String? backTitle = widget.path.length > 1
        ? widget.path[widget.path.length - 2].name
        : null;
    List<FileItem> backPath = widget.path.length > 1
        ? widget.path.sublist(0, widget.path.length - 1)
        : [];

    void handleBack() {
      final fullPath = '/${backPath.map((e) => e.name).join('/')}';
      final filesBloc = BlocProvider.of<FilesBloc>(context);

      filesBloc.add(LoadFilesAtPath(fullPath.isEmpty ? '/' : fullPath));

      Navigator.pushReplacement(
        context,
        MaterialPageRoute(
          builder: (_) => BlocProvider.value(
            value: filesBloc,
            child: FileExplorerPage(
              title: backTitle ?? "Files",
              path: backPath,
            ),
          ),
        ),
      );
    }

    void homeNavigation() {
      final filesBloc = BlocProvider.of<FilesBloc>(context);

      Navigator.pushReplacement(
        context,
        MaterialPageRoute(
          builder: (_) => BlocProvider.value(
            value: filesBloc,
            child: FileHomePage(),
          ),
        ),
      );
    }

    return MultiBlocListener(
      listeners: [
        // Compression dialog handler
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (previous, current) =>
              previous.compressionStatus != current.compressionStatus,
          listener: (context, state) {
            if (state.compressionStatus == FileCompressionStatus.inProgress) {
              showDialog(
                context: context,
                barrierColor: Colors.black.withOpacity(0.2),
                barrierDismissible: false,
                builder: (_) => buildLoadingDialog("Compressing..."),
              );
            } else if (state.compressionStatus ==
                    FileCompressionStatus.success ||
                state.compressionStatus == FileCompressionStatus.failure) {
              Navigator.of(context, rootNavigator: true).pop();

              if (state.compressionStatus == FileCompressionStatus.failure) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content:
                        Text('Compression failed: ${state.compressionError}'),
                  ),
                );
              }
            }
          },
        ),
        // Global error message handler
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (previous, current) =>
              previous.error != current.error && current.error != null,
          listener: (context, state) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(
                  'Error: ${state.error}',
                  style: const TextStyle(color: Colors.red),
                ),
              ),
            );
          },
        ),
        // Loading indicator for general loading states
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (previous, current) =>
              previous.loading != current.loading,
          listener: (context, state) async {
            if (state.loading && !isLoadingDialogShown) {
              isLoadingDialogShown = true;
              await showDialog(
                context: context,
                barrierColor: Colors.black.withOpacity(0.2),
                barrierDismissible: false,
                builder: (_) => buildLoadingDialog("Loading..."),
              );
              isLoadingDialogShown = false;
            } else if (!state.loading && isLoadingDialogShown) {
              Navigator.of(context, rootNavigator: true).pop();
            }
          },
        ),
        // Copy: Show conflict resolution dialog
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (prev, curr) =>
              prev.conflictingPaths != curr.conflictingPaths,
          listener: (context, state) {
            if (!state.loading &&
                state.conflictingPaths.isNotEmpty &&
                state.isCopyMode) {
              final fileName = p.basename(state.conflictingPaths.first);
              showDialog(
                context: context,
                builder: (_) => AlertDialog(
                  backgroundColor: Colors.grey[800],
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  title: Text(
                    'Confirm save as',
                    style: const TextStyle(
                        fontWeight: FontWeight.bold, fontSize: 16),
                  ),
                  content: Text(
                    '‘$fileName’ already exists, do you want to replace it?',
                  ),
                  actions: [
                    Padding(
                      padding: const EdgeInsets.only(right: 8.0, bottom: 8.0),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.end,
                            children: [
                              CustomButton(
                                label: "Cancel",
                                backgroundColor: Colors.grey[700]!,
                                textColor: Colors.white,
                                onPressed: () => {
                                  context.read<FilesBloc>().add(
                                        ContinueCopyWithConflictResolution(
                                          sourcePaths: state.conflictingPaths,
                                          destinationPath:
                                              state.conflictDestinationPath,
                                          strategy:
                                              ConflictResolutionStrategy.skip,
                                        ),
                                      ),
                                  Navigator.pop(context)
                                },
                              ),
                              const SizedBox(width: 12),
                              CustomButton(
                                label: "Replace and save",
                                backgroundColor: Colors.blue,
                                textColor: Colors.white,
                                onPressed: () {
                                  context.read<FilesBloc>().add(
                                        ContinueCopyWithConflictResolution(
                                          sourcePaths: state.conflictingPaths,
                                          destinationPath:
                                              state.conflictDestinationPath,
                                          strategy: ConflictResolutionStrategy
                                              .replace,
                                        ),
                                      );
                                  Navigator.pop(context);
                                },
                              ),
                            ],
                          )
                        ],
                      ),
                    ),
                  ],
                ),
              );
            }
          },
        ),
        // Move: Show conflict resolution dialog
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (prev, curr) =>
              prev.conflictingPaths != curr.conflictingPaths,
          listener: (context, state) {
            if (!state.loading &&
                state.conflictingPaths.isNotEmpty &&
                state.isMoveMode) {
              final fileName = p.basename(state.conflictingPaths.first);
              showDialog(
                context: context,
                builder: (_) => AlertDialog(
                  backgroundColor: Colors.grey[800],
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  title: Text(
                    'Confirm save as',
                    style: const TextStyle(
                        fontWeight: FontWeight.bold, fontSize: 16),
                  ),
                  content: Text(
                    '‘$fileName’ already exists, do you want to replace it?',
                  ),
                  actions: [
                    Padding(
                      padding: const EdgeInsets.only(right: 8.0, bottom: 8.0),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.end,
                            children: [
                              CustomButton(
                                label: "Cancel",
                                backgroundColor: Colors.grey[700]!,
                                textColor: Colors.white,
                                onPressed: () => {
                                  context.read<FilesBloc>().add(
                                        ContinueMoveWithConflictResolution(
                                          sourcePaths: state.conflictingPaths,
                                          destinationPath:
                                              state.conflictDestinationPath,
                                          strategy:
                                              ConflictResolutionStrategy.skip,
                                        ),
                                      ),
                                  Navigator.pop(context)
                                },
                              ),
                              const SizedBox(width: 12),
                              CustomButton(
                                label: "Replace and save",
                                backgroundColor: Colors.blue,
                                textColor: Colors.white,
                                onPressed: () {
                                  context.read<FilesBloc>().add(
                                        ContinueMoveWithConflictResolution(
                                          sourcePaths: state.conflictingPaths,
                                          destinationPath:
                                              state.conflictDestinationPath,
                                          strategy: ConflictResolutionStrategy
                                              .replace,
                                        ),
                                      );
                                  Navigator.pop(context);
                                },
                              ),
                            ],
                          )
                        ],
                      ),
                    ),
                  ],
                ),
              );
            }
          },
        ),
      ],
      child: BlocBuilder<FilesBloc, FilesState>(
        builder: (context, state) {
          final fileSystemList = state.fileSystemList;
          displayedFiles = getFilesAtPath(widget.path, fileSystemList);

          isCopyMode = state.isCopyMode;
          copiedPaths = state.copiedPaths;

          isMoveMode = state.isMoveMode;
          movedPaths = state.movedPaths;
          zipFilePath = state.zipFilePath;
          isExtractMode = state.isExtractMode;
          showHiddenFiles = state.showHiddenFiles;

          return Scaffold(
            appBar: MechanixNavigationBar(
              leadingWidget: IconButton(
                icon: const Icon(
                  Icons.arrow_back_ios,
                  size: 20,
                  color: Colors.blue,
                ),
                onPressed: selectionMode
                    ? clearSelection
                    : (isAtRoot ? homeNavigation : handleBack),
              ),
              title: selectionMode ? "Select" : currentTitle,
              titleStyle: context.textTheme.titleLarge,
              actionWidgets: selectionMode
                  ? [
                      Text(
                        "${selectedPaths.length} item${selectedPaths.length > 1 ? 's' : ''} selected",
                        style: const TextStyle(
                          color: Colors.grey,
                          fontSize: 14,
                        ),
                      ).padRight(24),
                    ]
                  : [
                      ValueListenableBuilder<bool>(
                        valueListenable: viewModeNotifier,
                        builder: (context, isList, _) {
                          return IconButton(
                            icon:
                                Image.asset(isList ? Images.list : Images.grid),
                            onPressed: () {
                              viewModeNotifier.value = !viewModeNotifier.value;
                            },
                            highlightColor: Colors.transparent,
                          );
                        },
                      ),
                      IconButton(
                        icon: Image.asset(Images.sortAscending),
                        onPressed: () async {
                          showSortMenu(context, state.currentSortBy);
                        },
                        highlightColor:
                            Colors.transparent, // Remove ripple effect on press
                      ),
                      IconButton(
                        icon: const Icon(Icons.search, color: Colors.white),
                        onPressed: () {
                          // Implement search logic
                        },
                        highlightColor: Colors.transparent,
                      ),
                      Builder(
                        builder: (context) {
                          return MechanixBottomSheetTheme(
                            data: MechanixBottomSheetThemeData(
                              backgroundColor:
                                  WidgetStateProperty.all(Colors.transparent),
                              borderRadius: 50,
                              shadowColor:
                                  WidgetStateProperty.all(Colors.black45),
                            ),
                            child: IconButton(
                              icon: Image.asset(Images.dots),
                              onPressed: () {
                                handleSelectionMore(context, state);
                              },
                              highlightColor: Colors.transparent,
                            ),
                          );
                        },
                      ),
                    ],
            ),
            body: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const SizedBox(height: 6),
                Expanded(
                  child: ContainerWidget(
                    child: ValueListenableBuilder<bool>(
                      valueListenable: viewModeNotifier,
                      builder: (context, isGrid, _) {
                        return isGrid
                            ? widget.title == 'recent'
                                ? buildGridViewForRecentFiles(
                                    context, fileSystemList)
                                : buildGridView(
                                    displayedFiles, context, widget.path)
                            : widget.title == 'recent'
                                ? buildListViewForRecentFiles(
                                    context, fileSystemList)
                                : buildListView(
                                    displayedFiles, context, widget.path);
                      },
                    ),
                  ),
                ),
              ],
            ),
            backgroundColor: Colors.black,
            floatingActionButtonLocation:
                FloatingActionButtonLocation.centerFloat,
            floatingActionButton:
                selectionMode ? _buildFloatingActionMenu(context) : null,
          );
        },
      ),
    );
  }

  Future<void> showSortMenu(BuildContext context, String selected) async {
    final isSizeSelected = selected.startsWith('size');
    final isDescending = selected == 'size_desc';
    final newSizeKey = isDescending ? 'size_asc' : 'size_desc';
    final sizeIcon =
        isDescending ? Images.sortDescending : Images.sortAscending;

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (ctx) {
        return Container(
          decoration: const BoxDecoration(
            color: Color.fromARGB(255, 70, 69, 69),
            borderRadius: BorderRadius.only(
              topLeft: Radius.circular(12),
              topRight: Radius.circular(12),
            ),
          ),
          width: double.infinity,
          child: MechanixMenu(
            backgroundColor: const Color.fromARGB(255, 70, 69, 69),
            itemPadding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
            items: [
              _buildSortMenuItem(
                context,
                key: 'none',
                label: 'None',
                isSelected: selected == 'none',
              ),
              MechanixMenuDivider(),
              _buildSortMenuItem(
                context,
                key: 'name',
                label: 'Name',
                isSelected: selected == 'name',
              ),
              MechanixMenuDivider(),
              _buildSortMenuItem(
                context,
                key: newSizeKey,
                label: 'Size',
                isSelected: isSizeSelected,
                trailingIcon: Image.asset(sizeIcon, width: 16, height: 16),
              ),
              MechanixMenuDivider(),
              _buildSortMenuItem(
                context,
                key: 'type',
                label: 'Type',
                isSelected: selected == 'type',
              ),
              MechanixMenuDivider(),
              _buildSortMenuItem(
                context,
                key: 'mod_time',
                label: 'File modified',
                isSelected: selected == 'mod_time',
              ),
              // Add more sort options here if needed
            ],
          ),
        );
      },
    );
  }

  MechanixMenuItem _buildSortMenuItem(
    BuildContext context, {
    required String key,
    required String label,
    required bool isSelected,
    Widget? trailingIcon,
  }) {
    return MechanixMenuItem(
      label: label,
      layout: MenuItemLayout.bothSides,
      leadingWidget: Icon(
        isSelected ? Icons.radio_button_checked : Icons.radio_button_unchecked,
        color: isSelected ? Colors.blue : Colors.grey,
        size: 18,
      ),
      trailingWidget: trailingIcon,
      onTap: () {
        handleSort(key);
        Navigator.of(context).pop();
      },
    );
  }

  Widget _buildFloatingActionMenu(BuildContext context) {
    final allSelected = selectedPaths.length == displayedFiles.length &&
        displayedFiles.isNotEmpty;

    return MechanixFloatingActionMenu(
      height: 48,
      backgroundColor: Colors.grey[800],
      items: [
        MechanixFabItem(
          iconWidget: Image.asset(Images.listChecks,
              color: allSelected ? Colors.blue : Colors.white, height: 20),
          onTap: () {
            onToggleSelectAll();
          },
        ),
        MechanixFabItem(
          iconWidget: Image.asset(
            Images.copy,
            height: 20,
          ),
          onTap: () {
            handleCopy();
          },
        ),
        MechanixFabItem(
          iconWidget: Image.asset(
            Images.delete,
            height: 20,
          ),
          onTap: () {
            handleDelete();
          },
        ),
        MechanixFabItem(
          anchorLink: _menuLink,
          iconWidget: Image.asset(
            Images.dots,
            color: _isMenuOpen ? Colors.grey : Colors.white,
            height: 20,
          ),
          onTap: () {
            if (_isMenuOpen) {
              _removeOverlay();
            } else {
              _toggleMenuOverlay(context);
            }
          },
        ),
        MechanixFabItem(
          iconWidget: Icon(
            Icons.close,
            color: Colors.white,
            size: 20,
          ),
          onTap: () {
            if (mounted) {
              final bloc = BlocProvider.of<FilesBloc>(context);
              if (isCopyMode) {
                bloc.add(CancelCopyMode());
              } else if (isMoveMode) {
                bloc.add(CancelMoveMode());
              } else if (isExtractMode) {
                bloc.add(CancelExtractMode());
              }
              clearSelection();
            }
          },
        ),
      ],
    ).padHorizontal(80);
  }

  void _toggleMenuOverlay(BuildContext context) {
    _menuEntry = OverlayEntry(
      builder: (_) => Positioned.fill(
        child: Stack(
          children: [
            GestureDetector(
              behavior: HitTestBehavior.translucent,
              onTap: () {
                _hideMenu();
              },
            ),
            CompositedTransformFollower(
              link: _menuLink,
              showWhenUnlinked: false,
              targetAnchor: Alignment.topRight,
              followerAnchor: Alignment.bottomRight,
              offset: const Offset(0, -4),
              child: Material(
                color: Colors.transparent,
                child: Theme(
                  data: Theme.of(context).copyWith(
                    extensions: [
                      const MechanixMenuThemeData(
                        backgroundColor:
                            WidgetStatePropertyAll(Color(0xFF424242)),
                        borderRadius: 12,
                        shadowColor: WidgetStatePropertyAll(Colors.black54),
                      ),
                      const MechanixMenuItemThemeData(
                        iconColor: WidgetStatePropertyAll(Colors.white70),
                        textStyle: TextStyle(color: Colors.white, fontSize: 14),
                      ),
                    ],
                  ),
                  child: MechanixMenu(
                    itemPadding:
                        const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
                    items: [
                      if (selectedPaths.length == 1)
                        MechanixMenuItem(
                          leadingWidget: Image.asset(Images.rename,
                              color: Colors.white70, height: 20),
                          label: "Rename",
                          onTap: () {
                            _hideMenu();
                            final selectedPath = selectedPaths.first;
                            _showRenameDialog(selectedPath);
                          },
                        ),
                      MechanixMenuDivider(),
                      MechanixMenuItem(
                        leadingWidget: Image.asset(Images.move,
                            color: Colors.white70, height: 20),
                        label: "Move",
                        onTap: () {
                          _hideMenu();
                          handleMove();
                        },
                      ),
                      MechanixMenuDivider(),
                      MechanixMenuItem(
                        leadingWidget: Image.asset(Images.createFolder,
                            color: Colors.white70, height: 20),
                        label: "Create folder",
                        onTap: () {
                          _hideMenu();
                          showCreateFolderDialog();
                          clearSelection();
                        },
                      ),
                      MechanixMenuDivider(),
                      MechanixMenuItem(
                        leadingWidget: Image.asset(Images.compress,
                            color: Colors.white70, height: 20),
                        label: "Compress",
                        onTap: () {
                          _hideMenu();
                          handleCompress();
                        },
                      ),
                      MechanixMenuDivider(),
                      MechanixMenuItem(
                        leadingWidget: Image.asset(Images.info,
                            color: Colors.white70, height: 20),
                        label: "Properties",
                        onTap: () {
                          _hideMenu();
                          handleProperties();
                        },
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );

    Overlay.of(context, rootOverlay: true).insert(_menuEntry!);
    setState(() {
      _isMenuOpen = true;
    });
  }

  void _hideMenu() {
    _menuEntry?.remove();
    _menuEntry = null;
    setState(() {
      _isMenuOpen = false;
    });
  }

  void _removeOverlay() {
    setState(() {
      _isMenuOpen = false;
    });
  }

  void onToggleSelectAll() {
    setState(() {
      if (selectedPaths.length == displayedFiles.length) {
        selectedPaths.clear();
      } else {
        selectedPaths = {
          for (final file in displayedFiles)
            '/${[...widget.path.map((e) => e.name), file.name].join('/')}'
        };
      }
      selectionMode = true;
    });
  }

  bool isSelected(String path) => selectedPaths.contains(path);

  void toggleSelection(String path) {
    setState(() {
      if (selectedPaths.contains(path)) {
        selectedPaths.remove(path);
      } else {
        selectedPaths.add(path);
      }
      // Keep selectionMode true as long as user initiated it
      selectionMode = true;
    });
  }

  void clearSelection() {
    setState(() {
      selectionMode = false;
      selectedPaths.clear();
    });
  }

  void enableSelect() {
    setState(() {
      selectionMode = true;
      selectedPaths.clear();
    });
  }

  void handleMove() {
    final currentPath = '/${widget.path.map((e) => e.name).join('/')}';
    List<String> selectedPathsList = selectedPaths.toList();
    final filesBloc = BlocProvider.of<FilesBloc>(context); // get bloc

    // Start move mode
    filesBloc.add(StartMoveMode(selectedPathsList));

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
              Text(
                'Select destination',
                style: const TextStyle(
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
                            () => reload(currentPath, filesBloc)),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.home,
                          iconColor: Colors.blueAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Downloads",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, downloadsDir, "Downloads",
                            filesBloc, () => reload(currentPath, filesBloc)),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.downloads,
                          iconColor: Colors.deepPurpleAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Documents",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, documentsDir, "Documents",
                            filesBloc, () => reload(currentPath, filesBloc)),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.homeDocuments,
                          iconColor: Colors.orangeAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Root (/)",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, "/", "Root", filesBloc,
                            () => reload(currentPath, filesBloc)),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.hardDrive,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
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
    ).whenComplete(() {
      // Clear selection only if needed when bottom sheet is closed
      clearSelection();
    });
  }

  void handleDelete() {
    _confirmDelete(context, selectedPaths);
    clearSelection();
  }

  void handleCopy() {
    BlocProvider.of<FilesBloc>(context)
        .add(StartCopyMode(selectedPaths.toList()));
    clearSelection();
  }

  void handleCompress() async {
    if (selectedPaths.isEmpty) return;

    final destinationDirPath = p.dirname(selectedPaths.first);
    final defaultZipName = "Archive.zip";

    String zipName = defaultZipName;

    final filesBloc = BlocProvider.of<FilesBloc>(context);

    await showModalBottomSheet(
      context: context,
      backgroundColor: Colors.black,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return Padding(
          padding: EdgeInsets.only(
            bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
          ),
          child: Container(
            padding: const EdgeInsets.all(20),
            decoration: const BoxDecoration(
              color: Colors.transparent,
              borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
            ),
            child: MechanixTextInputTheme(
              style: MechanixTextInputThemeData(
                borderRadius: BorderRadius.circular(50),
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Compress to...',
                    style: const TextStyle(
                      fontSize: 16,
                      color: Colors.grey,
                    ),
                  ).padBottom(12),

                  // Input field
                  Row(
                    children: [
                      Expanded(
                        flex: 3,
                        child: MechanixTextInput<String>.textInput(
                          hintText: defaultZipName,
                          initialValue: defaultZipName,
                          onChanged: (value) => zipName = value,
                          inputDecoration: InputDecoration(
                            contentPadding: const EdgeInsets.symmetric(
                                horizontal: 16, vertical: 16),
                            filled: true,
                            fillColor: const Color(0xFF2C2C2E),
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(50),
                              borderSide: BorderSide.none,
                            ),
                            suffixIcon: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                // Cancel button
                                OutlinedButton(
                                  style: OutlinedButton.styleFrom(
                                    padding: const EdgeInsets.symmetric(
                                        vertical: 16),
                                  ).copyWith(
                                    splashFactory: NoSplash
                                        .splashFactory, // Disable ripple animation
                                  ),
                                  onPressed: () =>
                                      Navigator.pop(bottomSheetContext),
                                  child: const Icon(
                                    Icons.close,
                                    color: Colors.white,
                                  ),
                                ),
                                const SizedBox(width: 6),
                                // OK / Compress button
                                ElevatedButton(
                                  style: ElevatedButton.styleFrom(
                                    padding: const EdgeInsets.symmetric(
                                        vertical: 16),
                                    backgroundColor: Colors.grey[800],
                                    foregroundColor: Colors.white,
                                    shape: RoundedRectangleBorder(
                                      borderRadius: BorderRadius.circular(50),
                                    ),
                                  ).copyWith(
                                    splashFactory: NoSplash
                                        .splashFactory, // Disable ripple animation
                                  ),
                                  onPressed: () {
                                    final trimmedName = zipName.trim();
                                    if (trimmedName.isNotEmpty) {
                                      final zipPath = p.join(
                                        destinationDirPath,
                                        trimmedName.endsWith('.zip')
                                            ? trimmedName
                                            : '$trimmedName.zip',
                                      );

                                      filesBloc.add(
                                        CompressEntitiesEvent(
                                          sourcePaths: selectedPaths.toList(),
                                          destinationZipPath: zipPath,
                                        ),
                                      );
                                    }
                                    Navigator.pop(bottomSheetContext);
                                    clearSelection();
                                  },
                                  child: const Icon(Icons.check),
                                ),
                                const SizedBox(width: 6),
                              ],
                            ),
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
  }

  void handleExtract() {
    BlocProvider.of<FilesBloc>(context)
        .add(StartExtractMode(selectedPaths.single));
    clearSelection();
  }

  void handleProperties() {
    if (selectedPaths.length == 1) {
      final selectedPath = selectedPaths.first;
      _showDetailsDialog(context, selectedPath);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text("Select a single item to view details",
              style: TextStyle(color: Colors.white)),
          duration: const Duration(seconds: 1),
          backgroundColor: Colors.grey[800],
        ),
      );
    }
  }

  void handleSelectionMore(BuildContext context, FilesState state) {
    final currentPath = '/${widget.path.map((e) => e.name).join('/')}';
    selectedPaths.add(currentPath);

    bool actionTaken = false; // Track if any menu item was selected

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (ctx) {
        return Container(
          decoration: const BoxDecoration(
            color: Color.fromARGB(255, 70, 69, 69),
            borderRadius: BorderRadius.only(
              topLeft: Radius.circular(12),
              topRight: Radius.circular(12),
            ),
          ),
          width: double.infinity,
          child: MechanixMenu(
            backgroundColor: const Color.fromARGB(255, 70, 69, 69),
            itemPadding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
            items: [
              MechanixMenuItem(
                label: "Select",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: Image.asset(Images.listChecks,
                    color: Colors.white70, height: mechanixIconSize),
                onTap: () {
                  actionTaken = true;
                  clearSelection();
                  enableSelect();
                  Navigator.of(context).pop();
                },
              ),
              MechanixMenuDivider(),
              MechanixMenuItem(
                label: "New Folder",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: Image.asset(Images.createFolder,
                    color: Colors.white70, height: mechanixIconSize),
                onTap: () {
                  actionTaken = true;
                  Navigator.of(context).pop();
                  showCreateFolderDialog();
                  clearSelection();
                },
              ),
              MechanixMenuDivider(),
              MechanixMenuItem(
                label: "Refresh",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: Image.asset(Images.refresh,
                    color: Colors.white70, height: mechanixIconSize),
                onTap: () {
                  actionTaken = true;
                  final filesBloc = BlocProvider.of<FilesBloc>(context);

                  reload(currentPath, filesBloc);
                  Navigator.of(context).pop();
                  clearSelection();
                },
              ),
              MechanixMenuDivider(),
              MechanixMenuItem(
                label:
                    showHiddenFiles ? "Hide Hidden Files" : "Show Hidden Files",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: Image.asset(
                  showHiddenFiles ? Images.eye : Images.eyeSlash,
                  color: Colors.white70,
                  height: mechanixIconSize,
                ),
                onTap: () {
                  actionTaken = true;
                  _toggleHiddenFiles();
                  Navigator.of(context).pop();
                  clearSelection();
                },
              ),
              MechanixMenuDivider(),
              MechanixMenuItem(
                label: "Copy",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: Image.asset(Images.copy,
                    color: Colors.white70, height: mechanixIconSize),
                onTap: () {
                  actionTaken = true;
                  handleCopy();
                  Navigator.of(context).pop();
                  clearSelection();
                },
              ),
              MechanixMenuDivider(),
              MechanixMenuItem(
                label: "Copy Path",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: const Icon(Icons.copy_all,
                    color: Colors.white70, size: mechanixIconSize),
                onTap: () {
                  actionTaken = true;
                  copyPath(currentPath);
                  Navigator.of(context).pop();
                  clearSelection();
                },
              ),
              MechanixMenuDivider(),
              if (isCopyMode || isMoveMode) ...[
                MechanixMenuItem(
                  label: "Paste",
                  layout: MenuItemLayout.iconLeft,
                  leadingWidget: const Icon(Icons.paste, color: Colors.white70),
                  onTap: () {
                    actionTaken = true;
                    handlePaste(context, state);
                    clearSelection();
                    Navigator.of(context).pop();
                  },
                ),
                MechanixMenuDivider(),
              ],
              MechanixMenuItem(
                label: "Open in Terminal",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: Image.asset(Images.terminal,
                    color: Colors.white70, height: mechanixIconSize),
                onTap: () {
                  actionTaken = true;
                  openInTerminal(currentPath);
                  Navigator.of(context).pop();
                  clearSelection();
                },
              ),
              MechanixMenuDivider(),
              MechanixMenuItem(
                label: "Properties",
                layout: MenuItemLayout.iconLeft,
                leadingWidget: Image.asset(Images.info,
                    color: Colors.white70, height: mechanixIconSize),
                onTap: () {
                  actionTaken = true;
                  Navigator.of(context).pop();
                  if (selectedPaths.length == 1) {
                    _showDetailsDialog(context, selectedPaths.first);
                  }
                  clearSelection();
                },
              ),
            ],
          ),
        );
      },
    ).whenComplete(() {
      if (!actionTaken) {
        // Bottom sheet closed without selecting any option
        clearSelection();
      }
    });
  }

  /// Opens a terminal window in the given path.
  /// Tries multiple terminal emulators (gnome-terminal, konsole, xfce4-terminal, xterm).
  Future<void> openInTerminal(String path) async {
    final candidates = [
      ['gnome-terminal', '--working-directory=$path'],
      ['konsole', '--workdir', path],
      ['xfce4-terminal', '--working-directory', path],
      ['xterm', '-e', 'cd $path; bash'],
    ];

    for (final cmd in candidates) {
      final executable = cmd.first;
      if (await Process.run('which', [executable])
          .then((p) => p.exitCode == 0)) {
        Process.start(executable, cmd.skip(1).toList());
        return;
      }
    }

    throw Exception("No terminal found");
  }

  // TODO: Test same location paste
  void handlePaste(BuildContext context, FilesState state) {
    final targetPath = '/${widget.path.map((e) => e.name).join('/')}';
    final bloc = BlocProvider.of<FilesBloc>(context);

    if (isCopyMode) {
      bloc.add(Copy(
        sourcePaths: state.copiedPaths,
        destinationPath: targetPath,
      ));
      bloc.add(CancelCopyMode());
    }

    if (isMoveMode) {
      bloc.add(Move(
        sourcePaths: state.movedPaths,
        destinationPath: targetPath,
      ));
      bloc.add(CancelMoveMode());
    }
  }

  void handleSort(String value) {
    BlocProvider.of<FilesBloc>(context).add(SortFiles(value));
  }

  void _showDetailsDialog(BuildContext context, String path) {
    final bloc = context.read<FilesBloc>();
    bloc.add(FetchFileDetails(path));

    final fileItem = FileItem(
      name: p.basename(path),
      type: p.extension(path) == '' ? 'dir' : p.extension(path),
      modified: DateTime.now(),
    );
    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return BlocProvider.value(
          value: bloc,
          child: BlocBuilder<FilesBloc, FilesState>(
            builder: (context, state) {
              final details = state.fileDetails;
              final hidden = p.basename(path).startsWith('.') ? 'Yes' : 'No';
              var readable = '-';
              var writable = '-';

              if (details != null) {
                final mode = details.mode;
                readable = (mode & 0x100) != 0 ? 'Yes' : 'No';
                writable = (mode & 0x80) != 0 ? 'Yes' : 'No';
              }

              if (details == null) {
                return Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: Colors.grey[800],
                    borderRadius:
                        const BorderRadius.vertical(top: Radius.circular(12)),
                  ),
                  child: const SizedBox(
                    height: 100,
                    child: Center(child: CircularProgressIndicator()),
                  ),
                );
              }

              final items = [
                buildDetailRow("Type", details.type.toString()),
                buildDetailRow("Size", formatBytes(details.size)),
                buildDetailRow("Modified", formatDateTime(details.modified)),
                buildDetailRow("Accessed", formatDateTime(details.accessed)),
                buildDetailRow("Changed", formatDateTime(details.changed)),
                buildDetailRow("Readable", readable),
                buildDetailRow("Writable", writable),
                buildDetailRow("Hidden", hidden),
              ];

              return Container(
                padding:
                    const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
                decoration: BoxDecoration(
                  color: Colors.grey[800],
                  borderRadius:
                      const BorderRadius.vertical(top: Radius.circular(12)),
                ),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(children: [
                      Image.asset(fileItem.iconPath, width: 28, height: 28),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(
                          fileItem.name,
                          style: const TextStyle(
                            fontWeight: FontWeight.bold,
                            fontSize: 20,
                            color: Colors.white,
                          ),
                          overflow: TextOverflow.ellipsis,
                          maxLines: 1,
                        ),
                      )
                    ]),
                    const SizedBox(height: 12),
                    MechanixSimpleListTheme(
                      style: MechanixSimpleListThemeData(
                        itemPadding: EdgeInsets.zero,
                        backgroundColor: Colors.transparent,
                        widgetMargin: EdgeInsets.only(bottom: 8),
                      ),
                      child: MechanixSimpleList.builder(
                          isDividerRequired: false,
                          padding: EdgeInsets.zero,
                          itemCount: items.length,
                          itemBuilder: (context, index) {
                            return items[index];
                          }),
                    ),
                    SizedBox(
                      width: double.infinity,
                      child: ElevatedButton(
                        style: ElevatedButton.styleFrom(
                          backgroundColor: Colors.grey[700],
                          foregroundColor: Colors.white,
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(50),
                          ),
                        ).copyWith(
                          splashFactory: NoSplash
                              .splashFactory, // Disable ripple animation
                        ),
                        onPressed: () => Navigator.of(bottomSheetContext).pop(),
                        child: const Text("Close"),
                      ),
                    ),
                  ],
                ),
              );
            },
          ),
        );
      },
    );
  }

  Widget buildDetailRow(String title, String value) {
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(title,
              style: const TextStyle(color: Colors.white, fontSize: 14)),
          Text(value,
              style: const TextStyle(color: Colors.white70, fontSize: 12)),
        ],
      ),
    );
  }

  void showCreateFolderDialog() {
    String folderName = ""; // Local variable to track input
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return Padding(
          padding: EdgeInsets.only(
            bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
          ),
          child: Container(
            padding: const EdgeInsets.all(20),
            decoration: const BoxDecoration(
              color: Colors.transparent,
              borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
            ),
            child: MechanixTextInputTheme(
              style: MechanixTextInputThemeData(
                borderRadius: BorderRadius.circular(50),
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Expanded(
                        flex: 3,
                        child: MechanixTextInput<String>.textInput(
                          hintText: "New folder",
                          onChanged: (value) {
                            folderName = value; // Update local variable
                          },
                          inputDecoration: InputDecoration(
                            contentPadding: const EdgeInsets.symmetric(
                                horizontal: 16, vertical: 16),
                            filled: true,
                            fillColor: const Color(0xFF2C2C2E),
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(50),
                              borderSide: BorderSide.none,
                            ),
                            suffixIcon: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                OutlinedButton(
                                  style: OutlinedButton.styleFrom(
                                          padding: EdgeInsets.symmetric(
                                              vertical: 16))
                                      .copyWith(
                                    splashFactory: NoSplash
                                        .splashFactory, // Disable ripple animation
                                  ),
                                  onPressed: () {
                                    Navigator.of(bottomSheetContext).pop();
                                  },
                                  child: Icon(
                                    Icons.close,
                                    color: Colors.white,
                                  ),
                                ),
                                const SizedBox(width: 6),
                                ElevatedButton(
                                  style: ElevatedButton.styleFrom(
                                    padding: EdgeInsets.symmetric(vertical: 16),
                                    backgroundColor: Colors.grey[800],
                                    foregroundColor: Colors.white,
                                    shape: RoundedRectangleBorder(
                                      borderRadius: BorderRadius.circular(50),
                                    ),
                                  ).copyWith(
                                    splashFactory: NoSplash
                                        .splashFactory, // Disable ripple animation
                                  ),
                                  onPressed: () {
                                    final trimmedName = folderName.trim();
                                    if (trimmedName.isNotEmpty) {
                                      final currentPath =
                                          '/${widget.path.map((e) => e.name).join('/')}';
                                      filesBloc.add(CreateFolder(
                                        path: currentPath,
                                        folderName: trimmedName,
                                      ));
                                    }
                                    Navigator.of(bottomSheetContext).pop();
                                  },
                                  child: Icon(Icons.check),
                                ),
                                const SizedBox(width: 6),
                              ],
                            ),
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
  }

  void _showRenameDialog(String oldPath) {
    String newName = p.basename(oldPath); // Local variable to track input
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.black,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return Padding(
          padding: EdgeInsets.only(
            bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
          ),
          child: Container(
            padding: const EdgeInsets.all(20),
            decoration: const BoxDecoration(
              color: Colors.transparent,
              borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
            ),
            child: MechanixTextInputTheme(
              style: MechanixTextInputThemeData(
                borderRadius: BorderRadius.circular(50),
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Rename \'${p.basename(oldPath)}\'',
                    style: const TextStyle(
                      fontSize: 16,
                      color: Colors.grey,
                    ),
                  ).padBottom(12),

                  // Input field
                  Row(
                    children: [
                      Expanded(
                        flex: 3,
                        child: MechanixTextInput<String>.textInput(
                          hintText: p.basename(oldPath),
                          initialValue: p.basename(oldPath),
                          onChanged: (value) {
                            newName = value;
                          },
                          inputDecoration: InputDecoration(
                            contentPadding: const EdgeInsets.symmetric(
                                horizontal: 16, vertical: 16),
                            filled: true,
                            fillColor: const Color(0xFF2C2C2E),
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(50),
                              borderSide: BorderSide.none,
                            ),
                            suffixIcon: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                OutlinedButton(
                                  style: OutlinedButton.styleFrom(
                                          padding: const EdgeInsets.symmetric(
                                              vertical: 16))
                                      .copyWith(
                                    splashFactory: NoSplash
                                        .splashFactory, // Disable ripple animation
                                  ),
                                  onPressed: () {
                                    Navigator.of(bottomSheetContext).pop();
                                  },
                                  child: const Icon(
                                    Icons.close,
                                    color: Colors.white,
                                  ),
                                ),
                                const SizedBox(width: 6),
                                ElevatedButton(
                                  style: ElevatedButton.styleFrom(
                                    padding: const EdgeInsets.symmetric(
                                        vertical: 16),
                                    backgroundColor: Colors.grey[800],
                                    foregroundColor: Colors.white,
                                    shape: RoundedRectangleBorder(
                                      borderRadius: BorderRadius.circular(50),
                                    ),
                                  ).copyWith(
                                    splashFactory: NoSplash
                                        .splashFactory, // Disable ripple animation
                                  ),
                                  onPressed: () {
                                    final trimmedName = newName.trim();
                                    if (trimmedName.isNotEmpty &&
                                        trimmedName != p.basename(oldPath)) {
                                      filesBloc.add(Rename(
                                        oldPath: oldPath,
                                        newName: trimmedName,
                                      ));
                                    }
                                    Navigator.of(bottomSheetContext).pop();
                                    clearSelection();
                                  },
                                  child: const Icon(Icons.check),
                                ),
                                const SizedBox(width: 6),
                              ],
                            ),
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
    clearSelection();
  }

  void _toggleHiddenFiles() {
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    final currentPath = '/${widget.path.map((e) => e.name).join('/')}';

    filesBloc.add(ToggleHiddenFiles(path: currentPath));
  }

  void reload(String currentPath, FilesBloc filesBloc) {
    filesBloc.add(LoadFilesAtPath(currentPath));
  }

  Future<void> copyPath(String path) async {
    await Clipboard.setData(ClipboardData(text: path));
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Path copied to clipboard',
            style: TextStyle(color: Colors.white)),
        duration: const Duration(seconds: 2),
        backgroundColor: Colors.grey[800],
      ),
    );
  }
}

List<FileItem> getFilesAtPath(
    List<FileItem> path, List<FileSystemEntity> fileSystemList) {
  // Build full path from root and path list
  String currentPath = '/';
  for (final item in path) {
    currentPath = p.join(currentPath, item.name);
  }

  final List<FileItem> items = [];

  try {
    for (final entity in fileSystemList) {
      final String name = p.basename(entity.path);
      if (name.isEmpty) continue;

      final stat = entity.statSync();
      final modifiedTime = stat.modified;

      if (entity is Directory) {
        items.add(FileItem(name: name, type: 'dir', modified: modifiedTime));
      } else if (entity is File) {
        final ext = p.extension(name);
        items.add(FileItem(
            name: name,
            type: ext.isNotEmpty ? ext : 'file',
            modified: modifiedTime));
      }
    }
  } catch (e) {
    print('Error reading directory at $currentPath: $e');
  }

  return items;
}

void _confirmDelete(BuildContext context, Set<String> selectedPaths) {
  final pathsToDelete = selectedPaths.toList();

  final isSingle = pathsToDelete.length == 1;
  final title = isSingle
      ? "Delete '${pathsToDelete.first.split('/').last}'"
      : "Delete ${pathsToDelete.length} selected files";
  final message = isSingle
      ? "This action will delete the file permanently"
      : "This action will delete the files permanently";

  showModalBottomSheet(
    context: context,
    backgroundColor: Colors.transparent,
    isScrollControlled: true,
    builder: (bottomSheetContext) => Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.grey[850],
        borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: const TextStyle(
              fontWeight: FontWeight.w500,
              fontSize: 18,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            message,
            style: const TextStyle(color: Colors.white70),
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
                  onPressed: () => Navigator.pop(bottomSheetContext),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: MechanixElevatedButton(
                  label: "Delete",
                  backgroundColor: Colors.red,
                  textColor: Colors.white,
                  borderRadius: 50,
                  onPressed: () {
                    Navigator.pop(bottomSheetContext);
                    BlocProvider.of<FilesBloc>(context)
                        .add(DeleteEntities(pathsToDelete));
                  },
                ),
              ),
            ],
          ),
        ],
      ),
    ),
  );
}

String formatBytes(int bytes, [int decimals = 2]) {
  if (bytes <= 0) return "0 B";
  const suffixes = ["B", "KB", "MB", "GB", "TB"];
  final i = (bytes == 0) ? 0 : (Math.log(bytes) / Math.log(1024)).floor();
  final size = bytes / Math.pow(1024, i);
  return "${size.toStringAsFixed(decimals)} ${suffixes[i]}";
}
