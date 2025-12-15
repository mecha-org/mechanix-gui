import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:pdfrx/pdfrx.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';
import 'package:widgets/widgets/textInput/mechanix_text_input_theme.dart';

/// A StatefulWidget to view PDF files with search and password protection support.
class PdfViewerPage extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;

  PdfViewerPage({super.key, required this.rootContext, required this.filePath});

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
  String title = '';

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
  void _next() async {
    final index = await _searcher.goToNextMatch();
    setState(() {
      currentMatchIndex = index;
    });
  }

  /// Navigate to previous search match
  void _prev() async {
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
                  height: 60,
                  decoration: const BoxDecoration(
                    color: Color(0xFF151515),
                    borderRadius:
                        BorderRadius.vertical(top: Radius.circular(12)),
                  ),
                  child: Center(
                    child: MechanixTextInput.password(
                      autofocus: true,
                      isPasswordField: obscureText,
                      theme: MechanixTextInputThemeData(
                        fillColor: const Color(0xFF151515),
                        borderSide: const BorderSide(color: Color(0xFF151515)),
                        focusedBorderSide:
                            const BorderSide(color: Color(0xFF151515)),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      cursorColor: Theme.of(context)
                          .extension<FilesTheme>()!
                          .primaryColor,
                      prefixIcon:
                          const IconWidget.fromIconData(icon: Icon(Icons.lock)),
                      hintText: 'Enter PDF password',
                      onChanged: (value) {
                        password = value;
                      },
                      anchorWidget: Padding(
                        padding: const EdgeInsets.only(left: 4.0, right: 4.0),
                        child: IconButton(
                          icon: const Icon(
                            Icons.close,
                            color: Colors.white70,
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
        appBar: isSearching ? _buildSearchAppBar() : _buildNormalAppBar(),
        body: ContainerWidget(
          child: Stack(
            children: [
              // Display the PDF viewer
              PdfViewer.file(
                widget.filePath,
                controller: _controller,
                passwordProvider: _passwordProvider,
                firstAttemptByEmptyPassword: true,
                params: PdfViewerParams(
                  backgroundColor: Colors.black,
                  enableTextSelection: true,
                  maxScale: 4.0,
                  minScale: 1.0,

                  // Optional: highlight search matches
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
                  color: Colors.black.withOpacity(0.7),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Text(
                  'Page $_currentPage of $_pageCount',
                  style: const TextStyle(
                    color: Colors.white70,
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
    title = title = p.basename(widget.filePath);

    return PreferredSize(
      preferredSize: const Size.fromHeight(60),
      child: Padding(
        padding: const EdgeInsets.only(top: 18, left: 16, right: 16),
        child: AppBar(
          automaticallyImplyLeading: false,
          scrolledUnderElevation: 0,
          title: Text(
            title,
            style: TextStyle(
              color: const Color(0xFFD2D2D2),
              fontSize: 20,
              fontWeight: FontWeight.w600,
              fontFamily:
                  Theme.of(context).extension<FilesTheme>()!.defaultFontFamily,
            ),
            overflow: TextOverflow.ellipsis,
          ),
          backgroundColor: Colors.transparent,
          elevation: 0,
        ),
      ),
    );
  }

  /// App bar shown while searching, with search input and match controls
  PreferredSizeWidget _buildSearchAppBar() {
    final matchLabel = matches.isNotEmpty && currentMatchIndex != null
        ? '${currentMatchIndex! + 1} of ${matches.length}'
        : '';

    return MechanixNavigationBar(
      theme: const MechanixNavigationBarThemeData(titleSpacing: 0),
      leadingWidget: IconButton(
        icon: const Icon(Icons.arrow_back_ios, color: Colors.blue, size: 16),
        onPressed: () {
          setState(() {
            isSearching = false;
            searchController.clear();
            _searcher.startTextSearch('',
                caseInsensitive: true); // clear matches
          });
        },
        highlightColor: Colors.transparent,
      ),
      titleWidget: Row(
        children: [
          // Search input field
          Expanded(
            child: TextField(
              controller: searchController,
              autofocus: true,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                hintText: 'Search',
                hintStyle: TextStyle(color: Colors.white70),
                border: InputBorder.none,
              ),
              onChanged: (value) {
                _searcher.startTextSearch(
                  value,
                  caseInsensitive: true,
                  goToFirstMatch: true,
                );
              },
            ),
          ),

          // Match count label
          if (matchLabel.isNotEmpty)
            Padding(
              padding: const EdgeInsets.only(left: 8.0),
              child: Text(
                matchLabel,
                style: const TextStyle(color: Colors.white70, fontSize: 14),
              ),
            ),
        ],
      ),
      actionWidgets: [
        IconButton(
          icon: const Icon(Icons.navigate_before, color: Colors.white),
          onPressed: _prev,
        ),
        IconButton(
          icon: const Icon(Icons.navigate_next, color: Colors.white),
          onPressed: _next,
        ),
      ],
    );
  }

  Widget _buildBottomBar(BuildContext context) {
    final state =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    return Container(
      color: Colors.grey.shade900,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          MechanixBottomBar(
            leadingWidget: [
              BottomBarButton(
                iconTheme: const MechanixBottomBarIconThemeData(
                    padding: EdgeInsets.only(left: 12)),
                iconPath: Images.back,
                onPressed: () => Navigator.pop(context),
              ),
            ],
            centerWidgetSpacing: 30,
            centerWidget: [
              BottomBarButton(
                iconPath: Images.search,
                onPressed: () {
                  showPdfSearchBottomSheet(context);
                },
              ),
              BottomBarButton(
                iconPath: Images.copy,
                onPressed: () {
                  state?.selectedPaths = {widget.filePath};
                  state?.handleCopy();
                },
              ),
              BottomBarButton(
                iconPath: Images.move,
                onPressed: () {
                  Navigator.pop(context);
                  state?.selectedPaths = {widget.filePath};
                  state?.handleMove();
                },
              ),
              BottomBarButton(
                iconWidget: IconWidget(
                  iconPath: Images.share,
                  iconColor: Colors.grey.shade600,
                ),
                onPressed: () {},
                isDisabled: true, //TODO : add share functionality
              ),
            ],
            anchorWidget: [
              BottomBarButton.widget(widget: buildActionsMenu(context)),
            ],
          ),
        ],
      ),
    );
  }

  Widget buildActionsMenu(BuildContext context) {
    final offset = const Offset(-8, -14);
    final state =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: IconWidget(
          iconPath: Images.dots,
          iconColor: isMenuOpen
              ? Theme.of(context).extension<FilesTheme>()!.primaryColor
              : Colors.white70),
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
            color: Colors.white70,
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

            // If rename succeeded : update title + filepath
            setState(() {
              title = p.basename(newPath);
            });

            // Also update widget.filePath for correct behavior
            widget.filePath = newPath;
          },
        ),
        MechanixMenuItemsType(
          title: "Properties",
          leading: Image.asset(
            Images.info,
            color: Colors.white70,
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
            color: Colors.white70,
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
            height: 60,
            child: MechanixTextInput.search(
              theme: MechanixTextInputThemeData(
                fillColor: const Color(0xFF151515),
                borderSide: const BorderSide(color: Color(0xFF151515)),
                focusedBorderSide: const BorderSide(color: Color(0xFF151515)),
                borderRadius: BorderRadius.circular(8),
              ),
              cursorColor:
                  Theme.of(context).extension<FilesTheme>()!.primaryColor,
              autofocus: true,
              prefixIcon: const IconWidget(
                iconPath: Images.search,
                iconColor: Color(0xFFD2D2D2),
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
    if (query.trim().isEmpty) {
      setState(() {
        matches = [];
        currentMatchIndex = null;
      });
      return;
    }

    _searcher.startTextSearch(
      query,
      caseInsensitive: true,
      goToFirstMatch: true,
    );
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
