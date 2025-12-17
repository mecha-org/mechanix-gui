import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_code_editor/flutter_code_editor.dart';
import 'package:flutter_highlight/flutter_highlight.dart';
import 'package:flutter_highlight/themes/monokai.dart';
import 'package:highlight/highlight_core.dart';
import 'package:highlight/languages/dart.dart';
import 'package:highlight/languages/json.dart';
import 'package:highlight/languages/plaintext.dart';
import 'package:highlight/languages/sql.dart';
import 'package:highlight/languages/yaml.dart';
import 'package:highlight/languages/python.dart';
import 'package:highlight/languages/java.dart';
import 'package:highlight/languages/cpp.dart';
import 'package:highlight/languages/ruby.dart';
import 'package:highlight/languages/xml.dart';
import 'package:highlight/languages/rust.dart';
import 'package:highlight/languages/javascript.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class CodeEditorPage extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;

  CodeEditorPage(
      {super.key, required this.rootContext, required this.filePath});

  @override
  State<CodeEditorPage> createState() => _CodeEditorPageState();
}

class _CodeEditorPageState extends State<CodeEditorPage> {
  bool _initialized = false;
  bool _isEditing = false;
  bool _isFileChanged = false;

  late String _code;
  late CodeController _codeController;
  String title = '';
  bool isMenuOpen = false;

  OverlayEntry? _searchOverlayEntry;
  String _searchQuery = '';

  List<int> _matchIndexes = [];
  int _currentMatchIndex = -1;
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _initEditor();
  }

  Future<void> _initEditor() async {
    final code = await File(widget.filePath).readAsString();
    final ext = p.extension(widget.filePath).replaceAll('.', '');

    final language = _getLanguage(ext);
    _code = code;

    _codeController = CodeController(
      text: code,
      language: language,
    );

    _codeController.addListener(_onCodeChanged);

    setState(() => _initialized = true);
  }

  void _onCodeChanged() {
    final changed = _codeController.text != _code;

    if (changed != _isFileChanged) {
      setState(() {
        _isFileChanged = changed;
      });
    }
  }

  Mode _getLanguage(String ext) {
    switch (ext) {
      case 'dart':
        return dart;
      case 'json':
        return json;
      case 'yaml':
      case 'yml':
        return yaml;
      case 'py':
        return python;
      case 'java':
        return java;
      case 'c':
      case 'cpp':
        return cpp;
      case 'js':
        return javascript;
      case 'sql':
        return sql;
      // case 'ini':
      // case 'toml':
      // return ini; // NOTE: Syntax highlighting modes for 'ini' and 'toml' are currently not supported. Using 'dart' as a fallback for these
      case 'rb':
        return ruby;
      case 'xml':
        return xml;
      case 'rs':
        return rust;
      case 'txt':
        return plaintext;
      default:
        return dart; // fallback
    }
  }

  String _getLanguageName(String ext) {
    switch (ext) {
      case 'dart':
      case 'json':
      case 'yaml':
      case 'py':
      case 'java':
      case 'c':
      case 'cpp':
      case 'js':
      case 'sql':
      // case 'ini':
      // case 'toml': // NOTE: Fallback to 'dart' for unsupported languages
      case 'rb':
      case 'xml':
      case 'rs':
      case 'yml':
      case 'txt':
        return ext;
      default:
        return 'dart'; // fallback
    }
  }

  Future<void> _save() async {
    final updated = _codeController.text;
    await File(widget.filePath).writeAsString(updated);

    setState(() {
      _code = updated;
      _isFileChanged = false;
      _isEditing = false;
    });

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: const Text("Saved successfully",
            style: TextStyle(color: Colors.white)),
        duration: const Duration(seconds: 2),
        backgroundColor: Colors.grey[800],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (!_initialized) {
      return const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      );
    }

    title = p.basename(widget.filePath);
    final hasMatches = _matchIndexes.isNotEmpty;
    final index = _currentMatchIndex;

    final canNavPrev = hasMatches && index > 0;
    final canNavNext = hasMatches && index < _matchIndexes.length - 1;

    return Scaffold(
        appBar: PreferredSize(
          preferredSize: const Size.fromHeight(60),
          child: Padding(
            padding: const EdgeInsets.only(top: 18, right: 16),
            child: AppBar(
              automaticallyImplyLeading: false,
              scrolledUnderElevation: 0,
              title: Text(
                title,
                style: TextStyle(
                  color: const Color(0xFFD2D2D2),
                  fontSize: 20,
                  fontWeight: FontWeight.w600,
                  fontFamily: Theme.of(context)
                      .extension<FilesTheme>()!
                      .defaultFontFamily,
                ),
                overflow: TextOverflow.ellipsis,
              ),
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
                                ? '${index + 1} of ${_matchIndexes.length}'
                                : '0 of 0',
                            style: TextStyle(
                              color: FilesThemeConstants.labelColor,
                              fontSize: 18,
                              fontWeight: FontWeight.w500,
                              fontFamily: Theme.of(context)
                                  .extension<FilesTheme>()!
                                  .defaultFontFamily,
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
        ),
        body: Padding(
            padding: const EdgeInsets.all(12),
            child: _isEditing
                ? SingleChildScrollView(
                    child: CodeTheme(
                      data: CodeThemeData(styles: monokaiTheme),
                      child:
                          // TextField(
                          //   controller: _codeController,
                          //   cursorColor: FilesThemeConstants.primaryColor,
                          //   contextMenuBuilder: (context, editableTextState) {
                          //     return AdaptiveTextSelectionToolbar(
                          //       anchors: editableTextState.contextMenuAnchors,
                          //       children: const [SelectionOptions()],
                          //     );
                          //   },
                          // )

                          CodeField(
                        controller: _codeController,
                        textStyle: const TextStyle(fontFamily: 'monospace'),
                      ),
                    ),
                  )
                : LayoutBuilder(
                    builder: (context, constraints) {
                      return SingleChildScrollView(
                        controller: _scrollController,
                        child: SizedBox(
                          width: constraints.maxWidth,
                          child: InteractiveViewer(
                            constrained: true,
                            minScale: 1,
                            maxScale: 4,
                            child: Stack(
                              children: [
                                HighlightView(
                                  _code,
                                  language: _getLanguageName(
                                    p
                                        .extension(widget.filePath)
                                        .replaceAll('.', ''),
                                  ),
                                  theme: monokaiTheme,
                                  padding: const EdgeInsets.all(12),
                                  textStyle: const TextStyle(
                                    fontFamily: 'monospace',
                                    fontSize: 14,
                                    height: 1.4,
                                  ),
                                ),
                                if (_searchQuery.isNotEmpty)
                                  Positioned.fill(
                                    child: IgnorePointer(
                                      child: CustomPaint(
                                        painter: _SearchHighlightPainter(
                                          code: _code,
                                          search: _searchQuery,
                                          textStyle: const TextStyle(
                                            fontFamily: 'monospace',
                                            fontSize: 14,
                                            height: 1.4,
                                          ),
                                          padding: const EdgeInsets.all(12),
                                        ),
                                      ),
                                    ),
                                  ),
                              ],
                            ),
                          ),
                        ),
                      );
                    },
                  )),
        bottomNavigationBar: _isEditing
            ? _buildEditingBottomBar(context)
            : _buildBottomBar(context));
  }

  Widget _buildEditingBottomBar(BuildContext context) {
    final state =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    return Container(
      color: Colors.grey.shade900,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          MechanixBottomBar(
            theme: const MechanixBottomBarThemeData(),
            leadingWidget: [
              BottomBarButton(
                iconTheme: const MechanixBottomBarIconThemeData(
                    padding: EdgeInsets.only(left: 12), iconSize: Size(28, 28)),
                iconPath: Images.back,
                onPressed: () {
                  setState(() => _isEditing = false);
                  _buildBottomBar(context);
                },
              ),
            ],
            anchorWidget: [
              BottomBarButton.widget(
                widget: MechanixFilledButton(
                  theme: buttonThemeData(context,
                      type: MechanixButtonType.cancel,
                      size: const Size(94, 40)),
                  label: "Cancel",
                  onPressed: () {
                    setState(() => _isEditing = false);
                    _buildBottomBar(context);
                  },
                ),
              ),
              BottomBarButton.widget(
                  widget: Padding(
                      padding: const EdgeInsets.only(right: 16),
                      child: MechanixFilledButton(
                        theme: buttonThemeData(context,
                            type: _isFileChanged
                                ? MechanixButtonType.action
                                : MechanixButtonType.disable,
                            size: const Size(94, 40)),
                        label: "Save",
                        onPressed: !_isFileChanged
                            ? null
                            : () {
                                _save();
                                setState(() => _isEditing = false);
                                _buildBottomBar(context);
                              },
                      ))),
            ],
          ),
        ],
      ),
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
            theme: const MechanixBottomBarThemeData(),
            leadingWidget: [
              BottomBarButton(
                iconTheme: const MechanixBottomBarIconThemeData(
                    padding: EdgeInsets.only(left: 12), iconSize: Size(28, 28)),
                iconPath: Images.back,
                onPressed: () => Navigator.pop(context),
              ),
            ],
            centerWidgetSpacing: 30,
            centerWidget: [
              BottomBarButton(
                iconWidget: IconWidget(
                  iconPath: Images.search,
                  iconHeight: 28,
                  iconWidth: 28,
                  iconColor: _isEditing
                      ? FilesThemeConstants.disableColor
                      : FilesThemeConstants.titleTextColor,
                ),
                isDisabled: _isEditing,
                onPressed: () {
                  _showSearchOverlay(context);
                },
              ),
              BottomBarButton(
                iconTheme: const MechanixBottomBarIconThemeData(
                    iconSize: Size(28, 28)),
                iconPath: Images.copy,
                onPressed: () {
                  state?.selectedPaths = {widget.filePath};
                  state?.handleCopy();
                },
              ),
              BottomBarButton(
                iconTheme: const MechanixBottomBarIconThemeData(
                    iconSize: Size(28, 28)),
                iconPath: Images.move,
                onPressed: () {
                  Navigator.pop(context);
                  state?.selectedPaths = {widget.filePath};
                  state?.handleMove();
                },
              ),
              BottomBarButton(
                iconTheme: const MechanixBottomBarIconThemeData(
                    iconSize: Size(28, 28)),
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
          iconHeight: 28,
          iconWidth: 28,
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
          title: "Edit",
          leading: IconWidget(
            iconPath: Images.edit,
            iconColor: _isEditing
                ? FilesThemeConstants.primaryColor
                : FilesThemeConstants.titleTextColor,
          ),
          isSelected: _isEditing,
          onTap: () {
            if (!_isEditing) {
              setState(() {
                isMenuOpen = false;
                _isEditing = true;
              });
            }
          },
        ),
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

  void _onSearchChanged(String query) {
    final trimmed = query.trim();

    _searchQuery = trimmed;
    _matchIndexes.clear();
    _currentMatchIndex = -1;

    if (query.trim().isEmpty) {
      setState(() {});
      return;
    }

    // Disable search under 3 chars
    if (trimmed.length > 2) {
      final text = _codeController.text.toLowerCase();
      final search = trimmed.toLowerCase();

      int index = 0;
      while ((index = text.indexOf(search, index)) != -1) {
        _matchIndexes.add(index);
        index += search.length;
      }

      if (_matchIndexes.isNotEmpty) {
        _currentMatchIndex = 0;
        _jumpToMatch();
      }

      setState(() {});
    }
  }

  void _jumpToMatch() {
    if (_matchIndexes.isEmpty) return;

    final matchIndex = _matchIndexes[_currentMatchIndex];

    // Count how many lines before the match
    final beforeText = _code.substring(0, matchIndex);
    final lineIndex = '\n'.allMatches(beforeText).length;

    const double lineHeight = 14 * 1.4; // fontSize * height
    const double topPadding = 12;

    final offset = lineIndex * lineHeight + topPadding;

    _scrollController.animateTo(
      offset,
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeOut,
    );
  }

  void _nextMatch() {
    if (_matchIndexes.isEmpty) return;

    setState(() {
      _currentMatchIndex = (_currentMatchIndex + 1) % _matchIndexes.length;
    });

    _jumpToMatch();
  }

  void _prevMatch() {
    if (_matchIndexes.isEmpty) return;

    setState(() {
      _currentMatchIndex = (_currentMatchIndex - 1 + _matchIndexes.length) %
          _matchIndexes.length;
    });

    _jumpToMatch();
  }

  void _showSearchOverlay(BuildContext context) {
    final overlay = Overlay.of(context);
    if (overlay == null) return;

    _searchOverlayEntry?.remove();

    _searchOverlayEntry = OverlayEntry(
      builder: (_) => Positioned(
        left: 0,
        right: 0,
        bottom: 0,
        child: Material(
          color: Colors.transparent,
          child: SizedBox(
            height: 60,
            child: MechanixTextInput.search(
              autofocus: false,
              hintText: "Search in file",
              cursorColor:
                  Theme.of(context).extension<FilesTheme>()!.primaryColor,
              prefixIcon: const IconWidget(
                iconPath: Images.search,
                iconColor: Color(0xFFD2D2D2),
                iconHeight: 24,
                iconWidth: 24,
              ),
              onChanged: _onSearchChanged,
              onClear: _clearSearch,
            ),
          ),
        ),
      ),
    );

    overlay.insert(_searchOverlayEntry!);
  }

  void _clearSearch() {
    _searchQuery = '';
    _matchIndexes.clear();
    _currentMatchIndex = -1;

    _searchOverlayEntry?.remove();
    _searchOverlayEntry = null;

    setState(() {});
  }
}

class _SearchHighlightPainter extends CustomPainter {
  final String code;
  final String search;
  final TextStyle textStyle;
  final EdgeInsets padding;

  _SearchHighlightPainter({
    required this.code,
    required this.search,
    required this.textStyle,
    required this.padding,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (search.isEmpty) return;

    final paint = Paint()..color = Colors.yellow.withOpacity(0.35);

    final textPainter = TextPainter(
      textDirection: TextDirection.ltr,
      textScaleFactor: 1.0,
    );

    final lines = code.split('\n');
    final query = search.toLowerCase();
    final lineHeight = textStyle.fontSize! * textStyle.height!;

    double y = padding.top;

    for (final line in lines) {
      final lower = line.toLowerCase();
      int start = 0;

      while (true) {
        final index = lower.indexOf(query, start);
        if (index == -1) break;

        final before = line.substring(0, index);
        final match = line.substring(index, index + search.length);

        textPainter.text = TextSpan(text: before, style: textStyle);
        textPainter.layout();

        final x = padding.left + textPainter.width;

        textPainter.text = TextSpan(text: match, style: textStyle);
        textPainter.layout();

        canvas.drawRect(
          Rect.fromLTWH(x, y, textPainter.width, lineHeight),
          paint,
        );

        start = index + search.length;
      }

      y += lineHeight;
    }
  }

  @override
  bool shouldRepaint(covariant _SearchHighlightPainter old) =>
      old.search != search || old.code != code;
}
