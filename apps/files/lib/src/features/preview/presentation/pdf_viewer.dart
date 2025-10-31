import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:pdfrx/pdfrx.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/textInput/mechanix_text_input_theme.dart';

/// A StatefulWidget to view PDF files with search and password protection support.
class PdfViewerPage extends StatefulWidget {
  final String filePath;

  const PdfViewerPage({super.key, required this.filePath});

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
  }

  @override
  void dispose() {
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
    bool obscureText = true;
    String password = '';

    final result = await showModalBottomSheet<String>(
      context: context,
      backgroundColor: Colors.black,
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
                  padding: const EdgeInsets.all(20),
                  decoration: const BoxDecoration(
                    color: Colors.transparent,
                    borderRadius:
                        BorderRadius.vertical(top: Radius.circular(12)),
                  ),
                  child: MechanixTextInputTheme(
                    style: MechanixTextInputThemeData(
                      borderRadius: BorderRadius.circular(50),
                    ),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          'Enter password',
                          style: TextStyle(
                            fontSize: 16,
                            color: Colors.grey,
                          ),
                        ).padBottom(12),
                        Row(
                          children: [
                            Expanded(
                              child: MechanixTextInput.password(
                                isPasswordField: obscureText,
                                onChanged: (value) {
                                  password = value;
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
                                  hintText: "Password here",
                                  hintStyle:
                                      const TextStyle(color: Colors.white54),
                                  prefixIcon: const Icon(Icons.lock_outline,
                                      color: Colors.white54),
                                  prefixIconConstraints: const BoxConstraints(
                                    minWidth: 40,
                                    minHeight: 40,
                                  ),
                                  suffixIcon: Row(
                                    mainAxisSize: MainAxisSize.min,
                                    children: [
                                      IconButton(
                                        icon: Icon(
                                          obscureText
                                              ? Icons.visibility_off
                                              : Icons.visibility,
                                          color: Colors.white54,
                                        ),
                                        onPressed: () {
                                          setModalState(() {
                                            obscureText = !obscureText;
                                          });
                                        },
                                      ),
                                      Container(
                                        width: 1,
                                        height: 28,
                                        color: Colors.white24,
                                        margin: const EdgeInsets.symmetric(
                                            horizontal: 8),
                                      ),
                                      OutlinedButton(
                                        style: OutlinedButton.styleFrom(
                                                padding:
                                                    const EdgeInsets.symmetric(
                                                        vertical: 14))
                                            .copyWith(
                                          splashFactory: NoSplash
                                              .splashFactory, // Disable ripple animation
                                        ),
                                        onPressed: () {
                                          password = '';
                                          Navigator.of(bottomSheetContext)
                                              .pop();
                                        },
                                        child: const Icon(
                                          Icons.close,
                                          color: Colors.white,
                                        ),
                                      ).padRight(6),
                                    ],
                                  ),
                                  suffixIconConstraints: const BoxConstraints(
                                    minWidth: 80,
                                    minHeight: 40,
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
              ),
            );
          },
        );
      },
    );

    if (!mounted) return null;

    if (result == null) {
      Navigator.of(context).pop();
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
                enableTextSelection: true,
                backgroundColor: Colors.black,
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
    );
  }

  /// Normal app bar with file name and search icon
  PreferredSizeWidget _buildNormalAppBar() {
    return CustomAppBar(
      titleWidget:
          Text(p.basename(widget.filePath), style: context.textTheme.bodySmall),
      leftIcon: const Icon(Icons.arrow_back_ios, color: Colors.blue, size: 16),
      leftIconOnTap: () => Navigator.pop(context),
      rightIcon1: const Icon(Icons.search),
      rightIcon1OnTap: () {
        setState(() {
          isSearching = true;
          searchController.clear();
        });
      },
    );
  }

  /// App bar shown while searching, with search input and match controls
  PreferredSizeWidget _buildSearchAppBar() {
    final matchLabel = matches.isNotEmpty && currentMatchIndex != null
        ? '${currentMatchIndex! + 1} of ${matches.length}'
        : '';

    return CustomAppBar(
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
                // Start live search
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
      leftIcon: const Icon(Icons.arrow_back_ios, color: Colors.blue, size: 16),
      leftIconOnTap: () {
        setState(() {
          isSearching = false;
          searchController.clear();
          _searcher.startTextSearch('', caseInsensitive: true); // clear matches
        });
      },
      rightIcon1: const Icon(Icons.navigate_before),
      rightIcon1OnTap: _prev,
      rightIcon2: const Icon(Icons.navigate_next),
      rightIcon2OnTap: _next,
    );
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
