import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_code_editor/flutter_code_editor.dart';
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
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/features/preview/presentation/confirmation_dialog.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

final pureBlackTheme = {
  ...monokaiTheme,
  'root': const TextStyle(
    backgroundColor: Colors.black,
  ),
};

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
  bool _isFileChanged = false;

  late String _code;
  late CodeController _codeController;
  bool isMenuOpen = false;

  OverlayEntry? _searchOverlayEntry;
  String _searchQuery = '';

  List<int> _matchIndexes = [];
  int _currentMatchIndex = -1;
  final ScrollController _scrollController = ScrollController();

  // Undo / Redo
  final List<_EditorSnapshot> _undoStack = [];
  final List<_EditorSnapshot> _redoStack = [];

  bool _isInternalChange = false;

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
    _undoStack.clear();
    _redoStack.clear();

    _undoStack.add(
      _EditorSnapshot(
        code,
        const TextSelection.collapsed(offset: 0),
      ),
    );

    setState(() => _initialized = true);
  }

  void _onCodeChanged() {
    if (_isInternalChange) return;

    final currentText = _codeController.text;
    final currentSelection = _codeController.selection;

    final last = _undoStack.last;

    if (last.text != currentText) {
      _undoStack.add(
        _EditorSnapshot(currentText, currentSelection),
      );
      _redoStack.clear();
    }

    final changed = currentText != _code;
    if (changed != _isFileChanged) {
      setState(() => _isFileChanged = changed);
    }
  }

  void _undo() {
    if (_undoStack.length <= 1) return;

    _isInternalChange = true;

    final current = _undoStack.removeLast();
    _redoStack.add(current);

    final previous = _undoStack.last;

    _codeController.value = TextEditingValue(
      text: previous.text,
      selection: previous.selection,
    );

    _isInternalChange = false;
    setState(() {});
  }

  void _redo() {
    if (_redoStack.isEmpty) return;

    _isInternalChange = true;

    final next = _redoStack.removeLast();
    _undoStack.add(next);

    _codeController.value = TextEditingValue(
      text: next.text,
      selection: next.selection,
    );

    _isInternalChange = false;
    setState(() {});
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
    });

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text("Saved successfully",
            style:
                TextStyle(color: context.colorScheme.surfaceContainerLowest)),
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

    final explorerState =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    final controller = explorerState?.controller;

    final hasMatches = _matchIndexes.isNotEmpty;
    final index = _currentMatchIndex;

    final canNavPrev = hasMatches && index > 0;
    final canNavNext = hasMatches && index < _matchIndexes.length - 1;

    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(70),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Padding(
              padding: const EdgeInsets.only(top: 12, right: 16),
              child: AppBar(
                automaticallyImplyLeading: false,
                scrolledUnderElevation: 0,
                backgroundColor: Colors.transparent,
                elevation: 0,
                title: _buildTitle(controller),
                actions: !hasMatches
                    ? null
                    : [
                        Padding(
                          padding: const EdgeInsets.only(right: 8),
                          child: Center(
                            child: Text(
                              '${index + 1} of ${_matchIndexes.length}',
                              style: TextStyle(
                                color: context.colorScheme.surfaceContainerHigh,
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

            /// Divider between AppBar & body
            Divider(
              height: 1,
              thickness: 1,
              color: context.colorScheme.tertiary,
            ),
          ],
        ),
      ),
      body: SingleChildScrollView(
        controller: _scrollController,
        child: CodeTheme(
          data: CodeThemeData(styles: pureBlackTheme),
          child: CodeField(
            controller: _codeController,
            cursorColor: context.colorScheme.primaryFixed,
            textStyle: const TextStyle(
              fontFamily: 'monospace',
              fontSize: 14,
            ),
          ),
        ),
      ),
      bottomNavigationBar: _buildBottomBar(context),
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
    final state =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    return Container(
      color: context.colorScheme.secondary,
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
                iconTheme: MechanixBottomBarIconThemeData(
                  iconSize: const Size(28, 28),
                  iconColor: _undoStack.length <= 1
                      ? context.surfaceContainerHigh
                      : context.colorScheme.onSurface,
                ),
                iconPath: Images.undo,
                isDisabled: _undoStack.length <= 1,
                onPressed: _undoStack.length <= 1 ? null : _undo,
              ),
              BottomBarButton(
                iconTheme: MechanixBottomBarIconThemeData(
                  iconSize: const Size(28, 28),
                  iconColor: _redoStack.isEmpty
                      ? context.surfaceContainerHigh // disabled color
                      : context.colorScheme.onSurface, // enabled color
                ),
                iconPath: Images.redo,
                isDisabled: _redoStack.isEmpty,
                onPressed: _redoStack.isEmpty ? null : _redo,
              ),
            ],
            anchorWidgetSpacing: 0,
            anchorWidget: [
              BottomBarButton.widget(
                  widget: MechanixFilledButton(
                theme: buttonThemeData(context,
                    type: _isFileChanged
                        ? MechanixButtonType.action
                        : MechanixButtonType.disable,
                    size: const Size(94, 40)),
                label: "Save",
                onPressed: !_isFileChanged
                    ? null
                    : () async {
                        final confirmed = await _confirmSave(context);

                        if (!confirmed) {
                          _discardChanges();
                          return;
                        }

                        await _save();
                      },
              )),
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
          title: "Search",
          leading: Image.asset(
            Images.search,
            height: mechanixIconSize,
          ),
          onTap: () {
            _showSearchOverlay(context);
          },
        ),
        MechanixMenuItemsType(
          title: "Copy",
          leading: Image.asset(
            Images.copy,
            height: mechanixIconSize,
            color: context.colorScheme.onSurface,
          ),
          onTap: () {
            state?.selectedPaths = {widget.filePath};
            state?.handleCopy();
          },
        ),
        MechanixMenuItemsType(
          title: "Move",
          leading: Image.asset(
            Images.move,
            height: mechanixIconSize,
            color: context.colorScheme.onSurface,
          ),
          onTap: () {
            Navigator.pop(context);
            state?.selectedPaths = {widget.filePath};
            state?.handleMove();
          },
        ),
        MechanixMenuItemsType(
          title: "Share",
          leading: Image.asset(
            Images.share,
            height: mechanixIconSize,
            color: Colors.grey.shade600,
          ),
          disabled: true,
          onTap: null,
        ),
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

  Future<bool> _confirmSave(BuildContext context) async {
    final result = await showModalBottomSheet<bool>(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (_) => ConfirmationBottomSheet(
        filePath: widget.filePath,
      ),
    );

    return result ?? false;
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

  void _discardChanges() {
    _isInternalChange = true;

    _codeController.value = TextEditingValue(
      text: _code, // last saved content
      selection: const TextSelection.collapsed(offset: 0),
    );

    _undoStack
      ..clear()
      ..add(
        _EditorSnapshot(
          _code,
          const TextSelection.collapsed(offset: 0),
        ),
      );

    _redoStack.clear();

    _isInternalChange = false;

    setState(() {
      _isFileChanged = false;
    });
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
              cursorColor: context.colorScheme.primaryFixed,
              prefixIcon: IconWidget(
                iconPath: Images.search,
                iconColor: context.colorScheme.onSurface,
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

class _EditorSnapshot {
  final String text;
  final TextSelection selection;

  _EditorSnapshot(this.text, this.selection);
}
