import 'dart:async';
import 'dart:io' show FileSystemEntity, File;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/pressable_icon.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:path/path.dart' as p;
import 'package:pdfrx/pdfrx.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

/// A StatefulWidget to view PDF files with search and password protection support.
class PdfViewerPage extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;
  FileExplorerPageState? state;

  PdfViewerPage(
      {super.key,
      required this.rootContext,
      required this.filePath,
      this.state});

  @override
  State<PdfViewerPage> createState() => _PdfViewerPageState();
}

class _PdfViewerPageState extends State<PdfViewerPage> {
  final PdfViewerController _controller = PdfViewerController();
  late final PdfTextSearcher _searcher;
  bool isSearching = false;
  final TextEditingController searchController = TextEditingController();
  bool passwordWasIncorrect = false;

  int? currentMatchIndex;
  List<PdfTextRangeWithFragments> matches = [];

  bool isMenuOpen = false;

  OverlayEntry? _searchOverlayEntry;
  int _currentPage = 1;
  int _pageCount = 0;

  OverlayEntry? _pageBubbleOverlay;
  Timer? _hideBubbleTimer;

  @override
  void initState() {
    super.initState();

    // Initialize text searcher and listen for changes in search results
    _searcher = PdfTextSearcher(_controller);
    _searcher.addListener(() {
      setState(() {
        currentMatchIndex = _searcher.currentIndex;
        matches = _searcher.matches;
      });
    });

    _controller.addListener(_onPageChanged);
  }

  @override
  void dispose() {
    _hideBubbleTimer?.cancel();
    _pageBubbleOverlay?.remove();
    _controller.removeListener(_onPageChanged);

    _searcher.dispose();
    super.dispose();
  }

  /// Navigate to next search match
  void _nextMatch() async {
    final index = await _searcher.goToNextMatch();
    setState(() {
      currentMatchIndex = index;
    });
  }

  /// Navigate to previous search match
  void _prevMatch() async {
    final index = await _searcher.goToPrevMatch();
    setState(() {
      currentMatchIndex = index;
    });
  }

  /// Prompts the user for a password if the PDF is password-protected
  /// TODO: Implement password validation logic for wrong password
  Future<String?> _passwordProvider() async {
    String password = '';
    bool obscureText = true;

    final result = await showModalBottomSheet<String>(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return StatefulBuilder(
          builder: (context, setModalState) {
            return RawKeyboardListener(
              focusNode: FocusNode(),
              autofocus: true,
              onKey: (event) {
                if (event.isKeyPressed(LogicalKeyboardKey.enter)) {
                  Navigator.of(bottomSheetContext).pop(password);
                }
              },
              child: Padding(
                padding: EdgeInsets.only(
                  bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
                ),
                child: Container(
                  height: 90,
                  decoration: const BoxDecoration(
                    borderRadius:
                        BorderRadius.vertical(top: Radius.circular(12)),
                  ),
                  child: Center(
                    child: MechanixTextInput.password(
                      autofocus: true,
                      isPasswordField: obscureText,
                      cursorColor: context.colorScheme.primaryFixed,
                      prefixIcon: const IconWidget(iconPath: Images.lock),
                      hintText: 'Enter PDF password',
                      onChanged: (value) {
                        password = value;
                      },
                      anchorWidget: Padding(
                        padding: const EdgeInsets.only(left: 2.0, right: 2.0),
                        child: IconButton(
                          icon: Icon(
                            Icons.close,
                            color: context.colorScheme.onSurface,
                            size: 24,
                          ),
                          onPressed: () {
                            password = '';
                            Navigator.of(bottomSheetContext).pop();
                          },
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            );
          },
        );
      },
    );

    if (!mounted) return null;

    if (result == null || result.isEmpty) {
      Navigator.of(context).pop(); // close PDF if cancelled
      return null;
    }

    return result;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: _buildNormalAppBar(),
        body: Stack(
          children: [
            // Display the PDF viewer
            PdfViewer.file(
              widget.filePath,
              controller: _controller,
              passwordProvider: _passwordProvider,
              firstAttemptByEmptyPassword: true,
              params: PdfViewerParams(
                // backgroundColor: context.colorScheme.surface,
                enableTextSelection: true,
                maxScale: 4.0,
                minScale: 1.0,
                errorBannerBuilder: (context, error, stack, document) {
                  return const SizedBox.shrink();
                },
                pageOverlaysBuilder: (context, pageRect, page) {
                  return [
                    CustomPaint(
                      size: pageRect.size,
                      painter: _PdfSearchHighlightPainter(
                        searcher: _searcher,
                        page: page,
                      ),
                    ),
                  ];
                },
              ),
              initialPageNumber: 1,
            ),
          ],
        ),
        bottomNavigationBar: _buildBottomBar(context));
  }

  void _onPageChanged() {
    final page = _controller.pageNumber;
    final count = _controller.pageCount ?? 0;

    if (page == null || page == _currentPage) return;

    setState(() {
      _currentPage = page;
      _pageCount = count;
    });

    _showPageBubble();
  }

  void _showPageBubble() {
    _pageBubbleOverlay?.remove();

    final overlay = Overlay.of(context);
    if (overlay == null) return;

    _pageBubbleOverlay = OverlayEntry(
      builder: (_) => Positioned(
        bottom: 80, // above bottom bar
        left: 0,
        right: 0,
        child: IgnorePointer(
          child: Center(
            child: AnimatedOpacity(
              opacity: 1,
              duration: const Duration(milliseconds: 150),
              child: Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 6,
                ),
                decoration: BoxDecoration(
                  color: context.colorScheme.surface.withOpacity(0.7),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Text(
                  'Page $_currentPage of $_pageCount',
                  style: TextStyle(
                    color: context.colorScheme.onSurface,
                    fontSize: 12,
                    fontWeight: FontWeight.w500,
                    decoration: TextDecoration.none,
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );

    overlay.insert(_pageBubbleOverlay!);

    _hideBubbleTimer?.cancel();
    _hideBubbleTimer = Timer(const Duration(seconds: 2), _hidePageBubble);
  }

  void _hidePageBubble() {
    _pageBubbleOverlay?.remove();
    _pageBubbleOverlay = null;
  }

  /// Normal app bar with file name and search icon
  PreferredSizeWidget _buildNormalAppBar() {
    final explorerState = widget.state;

    final controller = explorerState?.controller;

    final hasMatches = matches.isNotEmpty;
    final index = currentMatchIndex ?? 0;

    final canNavPrev = hasMatches && index > 0;
    final canNavNext = hasMatches && index < matches.length - 1;

    return PreferredSize(
      preferredSize: const Size.fromHeight(60),
      child: Padding(
        padding: const EdgeInsets.only(top: 6, left: 16, right: 16, bottom: 12),
        child: AppBar(
          automaticallyImplyLeading: false,
          scrolledUnderElevation: 0,
          title: _buildTitle(controller),
          backgroundColor: Colors.transparent,
          elevation: 0,
          actions: !hasMatches
              ? null
              : [
                  // Match counter
                  Padding(
                    padding: const EdgeInsets.only(right: 8),
                    child: Center(
                      child: Text(
                        hasMatches
                            ? '${index + 1} of ${matches.length}'
                            : '0 of 0',
                        style: TextStyle(
                          color: context.colorScheme.onSurfaceVariant,
                          fontSize: 18,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                    ),
                  ),

                  searchNavButton(
                    icon: Icons.keyboard_arrow_up,
                    onTap: canNavPrev ? _prevMatch : null,
                    context: context,
                  ),

                  searchNavButton(
                    icon: Icons.keyboard_arrow_down,
                    onTap: canNavNext ? _nextMatch : null,
                    context: context,
                  ),

                  const SizedBox(width: 8),
                ],
        ),
      ),
    );
  }

  Widget _buildTitle(FileManagerController? controller) {
    if (controller == null) {
      return MiddleEllipsisText(
        p.basename(widget.filePath),
        style: previewTitleStyle(context),
      );
    }

    return ValueListenableBuilder<List<FileSystemEntity>>(
      valueListenable: controller.paginatedEntities,
      builder: (_, __, ___) {
        final title = controller.getDisplayName(File(widget.filePath));
        return MiddleEllipsisText(
          title,
          style: previewTitleStyle(context),
        );
      },
    );
  }

  Widget _buildBottomBar(BuildContext context) {
    final state = widget.state;

    return MechanixBottomBar(
      theme: MechanixBottomBarThemeData(
          decoration: BoxDecoration(
              color: context.colorScheme.secondaryContainer,
              borderRadius: const BorderRadius.only(
                  topLeft: Radius.circular(8), topRight: Radius.circular(8)))),
      leadingWidget: [
        BottomBarButton.widget(
            widget: Padding(
          padding: const EdgeInsets.only(left: 4),
          child: DecoratedPressableIcon(
            iconPath: Images.back,
            onTap: () => Navigator.pop(context),
          ),
        )),
      ],
      centerWidgetSpacing: 30,
      centerWidget: [
        BottomBarButton.widget(
          widget: DecoratedPressableIcon(
            iconPath: Images.search,
            onTap: () {
              showPdfSearchBottomSheet(context);
            },
          ),
        ),
        BottomBarButton.widget(
          widget: DecoratedPressableIcon(
            iconPath: Images.copy,
            onTap: () {
              state?.selectedPaths = {widget.filePath};
              state?.handleCopy();
            },
          ),
        ),
        BottomBarButton.widget(
          widget: DecoratedPressableIcon(
            iconPath: Images.move,
            onTap: () {
              Navigator.pop(context);
              state?.selectedPaths = {widget.filePath};
              state?.handleMove();
            },
          ),
        ),
        const BottomBarButton.widget(
          widget: DecoratedPressableIcon(
            iconPath: Images.share,
            isDisabled: true, // TODO: add share functionality
            onTap: null,
          ),
        ),
      ],
      anchorWidget: [
        BottomBarButton.widget(widget: buildActionsMenu(context)),
      ],
    );
  }

  Widget buildActionsMenu(BuildContext context) {
    final offset = const Offset(-8, -14);
    final state = widget.state;

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: IconWidget(
          iconPath: Images.dots,
          iconWidth: 28,
          iconHeight: 28,
          iconColor: isMenuOpen
              ? context.colorScheme.primaryFixed
              : context.colorScheme.onSurface),
      openMenu: () {
        setState(() => isMenuOpen = true);
      },
      closeMenu: () {
        setState(() => isMenuOpen = false);
      },
      items: [
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.rename,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          title: 'Rename',
          onTap: () async {
            final oldPath = widget.filePath;

            // Wait for rename result
            final newPath = await state?.showRenameSheet(
              initialName: p.basename(oldPath),
            );

            // If user canceled : do nothing
            if (newPath == null) return;

            // Also update widget.filePath for correct behavior
            widget.filePath = newPath;
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
            state?.showDetailsDialog(widget.rootContext, widget.filePath);
          },
        ),
        MechanixMenuItemsType(
          title: "Delete",
          leading: Image.asset(
            Images.delete,
            color: context.colorScheme.onSurface,
            height: mechanixIconSize,
          ),
          onTap: () {
            Navigator.pop(context);
            state?.confirmDelete(widget.rootContext, {widget.filePath});
          },
        ),
      ],
    ).padRight(8);
  }

  void showPdfSearchBottomSheet(BuildContext context) {
    final overlay = Overlay.of(context);
    if (overlay == null) return;

    _searchOverlayEntry?.remove();

    _searchOverlayEntry = OverlayEntry(
      builder: (ctx) => Positioned(
        left: 0,
        right: 0,
        bottom: 0,
        child: Material(
          color: Colors.transparent,
          child: SizedBox(
            height: 90,
            child: MechanixTextInput.search(
              cursorColor: context.colorScheme.primaryFixed,
              autofocus: false,
              prefixIcon: IconWidget(
                iconPath: Images.search,
                iconColor: context.colorScheme.onSurface,
                iconHeight: 24,
                iconWidth: 24,
              ),
              hintText: "Search in PDF",
              onChanged: _onPdfSearchChanged,
              onClear: _clearPdfSearch,
            ),
          ),
        ),
      ),
    );

    overlay.insert(_searchOverlayEntry!);
  }

  void _onPdfSearchChanged(String query) {
    if (query.trim().isEmpty || query.trim().length < 3) {
      _searcher.startTextSearch('');
      setState(() {
        matches = [];
        currentMatchIndex = null;
      });
      return;
    }

    if (query.trim().length > 2) {
      _searcher.startTextSearch(
        query,
        caseInsensitive: true,
        goToFirstMatch: true,
      );
    }
  }

  void _clearPdfSearch() {
    _searcher.startTextSearch('');

    setState(() {
      matches = [];
      currentMatchIndex = null;
    });

    _searchOverlayEntry?.remove();
    _searchOverlayEntry = null;
  }
}

/// Custom painter to highlight matching search results in PDF pages
class _PdfSearchHighlightPainter extends CustomPainter {
  final PdfTextSearcher searcher;
  final PdfPage page;

  _PdfSearchHighlightPainter({
    required this.searcher,
    required this.page,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final pageRect = Offset.zero & size;
    searcher.pageTextMatchPaintCallback(canvas, pageRect, page);
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}
