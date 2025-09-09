import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_button.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_loading_dialog.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_popup_menu_item.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:mechanix_files/src/features/files/presentation/move_file_dialog.dart';
import 'view_mode_notifier.dart';
import 'navigations.dart';
import 'grid_view.dart';
import 'list_view.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:file/file.dart';
import 'package:path/path.dart' as p;
import 'dart:math' as Math;

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

  final GlobalKey _menuIconKey = GlobalKey();
  bool _isMenuOpen = false;

  @override
  Widget build(BuildContext context) {
    final isAtRoot = widget.path.isEmpty;
    final backIndex = widget.path.length - 2;
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
            appBar: AppBar(
              backgroundColor: Colors.black,
              titleSpacing: 0,
              leading: IconButton(
                icon: const Icon(
                  Icons.arrow_back_ios,
                  size: 20,
                  color: Colors.blue,
                ),
                onPressed: isAtRoot ? homeNavigation : handleBack,
              ),
              title: GestureDetector(
                onTap: isAtRoot ? homeNavigation : handleBack,
                child: Text(
                  currentTitle,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    fontSize: 18,
                    color: Colors.blue,
                  ),
                ),
              ),
              actions: selectionMode
                  ? null
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
                          );
                        },
                      ),
                      IconButton(
                        icon: Image.asset(Images.sortAscending),
                        onPressed: () async {
                          showSortMenu(context, state.currentSortBy);
                        },
                      ),
                      IconButton(
                        icon: const Icon(Icons.search, color: Colors.white),
                        onPressed: () {
                          // Implement search logic
                        },
                      ),
                      IconButton(
                        icon: Image.asset(Images.dots),
                        onPressed: () {
                          handleSelectionMore(context, state);
                        },
                      ),
                    ],
            ),
            body: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                buildNavigations(context, widget.path,
                    selectedCount: selectedPaths.length),
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
    final RenderBox overlay =
        Overlay.of(context).context.findRenderObject() as RenderBox;

    final isSizeSelected = selected.startsWith('size');
    final isDescending = selected == 'size_desc';
    final newSizeKey = isDescending ? 'size_asc' : 'size_desc';
    final sizeIcon =
        isDescending ? Images.sortDescending : Images.sortAscending;

    final sortValue = await showMenu<String>(
      context: context,
      position: RelativeRect.fromLTRB(
        overlay.size.width - 48,
        kToolbarHeight + 8,
        16,
        0,
      ),
      // Custom theme for popup menu
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
      color: Colors.grey[800],
      shadowColor: Colors.black45,
      elevation: 8,
      items: [
        _buildSortItem('none', 'None', selected == 'none'),
        const PopupMenuDivider(height: 1),
        _buildSortItem('name', 'Name', selected == 'name'),
        const PopupMenuDivider(height: 1),
        _buildSortItem(
          newSizeKey,
          'Size',
          isSizeSelected,
          trailingIcon: Image.asset(sizeIcon, width: 16, height: 16),
        ),
        const PopupMenuDivider(height: 1),
        _buildSortItem('type', 'Type', selected == 'type'),
        const PopupMenuDivider(height: 1),
        _buildSortItem('mod_time', 'File modified', selected == 'mod_time'),
        //  _buildSortItem('created', 'File created', selected == 'created'),
      ],
    );

    if (sortValue != null) {
      handleSort(sortValue);
    }
  }

  PopupMenuItem<String> _buildSortItem(
    String key,
    String label,
    bool isSelected, {
    Widget? trailingIcon,
    void Function()? onTap,
  }) {
    return PopupMenuItem<String>(
      value: key,
      onTap: onTap,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          SizedBox(
            width: 24,
            child: isSelected
                ? Icon(Icons.radio_button_checked, size: 18, color: Colors.blue)
                : Icon(Icons.radio_button_unchecked,
                    size: 18, color: Colors.grey),
          ),
          const SizedBox(width: 8),
          Expanded(child: Text(label)),
          if (trailingIcon != null) trailingIcon,
        ],
      ),
    );
  }

  Widget _buildFloatingActionMenu(BuildContext context) {
    final allSelected = selectedPaths.length == displayedFiles.length &&
        displayedFiles.isNotEmpty;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24),
      child: Container(
        decoration: BoxDecoration(
          color: Colors.grey[800],
          borderRadius: BorderRadius.circular(12),
          boxShadow: [
            BoxShadow(
              color: Colors.black26,
              blurRadius: 10,
              offset: Offset(0, -2),
            ),
          ],
        ),
        padding: const EdgeInsets.symmetric(horizontal: 12),
        height: 50,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            IconButton(
              icon: Image.asset(
                Images.listChecks,
                height: 28,
                width: 28,
                color: allSelected ? Colors.blue : Colors.white,
              ),
              onPressed: () {
                onToggleSelectAll();
              },
            ),
            IconButton(
              icon: Image.asset(
                Images.copy,
                height: 24,
                width: 24,
                color: Colors.white,
              ),
              onPressed: () {
                handleCopy();
              },
            ),
            IconButton(
              icon: Image.asset(
                Images.delete,
                height: 24,
                width: 24,
                color: Colors.white,
              ),
              onPressed: () {
                handleDelete();
              },
            ),
            _buildMenuIconButton(context), // <<< Show popup menu
            IconButton(
              icon: Image.asset(
                Images.xCircle,
                height: 28,
                width: 28,
                color: Colors.white,
              ),
              onPressed: () {
                // Notify parent to turn off selection mode
                if (mounted) {
                  final bloc = BlocProvider.of<FilesBloc>(context);
                  if (isCopyMode) {
                    bloc.add(CancelCopyMode());
                  } else if (isMoveMode) {
                    bloc.add(CancelMoveMode());
                  } else if (isExtractMode) {
                    bloc.add(CancelExtractMode());
                  }

                  clearSelection(); // Optional if wrapped in bottom sheet/modal
                }
              },
            ),
          ],
        ),
      ),
    );
  }

  void _toggleMenuOverlay(BuildContext context) async {
    setState(() {
      _isMenuOpen = true;
    });

    final RenderBox renderBox =
        _menuIconKey.currentContext!.findRenderObject() as RenderBox;
    final Offset offset = renderBox.localToGlobal(Offset.zero);
    final selectedPath = selectedPaths.first;

    final Size iconSize = renderBox.size;

    final double menuHeight = 220;

    final selected = await showMenu<String>(
      context: context,
      position: RelativeRect.fromLTRB(
        offset.dx - 120,
        offset.dy - menuHeight,
        offset.dx + iconSize.width,
        MediaQuery.of(context).size.height - offset.dy + iconSize.height,
      ),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      color: Colors.grey[800],
      shadowColor: Colors.black45,
      elevation: 8,
      items: [
        buildStyledMenuItem(
            'rename', 'Rename', Images.rename, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem(
            'move', 'Move', Images.move, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem('create_folder', 'Create folder',
            Images.createFolder, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem('compress', 'Compress', Images.compress,
            Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem('properties', 'Properties', Images.info,
            Colors.white, Colors.white),
      ],
    );

    if (selected == null) {
      _removeOverlay();
      return;
    }

    switch (selected) {
      case 'rename':
        _removeOverlay();
        _showRenameDialog(selectedPath);
        break;
      case 'move':
        _removeOverlay();
        handleMove();
        break;
      case 'create_folder':
        _removeOverlay();
        showCreateFolderDialog();
        break;
      case 'compress':
        _removeOverlay();
        handleCompress();
        break;
      case 'properties':
        _removeOverlay();
        handleProperties();
        break;
    }
  }

  void _removeOverlay() {
    setState(() {
      _isMenuOpen = false;
    });
  }

  Widget _buildMenuIconButton(BuildContext context) {
    return IconButton(
      key: _menuIconKey,
      icon: Image.asset(
        Images.dots,
        height: 28,
        width: 28,
        color:
            _isMenuOpen ? Colors.grey : Colors.white, // Change color when open
      ),
      onPressed: () => _toggleMenuOverlay(context),
    );
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
    final filesBloc = BlocProvider.of<FilesBloc>(context); // <- outside builder
    //final state = context.findAncestorStateOfType<FileExplorerPageState>();

    BlocProvider.of<FilesBloc>(context)
        .add(StartMoveMode(selectedPaths.toList()));
    showDialog(
      context: context,
      builder: (context) => MoveFileDialog(
        currentPath: currentPath,
        selectedPaths: selectedPathsList,
        filesBloc: filesBloc,
      ),
    );
    clearSelection();
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

    final TextEditingController controller = TextEditingController();
    final destinationDirPath = p.dirname(selectedPaths.first);
    final defaultZipName = "Archive.zip";

    final result = await showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text("Compress"),
        content: TextField(
          controller: controller..text = defaultZipName,
          decoration: const InputDecoration(
            labelText: "Compress to...",
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("Cancel"),
          ),
          ElevatedButton(
            onPressed: () {
              final zipName = controller.text.trim();
              if (zipName.isNotEmpty) {
                Navigator.pop(context, zipName);
              }
            },
            child: const Text("OK"),
          ),
        ],
      ),
    );

    if (result != null && result.isNotEmpty) {
      final zipPath = p.join(
          destinationDirPath, result.endsWith('.zip') ? result : '$result.zip');

      BlocProvider.of<FilesBloc>(context).add(
        CompressEntitiesEvent(
          sourcePaths: selectedPaths.toList(),
          destinationZipPath: zipPath,
        ),
      );

      clearSelection();
    }
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
        const SnackBar(
          content: Text("Select a single item to view details"),
        ),
      );
    }
  }

  void handleSelectionMore(BuildContext context, FilesState state) async {
    final currentPath = '/${widget.path.map((e) => e.name).join('/')}';
    selectedPaths.add(currentPath);
    final RenderBox overlay =
        Overlay.of(context).context.findRenderObject() as RenderBox;

    final selected = await showMenu<String>(
      context: context,
      position: RelativeRect.fromLTRB(
        overlay.size.width - 48,
        kToolbarHeight + 8,
        16,
        0,
      ),
      // Custom theme for popup menu
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
      color: Colors.grey[800],
      shadowColor: Colors.black45,
      elevation: 8,
      items: [
        buildStyledMenuItem(
            'select', 'Select', Images.listChecks, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem('create_folder', 'New Folder', Images.createFolder,
            Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem(
            'refresh', 'Refresh', Images.refresh, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem(
            'show_hidden',
            showHiddenFiles ? 'Hide Hidden Files' : 'Show Hidden Files',
            showHiddenFiles ? Images.eye : Images.eyeSlash,
            Colors.white,
            Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem(
            'copy', 'Copy', Images.copy, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem(
            'copy_path', 'Copy Path', Images.copy, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        if (isCopyMode || isMoveMode)
          buildStyledMenuItem(
              'paste', 'Paste', Images.paste, Colors.white, Colors.white),
        if (isCopyMode || isMoveMode) const PopupMenuDivider(height: 1),
        buildStyledMenuItem('open_in_terminal', 'Open In Terminal',
            Images.terminal, Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
        buildStyledMenuItem('properties', 'Properties', Images.info,
            Colors.white, Colors.white),
        const PopupMenuDivider(height: 1),
      ],
    );

    switch (selected) {
      case 'select':
        enableSelect();
        break;
      case 'copy':
        handleCopy();
        clearSelection();
        break;
      case 'paste':
        handlePaste(context, state);
        clearSelection();
        break;
      case 'create_folder':
        showCreateFolderDialog();
        clearSelection();
        break;
      case 'show_hidden':
        _toggleHiddenFiles();
        clearSelection();
        break;
      case 'properties':
        if (selectedPaths.length == 1) {
          _showDetailsDialog(context, selectedPaths.first);
        }
        clearSelection();
        break;
      case 'open_in_terminal':
        openInTerminal(currentPath);
        clearSelection();
        break;
      case 'refresh':
        reload(currentPath);
        clearSelection();
        break;
      case 'copy_path':
        copyPath(currentPath);
        clearSelection();
        break;
      default:
        clearSelection();
        break;
    }
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

    showDialog(
      context: context,
      builder: (dialogContext) {
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
                return const AlertDialog(
                  backgroundColor: Colors.grey,
                  title: Text("Properties"),
                  content: SizedBox(
                    height: 60,
                    child: Center(child: CircularProgressIndicator()),
                  ),
                );
              }

              final fileItem = FileItem(
                name: p.basename(path),
                type: p.extension(path) == '' ? 'dir' : p.extension(path),
                modified: DateTime.now(),
              );

              return AlertDialog(
                backgroundColor: Colors.grey[800],
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
                title: Text(
                  'Properties',
                  style: const TextStyle(
                      fontWeight: FontWeight.bold, fontSize: 18),
                ),
                content: SizedBox(
                  width: 400,
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Padding(
                        padding: const EdgeInsets.symmetric(vertical: 10),
                        child: Row(
                          children: [
                            Image.asset(fileItem.iconPath,
                                width: 32, height: 32),
                            const SizedBox(width: 12),
                            Expanded(
                              child: Text(
                                p.basename(path),
                                style: const TextStyle(
                                  fontSize: 16,
                                  color: Colors.white,
                                ),
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                          ],
                        ),
                      ),
                      _buildPropertyRow('Type', '${details.type}'),
                      _buildPropertyRow('Size', formatBytes(details.size)),
                      _buildPropertyRow(
                          'Modified', formatDateTime(details.modified)),
                      _buildPropertyRow(
                          'Accessed', formatDateTime(details.accessed)),
                      _buildPropertyRow(
                          'Changed', formatDateTime(details.changed)),
                      _buildPropertyRow('Readable', readable),
                      _buildPropertyRow('Writable', writable),
                      _buildPropertyRow('Hidden', hidden),
                    ],
                  ),
                ),
                actions: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      CustomButton(
                        label: "Close",
                        backgroundColor: Colors.grey[700]!,
                        textColor: Colors.white,
                        onPressed: () => Navigator.pop(context),
                      ),
                    ],
                  ),
                ],
              );
            },
          ),
        );
      },
    );
  }

  /// Helper widget to align label and value in two columns
  Widget _buildPropertyRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: const TextStyle(color: Colors.white70),
          ),
          const Spacer(),
          Text(
            value,
            style: const TextStyle(color: Colors.white),
          ),
        ],
      ),
    );
  }

  void showCreateFolderDialog() {
    final TextEditingController folderController = TextEditingController();
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text("Create Folder"),
        content: TextField(
          controller: folderController,
          decoration: const InputDecoration(hintText: "Enter folder name"),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("Cancel"),
          ),
          ElevatedButton(
            onPressed: () {
              final folderName = folderController.text.trim();
              if (folderName.isNotEmpty) {
                final currentPath =
                    '/${widget.path.map((e) => e.name).join('/')}';

                filesBloc.add(
                    CreateFolder(path: currentPath, folderName: folderName));
              }
              Navigator.pop(context);
            },
            child: const Text("OK"),
          ),
        ],
      ),
    );
  }

  void _showRenameDialog(String oldPath) {
    final TextEditingController renameController =
        TextEditingController(text: p.basename(oldPath));
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text("Rename"),
        content: TextField(
          controller: renameController,
          decoration: const InputDecoration(hintText: "Enter new name"),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("Cancel"),
          ),
          ElevatedButton(
            onPressed: () {
              final newName = renameController.text.trim();
              if (newName.isNotEmpty && newName != p.basename(oldPath)) {
                filesBloc.add(Rename(oldPath: oldPath, newName: newName));
              }
              Navigator.pop(context);
              clearSelection();
            },
            child: const Text("Rename"),
          ),
        ],
      ),
    );
  }

  void _toggleHiddenFiles() {
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    final currentPath = '/${widget.path.map((e) => e.name).join('/')}';

    filesBloc.add(ToggleHiddenFiles(path: currentPath));
  }

  void reload(String currentPath) {
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    filesBloc.add(LoadFilesAtPath(currentPath));
  }

  Future<void> copyPath(String path) async {
    await Clipboard.setData(ClipboardData(text: path));
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Path copied to clipboard'),
        duration: const Duration(seconds: 2),
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
      ? 'Delete \'${pathsToDelete.first.split('/').last}\' file'
      : 'Delete ${pathsToDelete.length} selected files';

  showDialog(
    context: context,
    builder: (_) => AlertDialog(
      backgroundColor: Colors.grey[800],
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
      title: Text(
        title,
        style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
      ),
      content: Text(
        isSingle
            ? "This action will delete the file permanently"
            : "This action will delete the files permanently",
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
                    onPressed: () => Navigator.pop(context),
                  ),
                  const SizedBox(width: 12),
                  CustomButton(
                    label: "Delete",
                    backgroundColor: Colors.red,
                    textColor: Colors.white,
                    onPressed: () {
                      Navigator.pop(context);
                      BlocProvider.of<FilesBloc>(context)
                          .add(DeleteEntities(pathsToDelete));
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

String formatBytes(int bytes, [int decimals = 2]) {
  if (bytes <= 0) return "0 B";
  const suffixes = ["B", "KB", "MB", "GB", "TB"];
  final i = (bytes == 0) ? 0 : (Math.log(bytes) / Math.log(1024)).floor();
  final size = bytes / Math.pow(1024, i);
  return "${size.toStringAsFixed(decimals)} ${suffixes[i]}";
}
