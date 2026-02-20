import 'dart:io';

import 'package:ellipsized_text/ellipsized_text.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/file_details_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/grid_view.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:mechanix_files/src/features/files/presentation/search_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/view_mode_notifier.dart';
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

class RecentFilesPage extends StatefulWidget {
  const RecentFilesPage({super.key});

  @override
  State<RecentFilesPage> createState() => RecentFilesPageState();
}

class RecentFilesPageState extends State<RecentFilesPage> {
  final FileManagerController controller = FileManagerController();
  final ValueNotifier<String> searchQuery = ValueNotifier('');
  bool isSearching = false;
  bool selectionMode = false;
  Set<String> selectedPaths = {};
  bool showHiddenFiles = false;
  bool isLoadingDialogShown = false;
  bool isCopyMode = false;
  List<String> copiedPaths = [];

  bool isFolderActionMenuOpen = false;
  bool isSelectionActionMenuOpen = false;
  bool isSortMenuOpen = false;
  bool isTextInputOpened = false;

  final searchOverlayController = SearchOverlayController();

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

  final ValueNotifier<bool> hasSelectionNotifier = ValueNotifier(false);
  final ValueNotifier<bool> allSelectedNotifier = ValueNotifier(false);

  @override
  void initState() {
    super.initState();

    /// Load recents once
    context.read<FilesBloc>().add(LoadRecentFiles());
  }

  @override
  void dispose() {
    searchQuery.dispose();
    searchOverlayController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
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
                    : Text(
                      "Recents",
                      style: TextStyle(
                        fontSize: 24,
                        color: context.colorScheme.primary,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
          ),
        ),
      ),
      body: BlocBuilder<FilesBloc, FilesState>(
        builder: (context, state) {
          /// Loading
          if (state.loading) {
            return const Center(child: CircularProgressIndicator());
          }

          /// Empty
          if (state.fileSystemList.isEmpty) {
            return const Center(child: Text("No recent files"));
          }

          /// Content
          return ValueListenableBuilder<String>(
            valueListenable: searchQuery,
            builder: (context, query, _) {
              final filteredFiles =
                  query.isEmpty
                      ? state.fileSystemList
                      : state.fileSystemList.where((file) {
                        return p
                            .basename(file.path)
                            .toLowerCase()
                            .contains(query.toLowerCase());
                      }).toList();

              return ValueListenableBuilder<bool>(
                valueListenable: viewModeNotifier,
                builder: (context, isGrid, _) {
                  return isGrid
                      ? buildGridViewForRecentFiles(context, filteredFiles)
                      : buildListViewForRecentFiles(context, filteredFiles);
                },
              );
            },
          );
        },
      ),
      bottomNavigationBar:
          !isSearching ? _buildBottomActionMenuBar(context) : null,
    );
  }

  Future<void> handleBack() async {
    Navigator.pop(context);
  }

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
                        handleBack();
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
                      searchOverlayController.show(
                        context,
                        searchQuery: searchQuery,

                        onClear: () {
                          clearSearch();
                          _buildBottomActionMenuBar(context);
                        },

                        onSearch: (query) {
                          if (query.length > 2) {
                            controller.search(query);
                          }
                        },
                      );
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
                    onPressed: null,
                    icon: IconWidget(
                      iconHeight: 26,
                      iconWidth: 26,
                      iconPath: Images.move,
                      iconColor: context.colorScheme.outline,
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
            color: context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Extract',
          onTap: null,
          disabled: true,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.compress,
            color: context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Compress',
          onTap: null,
          disabled: true,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.duplicate,
            color: context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Duplicate',
          onTap: null,
          disabled: true,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.rename,
            color: context.colorScheme.onSurfaceVariant,
            height: 22,
          ),
          title: 'Rename',
          onTap: null,
          disabled: true,
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
    final offset = const Offset(-8, -14);

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
            color: context.colorScheme.onSurfaceVariant,
            height: mechanixIconSize,
            width: mechanixIconSize,
          ),
          disabled: true,
          onTap: null,
        ),
        MechanixMenuItemsType(
          title: "New folder",
          leading: Image.asset(
            Images.createFolder,
            color: context.colorScheme.onSurfaceVariant,
            height: mechanixIconSize,
          ),
          disabled: true,
          onTap: null,
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
            color: context.colorScheme.onSurfaceVariant,
            height: mechanixIconSize,
          ),
          onTap: null,
          disabled: true,
        ),
      ],
    ).padRight(8);
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

  void handleSelectAll() {
    final state = context.read<FilesBloc>().state;

    final files = state.fileSystemList;

    setState(() {
      selectedPaths = {for (final f in files) f.path};
      selectionMode = true;
    });

    setHasSelection(selectedPaths.isNotEmpty);
    allSelectedNotifier.value = true;

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      _openFabMenuProgrammatically();
    });
  }

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

  void _toggleHiddenFiles() {
    controller.toggleShowHiddenFiles();
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    filesBloc.add(ToggleHiddenFiles());
  }

  void clearSearch() {
    setState(() {
      isSearching = false;
      searchQuery.value = '';
    });

    // safely remove overlay if still mounted
    searchOverlayController.hide();

    // Reload directory content when clearing search
    controller.search('');
  }

  void handleProperties() {
    if (selectedPaths.length == 1) {
      final selectedPath = selectedPaths.first;
      debugPrint("Showing details for: $selectedPath");
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

  void handleDelete() {
    confirmDelete(context, selectedPaths);
    clearSelection();
  }

  void confirmDelete(BuildContext context, Set<String> selectedPaths) {
    final pathsToRemove = selectedPaths.toList();

    final isSingle = pathsToRemove.length == 1;
    final message =
        isSingle
            ? "This action will remove file from recents"
            : "This action will remove the files from recents";

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
                      final rawName = pathsToRemove.first.split('/').last;

                      final prefixWidth = textWidth("Remove '", baseStyle);
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
                            TextSpan(text: "Remove ", style: baseStyle),
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
                          TextSpan(text: "Remove ", style: baseStyle),
                          TextSpan(
                            text: "${pathsToRemove.length} files?",
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
                        label: "Remove",
                        onPressed: () {
                          Navigator.pop(bottomSheetContext);
                          context.read<FilesBloc>().add(
                            RemoveRecentEntities(pathsToRemove),
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
}
