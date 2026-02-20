import 'dart:async';
import 'dart:io' as io;
import 'dart:io';

import 'package:ellipsized_text/ellipsized_text.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_loading_dialog.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/pressable_icon.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/conflict_resolution_bottomsheet.dart';
import 'package:mechanix_files/src/features/files/presentation/extract_file_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/file_details_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/move_file_dialog.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';
import 'package:widgets/widgets/notification/notification_type.dart';

import 'grid_view.dart';
import 'list_view.dart';
import 'view_mode_notifier.dart';

class FileExplorerPage extends StatefulWidget {
  final String title;
  final List<FileItem> path;
  final String? startPath;

  const FileExplorerPage({
    super.key,
    this.title = "Files",
    this.path = const [],
    this.startPath,
  });

  @override
  State<FileExplorerPage> createState() => FileExplorerPageState();
}

class FileExplorerPageState extends State<FileExplorerPage> {
  final FileManagerController controller = FileManagerController();
  List<FileItem> displayedFiles = [];
  bool selectionMode = false;
  Set<String> selectedPaths = {};
  bool showHiddenFiles = false;
  bool isLoadingDialogShown = false;
  bool isCopyMode = false;
  bool isMoveMode = false;
  bool isExtractMode = false;
  bool isHomePageDir = false;
  String zipFilePath = '';
  List<String> copiedPaths = [];
  List<String> movedPaths = [];

  bool isFolderActionMenuOpen = false;
  bool isSelectionActionMenuOpen = false;
  bool isSortMenuOpen = false;

  final downloadsDir = AppPaths.downloadsDir;
  final documentsDir = AppPaths.documentsDir;
  final homeDir = AppPaths.homeDir;
  final recentDir = AppPaths.recentDir;

  final ValueNotifier<String> searchQuery = ValueNotifier('');
  bool isSearching = false;

  /// For list/grid view
  final ScrollController _scrollController = ScrollController();
  String currentPath = '';

  final ValueNotifier<bool> allSelectedNotifier = ValueNotifier(false);

  bool isTextInputOpened = false;
  final FocusNode _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();

    _scrollController.addListener(_onScroll);

    // Default to home directory if no startPath is provided
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      if (!mounted) return;

      // Start from home
      await controller.openDirectory(Directory(homeDir));
      if (!mounted) return;

      final initialPath = widget.startPath;
      if (initialPath == null) return;

      final type = io.FileSystemEntity.typeSync(initialPath);

      if (type == FileSystemEntityType.file) {
        final file = File(initialPath);

        // Navigate into parent AFTER home exists
        await controller.openDirectory(file.parent);
        if (!mounted) return;

        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (!mounted) return;
          handleFileTap(
            context,
            file,
            initialPath,
            selectionMode,
            this,
            controller,
          );
        });
      } else {
        await controller.openDirectory(Directory(initialPath));
      }
    });

    controller.getPathNotifier.addListener(() {
      final newPath = controller.getPathNotifier.value;
      setState(() {
        currentPath = newPath;
      });
    });

    searchQuery.addListener(() {
      if (searchQuery.value.isEmpty) {
        controller.reload(); // Reload when user clears search
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
    _fabController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  Future<void> focusChange() async {
    if (!_focusNode.hasFocus) {
      await Future.delayed(const Duration(milliseconds: 300));
    }

    setState(() => isTextInputOpened = _focusNode.hasFocus);
  }

  final FloatingActionBarController _fabController =
      FloatingActionBarController();

  void _openFabMenuProgrammatically() {
    if (!mounted) return;
    _fabController.open();
  }

  void _closeFabMenuProgrammatically() {
    if (!mounted) return;
    _fabController.close();
  }

  void _toggleFabMenuProgrammatically() {
    _fabController.toggle();
  }

  final ValueNotifier<bool> hasSelectionNotifier = ValueNotifier(false);

  @override
  Widget build(BuildContext context) {
    final isAtRoot = currentPath == '/' || currentPath.isEmpty;

    final isDocumentsDir = currentPath == documentsDir;
    final isDownloadsDir = currentPath == downloadsDir;
    final isHomeDir = currentPath == homeDir;
    final isRecentDir = currentPath == recentDir;
    isHomePageDir =
        isHomeDir ||
        isDownloadsDir ||
        isDocumentsDir ||
        isAtRoot ||
        isRecentDir;

    return MultiBlocListener(
      listeners: [
        // Compression dialog handler
        BlocListener<FilesBloc, FilesState>(
          listenWhen:
              (previous, current) =>
                  previous.compressionStatus != current.compressionStatus,
          listener: (context, state) {
            if (state.compressionStatus == FileCompressionStatus.inProgress) {
              showDialog(
                context: context,
                barrierColor: context.colorScheme.surface.withOpacity(0.2),
                barrierDismissible: false,
                builder: (_) => buildLoadingDialog(context, "Compressing..."),
              );
            } else if (state.compressionStatus ==
                    FileCompressionStatus.success ||
                state.compressionStatus == FileCompressionStatus.failure) {
              Navigator.of(context, rootNavigator: true).pop();

              if (state.compressionStatus == FileCompressionStatus.failure) {
                MechanixNotification.show(
                  context: context,
                  notificationType: NotificationType.error,
                  message: "Compression failed: ${state.compressionError}",
                );
              }

              if (state.compressionStatus == FileCompressionStatus.success) {
                MechanixNotification.show(
                  context: context,
                  notificationType: NotificationType.success,
                  message: "Compression complete",
                );
              }
            }
          },
        ),
        // Global error message handler
        BlocListener<FilesBloc, FilesState>(
          listenWhen:
              (previous, current) =>
                  previous.error != current.error && current.error != null,
          listener: (context, state) {
            MechanixNotification.show(
              context: context,
              notificationType: NotificationType.error,
              message: "Error: ${state.error}",
            );
          },
        ),
        // Loading indicator for general loading states
        BlocListener<FilesBloc, FilesState>(
          listenWhen:
              (previous, current) => previous.loading != current.loading,
          listener: (context, state) async {
            if (state.loading && !isLoadingDialogShown) {
              isLoadingDialogShown = true;
              //TODO: shows always in a load state @yogitah
              // await showDialog(
              //   context: context,
              //   barrierColor: context.colorScheme.surface.withOpacity(0.2),
              //   barrierDismissible: false,
              //   builder: (_) => buildLoadingDialog(context, "Loading..."),
              // );
              isLoadingDialogShown = false;
            } else if (!state.loading && isLoadingDialogShown) {
              Navigator.of(context, rootNavigator: true).pop();
            }
          },
        ),
        // // Copy: Show conflict resolution dialog
        BlocListener<FilesBloc, FilesState>(
          listenWhen:
              (prev, curr) => prev.conflictingPaths != curr.conflictingPaths,
          listener: (context, state) {
            if (!state.loading &&
                state.conflictingPaths.isNotEmpty &&
                state.isCopyMode) {
              showModalBottomSheet<ConflictResolutionStrategy>(
                context: context,
                useRootNavigator: true,
                isScrollControlled: true,
                backgroundColor: Colors.transparent,
                builder: (sheetContext) {
                  return ConflictResolutionBottomSheet(
                    conflictingPaths: state.conflictingPaths,
                    destinationPath: state.conflictDestinationPath,
                    controller: controller,
                  );
                },
              );
            }
          },
        ),
        // Controller settings sync
        BlocListener<FilesBloc, FilesState>(
          listenWhen:
              (prev, curr) =>
                  prev.showHiddenFiles != curr.showHiddenFiles ||
                  prev.currentSortBy != curr.currentSortBy ||
                  prev.isAscending != curr.isAscending,
          listener: (context, state) {
            controller.syncSettings(
              showHidden: state.showHiddenFiles,
              sortMode: state.currentSortBy,
              isAscending: state.isAscending,
            );
          },
        ),
      ],
      child: Scaffold(
        appBar: PreferredSize(
          preferredSize: const Size.fromHeight(48),
          child: Padding(
            padding: const EdgeInsets.only(top: 18, right: 16),
            child: MechanixNavigationBar(
              automaticallyImplyLeading: false,
              theme: const MechanixNavigationBarThemeData(
                scrolledUnderElevation: 0,
              ),
              titleWidget:
                  selectionMode
                      ? Text(
                        "${selectedPaths.length} Selected",
                        style: TextStyle(
                          fontSize: 24,
                          color: context.colorScheme.primary,
                          fontWeight: FontWeight.w600,
                        ),
                      ).padRight(24)
                      : ValueListenableBuilder<String>(
                        valueListenable: controller.getPathNotifier,
                        builder: (context, path, _) {
                          final title =
                              (path == '/'
                                  ? "Root"
                                  : getCurrentFolderName(path));

                          return EllipsizedText(
                            title,
                            type: EllipsisType.middle,
                            style: TextStyle(
                              fontSize: 24,
                              color: context.colorScheme.primary,
                              fontWeight: FontWeight.w600,
                            ),
                          );
                        },
                      ),
            ),
          ),
        ),
        body: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: 6),
            Expanded(
              child: ValueListenableBuilder<String>(
                valueListenable: searchQuery,
                builder: (context, query, _) {
                  return ValueListenableBuilder<bool>(
                    valueListenable: viewModeNotifier,
                    builder: (context, isGrid, _) {
                      return isGrid
                          ? buildGridView(
                            context,
                            _scrollController,
                            controller,
                          )
                          : buildListView(
                            context,
                            _scrollController,
                            controller,
                          );
                    },
                  );
                },
              ),
            ),
          ],
        ),
        bottomNavigationBar:
            !isSearching ? _buildBottomActionMenuBar(context) : null,
      ),
    );
  }

  Future<void> handleBack() async {
    if (controller.getCurrentPath.isEmpty) {
      return;
    }

    final current = Directory(controller.getCurrentPath);
    final parent = current.parent;

    // If we are at root, go to FileHomePage
    if (parent.path == current.path || await controller.isRootDirectory()) {
      homeNavigation();
      return;
    }

    // Otherwise go up one directory
    await controller.goToParentDirectory();
  }

  void homeNavigation() {
    Navigator.pop(context);
  }

  OverlayEntry? _searchOverlayEntry;

  void showSearchBottomSheet(
    BuildContext context,
    ValueNotifier<String> searchQuery,
  ) {
    final overlay = Overlay.of(context);
    if (overlay == null) return;

    _searchOverlayEntry?.remove();

    _searchOverlayEntry = OverlayEntry(
      builder: (ctx) {
        WidgetsBinding.instance.addPostFrameCallback((_) async {
          await Future.delayed(const Duration(milliseconds: 300));
          _focusNode.addListener(focusChange);
          if (!mounted) return;
          if (!_focusNode.canRequestFocus) return;
          if (_searchOverlayEntry == null) return; // overlay still exists

          // FocusManager.instance.primaryFocus?.unfocus();
          // _focusNode.requestFocus();
        });

        return Positioned(
          left: 0,
          right: 0,
          bottom: 0,
          child: Material(
            color: Colors.transparent,
            child: Container(
              color: context.secondaryContainer,
              child: MechanixTextInput.search(
                autofocus: true,
                // canRequestFocus: true,
                focusNode: _focusNode,
                prefixIcon: IconWidget(
                  iconPath: Images.search,
                  iconColor: context.colorScheme.onSurface,
                  iconHeight: 24,
                  iconWidth: 24,
                ),
                isClearButtonRequired: false,
                anchorWidget: Padding(
                  padding: const EdgeInsets.only(left: 5),
                  child: DecoratedPressableIcon(
                    onTap: () {
                      clearSearch();
                    },
                    tapBackgroundColor: context.colorScheme.surfaceContainer
                        .withAlpha(100),
                    icon: const Icon(Icons.close),
                  ),
                ),
                hintText: "Search here",
                onChanged: (query) {
                  searchQuery.value = query;

                  if (query.trim().length > 2) {
                    controller.search(query.trim());
                  }
                },
                onClear: () {
                  clearSearch();
                  _buildBottomActionMenuBar(context);
                },
              ),
            ),
          ),
        );
      },
    );

    overlay.insert(_searchOverlayEntry!);
  }

  void clearSearch() {
    setState(() {
      isSearching = false;
      searchQuery.value = '';
    });

    // safely remove overlay if still mounted
    _searchOverlayEntry?.remove();
    _searchOverlayEntry = null;

    // Reload directory content when clearing search
    controller.search('');
  }

  /// Shows bottom sheet for selecting sort mode
  Widget showSortMenu(
    BuildContext context,
    String currentSortByFromBloc,
    bool ascendingFromBloc,
  ) {
    // Convert sort key → SortBy enum
    final currentSort = sortByFromKey(currentSortByFromBloc);

    // Decide which ascending value to use:
    // 1. If controller already has value → use it
    // 2. Otherwise → use selector (bloc) value
    final effectiveAscending =
        controller.hasSortApplied ? controller.isAscending : ascendingFromBloc;

    final offset = const Offset(-8, -14);

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 100),
      buttonIcon: IconWidget(
        iconWidth: 28,
        iconHeight: 28,
        iconPath: Images.sortAscending,
        iconColor:
            isSortMenuOpen
                ? context.colorScheme.primaryContainer
                : context.colorScheme.onSurface,
      ),
      openMenu: () => setState(() => isSortMenuOpen = true),
      closeMenu: () => setState(() => isSortMenuOpen = false),
      items: [
        _buildSortMenuItem(
          context,
          key: 'name',
          label: 'Name',
          currentSort: currentSort,
          isAscending: effectiveAscending,
        ),
        _buildSortMenuItem(
          context,
          key: 'type',
          label: 'Date created',
          currentSort: currentSort,
          isAscending: effectiveAscending,
          isDisabled: true,
        ),
        _buildSortMenuItem(
          context,
          key: 'accessed_time',
          label: 'Date accessed',
          currentSort: currentSort,
          isAscending: effectiveAscending,
        ),
        _buildSortMenuItem(
          context,
          key: 'mod_time',
          label: 'Date modified',
          currentSort: currentSort,
          isAscending: effectiveAscending,
        ),
        _buildSortMenuItem(
          context,
          key: 'size',
          label: 'Size',
          currentSort: currentSort,
          isAscending: effectiveAscending,
        ),
      ],
    );
  }

  /// Builds a selectable menu item
  MechanixMenuItemsType _buildSortMenuItem(
    BuildContext context, {
    required String key,
    required String label,
    required SortBy currentSort,
    required bool isAscending,
    bool isDisabled = false,
  }) {
    final isSelected = sortByFromKey(key) == currentSort;

    // If selected → choose correct icon
    Widget? trailing;
    if (isSelected) {
      final icon = isAscending ? Images.sortAscending : Images.sortDescending;
      trailing = Padding(
        padding: const EdgeInsets.only(right: 14),
        child: Image.asset(
          icon,
          width: 24,
          height: 24,
          color: context.colorScheme.primaryContainer,
        ),
      );
    }

    return MechanixMenuItemsType(
      title: label,
      trailing: trailing,
      isSelected: isSelected,
      disabled: isDisabled,
      onTap: () {
        final sortBy = sortByFromKey(key);

        bool nextAscending = isAscending;

        // Toggle if same key tapped again
        if (currentSort == sortBy) {
          nextAscending = !isAscending;
        } else {
          nextAscending = true; // default ASC on first selection
        }

        controller.sortBy(sortBy, isAscending: nextAscending);
        controller.reload();

        handleSortMode(key, nextAscending);
      },
    );
  }

  bool get hasSelection => selectedPaths.isNotEmpty;

  Widget _buildBottomActionMenuBar(BuildContext context) {
    return MechanixBottomBar(
      theme: MechanixBottomBarThemeData(
        height: isTextInputOpened ? 65 : 90,
        decoration: BoxDecoration(
          color: context.colorScheme.secondaryContainer,
          borderRadius:
              !selectionMode
                  ? const BorderRadius.only(
                    topLeft: Radius.circular(8),
                    topRight: Radius.circular(8),
                  )
                  : null,
        ),
      ),
      leadingWidget: [
        BottomBarButton.widget(
          widget: Padding(
            padding: const EdgeInsets.only(left: 8),
            child: IconButton(
              icon: const IconWidget(
                iconHeight: 28,
                iconWidth: 28,
                iconPath: Images.back,
              ),
              onPressed:
                  selectionMode
                      ? clearSelection
                      : () {
                        if (isHomePageDir) {
                          homeNavigation();
                        } else {
                          handleBack();
                        }
                      },
            ),
          ),
        ),
      ],
      centerWidgetSpacing: 28,
      centerWidget: [
        BottomBarButton.widget(
          widget: IconButton(
            onPressed:
                !selectionMode
                    ? () {
                      setState(() => isSearching = true);
                      showSearchBottomSheet(context, searchQuery);
                    }
                    : null,
            icon: IconWidget(
              iconHeight: 28,
              iconWidth: 28,
              iconPath: Images.search,
              iconColor: selectionMode ? context.colorScheme.outline : null,
            ),
          ),
        ),
        BottomBarButton.widget(
          widget: ValueListenableBuilder<bool>(
            valueListenable: viewModeNotifier,
            builder: (context, isList, _) {
              return IconButton(
                onPressed: () {
                  viewModeNotifier.value = !isList;
                },
                icon: IconWidget(
                  iconHeight: 28,
                  iconWidth: 28,
                  iconPath: isList ? Images.list : Images.grid,
                ),
              );
            },
          ),
        ),
        BottomBarButton.widget(
          widget: BlocSelector<
            FilesBloc,
            FilesState,
            (String sortBy, bool ascending)
          >(
            selector: (state) => (state.currentSortBy, state.isAscending),
            builder: (context, sortData) {
              final currentSortBy = sortData.$1;
              final isAscending = sortData.$2;

              return showSortMenu(context, currentSortBy, isAscending);
            },
          ),
        ),
        BottomBarButton.extension(
          animationDuration: const Duration(milliseconds: 100),
          floatingActionBarTheme: MechanixFloatingActionBarThemeData(
            barMainAxisAlignment: MainAxisAlignment.center,
            width: double.infinity,
            barSpacing: 26,
            decoration: BoxDecoration(
              color: context.colorScheme.secondary,
              borderRadius: const BorderRadius.only(
                topLeft: Radius.circular(8),
                topRight: Radius.circular(8),
              ),
            ),
          ),
          iconTheme: const MechanixBottomBarIconThemeData(),
          outsideClickDisabled: true,
          floatingActionBarController: _fabController,
          offset: const Offset(-112, -4),
          iconWidget: IconWidget(
            iconPath: Images.checkCircle,
            iconWidth: 28,
            iconHeight: 28,
            iconColor:
                selectionMode
                    ? context.colorScheme.primaryContainer
                    : context.colorScheme.onSurface,
          ),
          isSelected: selectionMode,
          onPressed: () {
            if (!selectionMode) enableSelect();
            isSelectionActionMenuOpen = false;
          },
          onExtensionClose: () {
            if (selectionMode) clearSelection();
            isFolderActionMenuOpen =
                false; // TODO: Temporary fix - to show inactive menu button color
          },
          extensionWidgets: [
            BottomBarButton.widget(
              widget: ValueListenableBuilder<bool>(
                valueListenable: hasSelectionNotifier,
                builder: (context, hasSelection, _) {
                  return IconButton(
                    onPressed: hasSelection ? handleCopy : null,
                    icon: IconWidget(
                      iconHeight: 26,
                      iconWidth: 26,
                      iconPath: Images.copy,
                      iconColor:
                          !hasSelection ? context.colorScheme.outline : null,
                    ),
                  );
                },
              ),
            ),
            BottomBarButton.widget(
              widget: ValueListenableBuilder<bool>(
                valueListenable: hasSelectionNotifier,
                builder: (context, hasSelection, _) {
                  return IconButton(
                    onPressed: hasSelection ? handleMove : null,
                    icon: IconWidget(
                      iconHeight: 26,
                      iconWidth: 26,
                      iconPath: Images.move,
                      iconColor:
                          !hasSelection ? context.colorScheme.outline : null,
                    ),
                  );
                },
              ),
            ),
            BottomBarButton.widget(
              widget: IconButton(
                onPressed: null,
                icon: IconWidget(
                  iconHeight: 26,
                  iconWidth: 26,
                  iconPath: Images.share,
                  iconColor: context.colorScheme.outline,
                ),
              ),
            ),
            BottomBarButton.widget(
              widget: ValueListenableBuilder<bool>(
                valueListenable: hasSelectionNotifier,
                builder: (context, hasSelection, _) {
                  return IconButton(
                    onPressed: hasSelection ? handleDelete : null,
                    icon: IconWidget(
                      iconHeight: 26,
                      iconWidth: 26,
                      iconPath: Images.delete,
                      iconColor:
                          !hasSelection ? context.colorScheme.outline : null,
                    ),
                  );
                },
              ),
            ),
          ],
        ),
      ],
      anchorWidget: [
        BottomBarButton.widget(
          widget:
              selectionMode
                  ? buildSelectionActionsMenu(context)
                  : buildFolderActionsMenu(context),
        ),
      ],
    );
  }

  Widget buildSelectionActionsMenu(BuildContext context) {
    final offset = const Offset(-8, -14);
    // Check if selected files are ZIP files
    final isZipFileSelected =
        selectedPaths.isNotEmpty &&
        selectedPaths.every((p) => p.toLowerCase().endsWith('.zip'));

    final isSingleSelection = selectedPaths.length == 1;
    final selectedPath = isSingleSelection ? selectedPaths.first : null;
    final isFileSelected =
        isSingleSelection &&
        FileSystemEntity.typeSync(selectedPath!) == FileSystemEntityType.file;

    return MechanixMenu(
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 100),
      offset: offset,
      buttonIcon: IconWidget(
        iconPath: Images.dots,
        iconColor:
            isSelectionActionMenuOpen
                ? context.colorScheme.primaryContainer
                : context.colorScheme.onSurface,
        iconHeight: 28,
        iconWidth: 28,
      ),
      openMenu: () {
        setState(() => isSelectionActionMenuOpen = true);
      },
      closeMenu: () {
        setState(() => isSelectionActionMenuOpen = false);
      },
      items: [
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.extract,
            color:
                isZipFileSelected
                    ? context.colorScheme.onSurface
                    : context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Extract',
          onTap: () {
            handleExtraction(context, selectedPaths);
          },
          disabled: !isZipFileSelected,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.compress,
            color:
                hasSelection
                    ? context.colorScheme.onSurface
                    : context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Compress',
          onTap: hasSelection ? handleCompress : null,
          disabled: !hasSelection,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.duplicate,
            color:
                isFileSelected
                    ? context.colorScheme.onSurface
                    : context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Duplicate',
          onTap:
              isFileSelected
                  ? () {
                    handleDuplicate(selectedPath);
                  }
                  : null,
          disabled: !isFileSelected,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.rename,
            color:
                selectedPaths.length == 1
                    ? context.colorScheme.onSurface
                    : context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Rename',
          onTap:
              selectedPaths.length == 1
                  ? () {
                    final selectedPath = selectedPaths.first;
                    showRenameSheet(initialName: p.basename(selectedPath));
                    clearSelection();
                  }
                  : null,
          disabled: selectedPaths.length != 1,
        ),
        MechanixMenuItemsType(
          title: "Select all",
          leading: Image.asset(
            Images.listChecks,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          onTap: () {
            handleSelectAll();
          },
        ),
        MechanixMenuItemsType(
          title:
              controller.showHiddenFiles
                  ? "Hide hidden files"
                  : "Show hidden files",
          leading: Image.asset(
            controller.showHiddenFiles ? Images.eye : Images.eyeSlash,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          onTap: () {
            _toggleHiddenFiles();
          },
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.info,
            color:
                selectedPaths.length == 1
                    ? context.colorScheme.onSurface
                    : context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Properties',
          onTap: selectedPaths.length == 1 ? handleProperties : null,
          disabled: selectedPaths.length != 1,
        ),
      ],
    ).padRight(8);
  }

  Widget buildFolderActionsMenu(BuildContext context) {
    final currentPath = controller.getPathNotifier.value;
    bool actionTaken = false;

    final state = BlocProvider.of<FilesBloc>(context).state;
    final offset = const Offset(-8, -14);
    bool isPasteDisabled = !(state.isCopyMode || state.isMoveMode);

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 100),
      buttonIcon: IconWidget(
        iconPath: Images.dots,
        iconColor:
            isFolderActionMenuOpen
                ? context.colorScheme.primaryContainer
                : context.colorScheme.onSurface,
        iconHeight: 28,
        iconWidth: 28,
      ),
      openMenu: () {
        setState(() => isFolderActionMenuOpen = true);
      },
      closeMenu: () {
        setState(() => isFolderActionMenuOpen = false);
      },
      items: [
        MechanixMenuItemsType(
          title: "Paste",
          leading: Image.asset(
            Images.paste,
            color:
                isPasteDisabled
                    ? context.colorScheme.onSurfaceVariant
                    : context.colorScheme.onSurface,
            height: mechanixIconSize,
            width: mechanixIconSize,
          ),
          disabled: isPasteDisabled,
          onTap: () {
            actionTaken = true;
            handlePaste(context);
            clearSelection();
            reload();
          },
        ),
        MechanixMenuItemsType(
          title: "New folder",
          leading: Image.asset(
            Images.createFolder,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            createFolderAndRename();
            clearSelection();
          },
        ),
        MechanixMenuItemsType(
          title: "Select all",
          leading: Image.asset(
            Images.listChecks,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            handleSelectAll();
          },
        ),
        MechanixMenuItemsType(
          title:
              controller.showHiddenFiles
                  ? "Hide hidden files"
                  : "Show hidden files",
          leading: Image.asset(
            controller.showHiddenFiles ? Images.eye : Images.eyeSlash,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            _toggleHiddenFiles();
          },
        ),
        MechanixMenuItemsType(
          title: "Properties",
          leading: Image.asset(
            Images.info,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            showDetailsDialog(context, currentPath);
            clearSelection();
          },
        ),
      ],
    ).padRight(8);
  }

  void handleSelectAll() {
    final files = controller.paginatedEntities.value; // current page

    setState(() {
      // Always select ALL files (no toggle)
      selectedPaths = {for (final f in files) f.path};
      selectionMode = true;
    });

    // Update notifiers
    setHasSelection(selectedPaths.isNotEmpty);
    allSelectedNotifier.value = true;

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      _openFabMenuProgrammatically();
    });
  }

  bool isSelected(String path) => selectedPaths.contains(path);

  void setHasSelection(bool value) {
    if (hasSelectionNotifier.value != value) {
      hasSelectionNotifier.value = value;
    }
  }

  void toggleSelection(String path) {
    setState(() {
      if (selectedPaths.contains(path)) {
        selectedPaths.remove(path);
      } else {
        selectedPaths.add(path);
      }

      setHasSelection(selectedPaths.isNotEmpty);

      // Keep selectionMode true once initiated
      selectionMode = true;
    });

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;

      if (selectionMode) {
        _openFabMenuProgrammatically();
      } else {
        _closeFabMenuProgrammatically();
      }
    });
  }

  void clearSelection() {
    if (!mounted) return;

    setState(() {
      selectionMode = false;
      selectedPaths.clear();
    });

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        _closeFabMenuProgrammatically();
      }
    });

    if (hasSelectionNotifier.value != false) {
      hasSelectionNotifier.value = false;
    }
  }

  void enableSelect() {
    setState(() {
      selectionMode = true;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _openFabMenuProgrammatically(); // Open FAB when selection starts
      });
      selectedPaths.clear();
    });
  }

  Widget _buildCustomMoveSheet() {
    final filesBloc = context.read<FilesBloc>();
    final screenHeight = MediaQuery.of(context).size.height;
    final sheetWidth = MediaQuery.of(context).size.width;

    return ClipPath(
      clipper: TabClipper(shift: sheetWidth * 0.70),
      child: Container(
        height: screenHeight * 0.98,
        decoration: BoxDecoration(
          color: context.colorScheme.surfaceContainerHigh,
          borderRadius: const BorderRadius.only(
            topLeft: Radius.circular(12),
            topRight: Radius.circular(12),
          ),
        ),
        child: MoveBottomSheetContent(
          filesBloc: filesBloc,
          selectedCount: selectedPaths.length,
          reload: reload,
          currentPath: controller.getPathNotifier.value,
          rootContext: context,
        ),
      ),
    );
  }

  void handleMove() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _closeFabMenuProgrammatically();
    });

    final filesBloc = context.read<FilesBloc>();
    filesBloc.add(StartMoveMode(selectedPaths.toList()));

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      enableDrag: true,
      builder: (_) {
        return BlocProvider.value(
          value: filesBloc, // keep same instance
          child: BlocListener<FilesBloc, FilesState>(
            listenWhen:
                (prev, curr) => prev.conflictingPaths != curr.conflictingPaths,
            listener: (context, state) async {
              if (!state.loading &&
                  state.conflictingPaths.isNotEmpty &&
                  state.isMoveMode) {
                final rootContext = Navigator.of(context).context;

                final conflicts =
                    state.conflictingPaths.map((path) {
                      return FileConflict(
                        path: path,
                        fileName: p.basename(path),
                        destination: state.conflictDestinationPath,
                      );
                    }).toList();

                totalMovedCount = state.movedPaths.length;

                await handleConflictsSequentially(rootContext, conflicts);

                if (rootContext.mounted) {
                  final folderName = p.basename(state.conflictDestinationPath);

                  MechanixNotification.show(
                    context: context,
                    notificationType: NotificationType.success,
                    message:
                        totalMovedCount > 0
                            ? "Moved $totalMovedCount item${totalMovedCount > 1 ? 's' : ''} to '$folderName'"
                            : "No items were moved",
                  );

                  // exit move mode
                  filesBloc.add(CancelMoveMode());
                  reload();
                }
              }
            },
            child: _buildCustomMoveSheet(),
          ),
        );
      },
    ).whenComplete(() => clearSelection());
  }

  void handleDelete() {
    confirmDelete(context, selectedPaths);
    clearSelection();
  }

  void handleCopy() {
    BlocProvider.of<FilesBloc>(
      context,
    ).add(StartCopyMode(selectedPaths.toList()));

    MechanixNotification.show(
      context: context,
      notificationType: NotificationType.success,
      message:
          "Copied ${selectedPaths.length} item${selectedPaths.length > 1 ? 's' : ''}",
    );

    clearSelection();
  }

  void showCompressOverlay({
    required String defaultZipName,
    required String destinationDirPath,
    required List<String> selectedPaths,
  }) {
    String currentName = defaultZipName;
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    final overlay = Overlay.of(context);
    OverlayEntry? entry;

    entry = OverlayEntry(
      builder: (ctx) {
        WidgetsBinding.instance.addPostFrameCallback((_) async {
          if (!_focusNode.canRequestFocus) return;
          if (entry == null) return;

          FocusManager.instance.primaryFocus?.unfocus();
          _focusNode.requestFocus();
        });

        return Positioned(
          left: 0,
          right: 0,
          bottom: 0,
          child: Material(
            color: Colors.transparent,
            child: StatefulBuilder(
              builder: (context, setState) {
                final bool isEmpty = currentName.trim().isEmpty;
                final bool showCheck = !isEmpty;

                return Padding(
                  padding: EdgeInsets.only(
                    bottom: MediaQuery.of(ctx).viewInsets.bottom,
                  ),
                  child: Container(
                    height: 90,
                    child: MechanixTextInput.textInput(
                      autofocus: true,
                      canRequestFocus: true,
                      focusNode: _focusNode,
                      cursorColor: context.colorScheme.primaryContainer,
                      initialValue: defaultZipName,
                      onChanged: (v) {
                        setState(() {
                          currentName = v;
                        });
                      },
                      anchorWidget:
                          showCheck
                              ? Padding(
                                padding: const EdgeInsets.only(left: 5),
                                child: DecoratedPressableIcon(
                                  icon: const Icon(Icons.check),
                                  onTap: () async {
                                    final trimmed = currentName.trim();

                                    if (trimmed.isEmpty) return;

                                    final zipPath = p.join(
                                      destinationDirPath,
                                      trimmed.endsWith('.zip')
                                          ? trimmed
                                          : '$trimmed.zip',
                                    );

                                    final exists = await File(zipPath).exists();
                                    if (exists) {
                                      setState(() {
                                        currentName = "";
                                      });
                                      return;
                                    }

                                    entry?.remove();

                                    filesBloc.add(
                                      CompressEntitiesEvent(
                                        sourcePaths: selectedPaths.toList(),
                                        destinationZipPath: zipPath,
                                        controller: controller,
                                      ),
                                    );

                                    clearSelection();
                                  },
                                  tapBackgroundColor: context
                                      .colorScheme
                                      .surfaceContainer
                                      .withAlpha(100),
                                ),
                              )
                              : Padding(
                                padding: const EdgeInsets.only(left: 5),
                                child: DecoratedPressableIcon(
                                  onTap: () {
                                    entry?.remove();
                                  },
                                  tapBackgroundColor: context
                                      .colorScheme
                                      .surfaceContainer
                                      .withAlpha(100),
                                  icon: const Icon(Icons.close),
                                ),
                              ),
                    ),
                  ),
                );
              },
            ),
          ),
        );
      },
    );

    overlay.insert(entry);
  }

  Future<void> handleDuplicate(String sourcePath) async {
    final file = File(sourcePath);
    if (!await file.exists()) return;

    final dirPath = p.dirname(sourcePath);
    final baseName = p.basenameWithoutExtension(sourcePath);
    final ext = p.extension(sourcePath);

    String newName = '$baseName (Copy)$ext';
    String newPath = p.join(dirPath, newName);

    int copyIndex = 2;

    // If "(Copy)" already exists → "(Copy 2)", "(Copy 3)", etc.
    while (await File(newPath).exists()) {
      newName = '$baseName (Copy $copyIndex)$ext';
      newPath = p.join(dirPath, newName);
      copyIndex++;
    }

    try {
      await file.copy(newPath);

      MechanixNotification.show(
        context: context,
        notificationType: NotificationType.success,
        message: "Duplicated as '$newName'",
      );

      reload(); // refresh list
    } catch (e) {
      MechanixNotification.show(
        context: context,
        notificationType: NotificationType.error,
        message: "Failed to duplicate file",
      );
    } finally {
      clearSelection();
    }
  }

  void handleCompress() async {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _closeFabMenuProgrammatically();
    });

    if (selectedPaths.isEmpty) return;

    final destinationDirPath = p.dirname(selectedPaths.first);

    String baseZipName;

    if (selectedPaths.length == 1) {
      // Single file → filename.zip
      final singlePath = selectedPaths.first;
      baseZipName = p.basenameWithoutExtension(singlePath);
    } else {
      // Multiple files → Archive.zip
      baseZipName = 'Archive';
    }

    final uniqueZipName = await generateUniqueZipName(
      destinationDir: destinationDirPath,
      baseName: baseZipName,
    );

    showCompressOverlay(
      defaultZipName: uniqueZipName,
      destinationDirPath: destinationDirPath,
      selectedPaths: selectedPaths.toList(),
    );
  }

  Widget _buildCustomExtractSheet() {
    final filesBloc = context.read<FilesBloc>();
    final screenHeight = MediaQuery.of(context).size.height;
    final sheetWidth = MediaQuery.of(context).size.width;

    return ClipPath(
      clipper: TabClipper(shift: sheetWidth * 0.70),
      child: Container(
        height: screenHeight * 0.98,
        decoration: BoxDecoration(
          color: context.colorScheme.surfaceContainerHigh,
          borderRadius: const BorderRadius.only(
            topLeft: Radius.circular(12),
            topRight: Radius.circular(12),
          ),
        ),
        child: ExtractBottomSheetContent(
          filesBloc: filesBloc,
          selectedCount: selectedPaths.length,
          reload: reload,
          currentPath: controller.getPathNotifier.value,
          rootContext: context,
        ),
      ),
    );
  }

  void handleExtraction(BuildContext context, Set<String> selectedPaths) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _closeFabMenuProgrammatically();
    });

    final filesBloc = context.read<FilesBloc>();
    filesBloc.add(StartExtractMode(selectedPaths.toList()));

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      enableDrag: true,
      builder: (_) {
        return BlocProvider.value(
          value: filesBloc, // keep same bloc instance
          child: BlocListener<FilesBloc, FilesState>(
            listenWhen:
                (prev, curr) =>
                    prev.extractStatus != curr.extractStatus &&
                    curr.extractStatus == FileExtractStatus.completed,
            listener: (context, state) async {
              final rootContext = Navigator.of(context).context;
              if (!rootContext.mounted) return;

              final successCount = state.extractSuccessCount;
              final failureCount = state.extractFailureCount;

              String msg;
              if (failureCount == 0) {
                msg =
                    "Extracted $successCount file${successCount > 1 ? 's' : ''} successfully";
              } else if (successCount == 0) {
                msg =
                    "Failed to extract $failureCount file${failureCount > 1 ? 's' : ''}";
              } else {
                msg =
                    "$successCount extracted successfully • $failureCount failed";
              }

              // Show final summary
              MechanixNotification.show(
                context: context,
                notificationType: NotificationType.success,
                message: msg,
              );

              // Exit extract mode
              filesBloc.add(CancelExtractMode());
              reload();
            },
            child: _buildCustomExtractSheet(),
          ),
        );
      },
    ).whenComplete(() {
      clearSelection();
    });
  }

  void handleProperties() {
    if (selectedPaths.length == 1) {
      final selectedPath = selectedPaths.first;
      showDetailsDialog(context, selectedPath);
    } else {
      MechanixNotification.show(
        context: context,
        notificationType: NotificationType.standard,
        message: "Select a single item to view details",
      );
    }
    clearSelection();
  }

  void handlePaste(BuildContext context) {
    final targetPath = controller.getPathNotifier.value;

    final bloc = BlocProvider.of<FilesBloc>(context);
    // Read the current bloc state synchronously and perform the paste operation.
    final state = bloc.state;

    if (state.isCopyMode) {
      bloc.add(
        Copy(
          sourcePaths: state.copiedPaths,
          destinationPath: targetPath,
          controller: controller,
        ),
      );
      bloc.add(CancelCopyMode());
    }
  }

  void handleSortMode(String value, bool isAscending) {
    BlocProvider.of<FilesBloc>(context).add(SortFiles(value, isAscending));
  }

  void showDetailsDialog(BuildContext context, String path) {
    final bloc = context.read<FilesBloc>();
    bloc.add(FetchFileDetails(path));

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      barrierColor: Colors.transparent,
      enableDrag: false,
      builder: (sheetContext) {
        return GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: () => Navigator.pop(sheetContext),
          child: FileDetailsDialog(path: path),
        );
      },
    );
  }

  Future<void> createFolderAndRename() async {
    final path = controller.getCurrentPath;
    final bloc = context.read<FilesBloc>();

    final folderName = await generateUniqueFolderName(path);

    // Create the folder
    bloc.add(
      CreateFolder(path: path, folderName: folderName, controller: controller),
    );

    // After creation, show rename sheet with initial value
    Future.delayed(const Duration(milliseconds: 200), () {
      showRenameSheet(initialName: folderName);
    });
  }

  Future<String?> showRenameSheet({required String initialName}) async {
    String folderName = initialName;
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    final oldPath = p.join(controller.getCurrentPath, initialName);

    final completer = Completer<String?>();
    controller.markNewFolder(oldPath);

    final overlay = Overlay.of(context);
    OverlayEntry? entry;

    entry = OverlayEntry(
      builder: (ctx) {
        _focusNode.addListener(focusChange);

        WidgetsBinding.instance.addPostFrameCallback((_) async {
          if (!_focusNode.canRequestFocus) return;
          if (entry == null) return;

          // FocusManager.instance.primaryFocus?.unfocus();
          // _focusNode.requestFocus();

          print("_focusNode.hasFocus");
          print(_focusNode.hasFocus);
          // setState(() {
          //   isTextInputOpened = _focusNode.hasFocus;
          // });
        });

        return Positioned(
          left: 0,
          right: 0,
          bottom: 0,
          child: Material(
            color: Colors.transparent,
            child: StatefulBuilder(
              builder: (context, setState) {
                final bool isEmpty = folderName.trim().isEmpty;
                final bool isSame = folderName.trim() == initialName.trim();
                final bool showCheck = !isEmpty && !isSame;

                return Padding(
                  padding: EdgeInsets.zero,
                  child: Column(
                    children: [
                      Container(
                        color: context.surfaceContainerHigh,
                        child: MechanixTextInput.textInput(
                          autofocus: true,
                          focusNode: _focusNode,
                          cursorColor: context.colorScheme.primaryContainer,
                          onChanged: (v) {
                            setState(() {
                              folderName = v;
                            });

                            controller.setLiveRename(oldPath, v);
                          },
                          initialValue: initialName,
                          anchorWidget:
                              showCheck
                                  ? Padding(
                                    padding: const EdgeInsets.only(left: 5),
                                    child: DecoratedPressableIcon(
                                      icon: const Icon(Icons.check),
                                      onTap: () {
                                        entry?.remove();

                                        final newFullPath = p.join(
                                          p.dirname(oldPath),
                                          folderName,
                                        );

                                        filesBloc.add(
                                          Rename(
                                            oldPath: oldPath,
                                            newName: folderName,
                                            controller: controller,
                                          ),
                                        );

                                        controller.clearLiveRename();
                                        controller.clearNewFolder();

                                        completer.complete(newFullPath);
                                      },
                                      tapBackgroundColor: context
                                          .colorScheme
                                          .surfaceContainer
                                          .withAlpha(100),
                                    ),
                                  )
                                  : Padding(
                                    padding: const EdgeInsets.only(left: 5),
                                    child: DecoratedPressableIcon(
                                      icon: const Icon(Icons.close),
                                      onTap: () {
                                        entry?.remove();
                                        controller.clearLiveRename();
                                        controller.clearNewFolder();
                                        completer.complete(null);
                                      },
                                      tapBackgroundColor: context
                                          .colorScheme
                                          .surfaceContainer
                                          .withAlpha(100),
                                    ),
                                  ),
                        ),
                      ),
                    ],
                  ),
                );
              },
            ),
          ),
        );
      },
    );

    overlay.insert(entry);

    return completer.future;
  }

  void _toggleHiddenFiles() {
    controller.toggleShowHiddenFiles();
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    filesBloc.add(ToggleHiddenFiles());
  }

  void reload() {
    controller.reload();
  }

  void confirmDelete(BuildContext context, Set<String> selectedPaths) {
    final pathsToDelete = selectedPaths.toList();

    final isSingle = pathsToDelete.length == 1;
    final message =
        isSingle
            ? "This action will delete the file permanently"
            : "This action will delete the files permanently";

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        final double bottomSheetWidth =
            MediaQuery.of(bottomSheetContext).size.width;

        return ClipPath(
          clipper: TabClipper(shift: 400),
          child: Container(
            padding: const EdgeInsets.only(
              left: 16,
              right: 16,
              bottom: 30,
              top: 32,
            ),
            decoration: BoxDecoration(
              color: context.colorScheme.surfaceContainerHigh,
            ),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                LayoutBuilder(
                  builder: (context, constraints) {
                    var baseStyle = confirmationDialogRegularStyle(context);

                    var boldStyleText = confirmationDialogBoldStyle(context);

                    if (isSingle) {
                      final rawName = pathsToDelete.first.split('/').last;

                      final prefixWidth = textWidth("Delete '", baseStyle);
                      final suffixWidth = textWidth("' ?", baseStyle);
                      final availableWidth =
                          constraints.maxWidth - prefixWidth - suffixWidth;

                      final truncatedName = middleEllipsisString(
                        rawName,
                        availableWidth,
                        boldStyleText,
                      );

                      return RichText(
                        maxLines: 1,
                        overflow: TextOverflow.clip,
                        text: TextSpan(
                          children: [
                            TextSpan(text: "Delete ", style: baseStyle),
                            TextSpan(
                              text: "'$truncatedName' ?",
                              style: boldStyleText,
                            ),
                          ],
                        ),
                      );
                    }

                    // Multi-file case
                    return RichText(
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      text: TextSpan(
                        children: [
                          TextSpan(text: "Delete ", style: baseStyle),
                          TextSpan(
                            text: "${pathsToDelete.length} files?",
                            style: boldStyleText,
                          ),
                        ],
                      ),
                    );
                  },
                ),
                const SizedBox(height: 8),
                Text(
                  message,
                  style: TextStyle(
                    color: context.colorScheme.onSurface,
                    fontSize: 20,
                    fontWeight: FontWeight.w400,
                  ),
                ),
                const SizedBox(height: 16),
                Row(
                  children: [
                    Expanded(
                      child: MechanixFilledButton(
                        onPressed: () => Navigator.pop(bottomSheetContext),
                        theme: buttonThemeData(
                          context,
                          type: MechanixButtonType.cancel,
                        ),
                        label: "Cancel",
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: MechanixFilledButton(
                        theme: buttonThemeData(
                          context,
                          type: MechanixButtonType.delete,
                        ),
                        label: "Delete",
                        onPressed: () {
                          Navigator.pop(bottomSheetContext);
                          context.read<FilesBloc>().add(
                            DeleteEntities(pathsToDelete, controller),
                          );
                        },
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
}
