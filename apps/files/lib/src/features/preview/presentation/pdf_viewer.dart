import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:pdfrx/pdfrx.dart';
import 'package:path/path.dart' as p;

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
    final controller = TextEditingController();

    final result = await showDialog<String>(
      context: context,
      barrierDismissible: false,
      builder: (context) {
        return Dialog(
          backgroundColor: Colors.black,
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
          child: Padding(
            padding: const EdgeInsets.fromLTRB(24, 24, 24, 16),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'This file is protected',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: controller,
                  obscureText: true,
                  style: const TextStyle(color: Colors.white),
                  decoration: const InputDecoration(
                    labelText: 'Password',
                    labelStyle: TextStyle(color: Colors.white70),
                    enabledBorder: UnderlineInputBorder(
                      borderSide: BorderSide(color: Colors.white38),
                    ),
                    focusedBorder: UnderlineInputBorder(
                      borderSide: BorderSide(color: Colors.blueAccent),
                    ),
                  ),
                ),
                const SizedBox(height: 24),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    TextButton(
                      onPressed: () => Navigator.of(context).pop(null),
                      child: const Text('Cancel',
                          style: TextStyle(color: Colors.white)),
                    ),
                    const SizedBox(width: 16),
                    ElevatedButton(
                      onPressed: () =>
                          Navigator.of(context).pop(controller.text),
                      style: ElevatedButton.styleFrom(
                        foregroundColor: Colors.white,
                        backgroundColor: Colors.blueAccent,
                        padding: const EdgeInsets.symmetric(
                            horizontal: 20, vertical: 12),
                      ),
                      child: const Text('Open'),
                    ),
                  ],
                ),
              ],
            ),
          ),
        );
      },
    );

    if (!mounted) return null;

    // If dialog was dismissed, optionally close the PDF page
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
      title: p.basename(widget.filePath),
      leftIcon: const Icon(Icons.arrow_back),
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
      leftIcon: const Icon(Icons.arrow_back),
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
