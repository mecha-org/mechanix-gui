import 'dart:convert';
import 'dart:io' as io;
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/notes_fab_icon.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/commons/styles/quill_editor_styles.dart';
import 'package:mechanix_notes/src/features/editor/audio_embed.dart';
import 'package:mechanix_notes/src/features/editor/menu_options.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/additional_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/alignment_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/text_editor_toolbar.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/models/toolbar_model.dart';
import "package:path/path.dart" as path;

import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:flutter_quill_extensions/flutter_quill_extensions.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/floatingActionButton/mechanix_fab_items.dart';

class NotesEditor extends StatefulWidget {
  final NoteHive? note;
  const NotesEditor({super.key, this.note});

  @override
  State<NotesEditor> createState() => _NotesEditorState();
}

class _NotesEditorState extends State<NotesEditor> {
  late final QuillController _controller;
  final FocusNode _focusNode = FocusNode();
  bool toolbarToggle = true;
  final LayerLink linkLayer = LayerLink();
  final LayerLink optionsLayer = LayerLink();
  final TextEditingController _titleController = TextEditingController();
  late bool isEditing;
  bool isUndo = false;
  bool isRedo = false;
  late bool isPinned = false;
  late String tag;
  ToolbarEnum? selectedToolbar;

  @override
  void initState() {
    super.initState();

    isEditing = widget.note != null;

    if (isEditing) {
      _titleController.text = widget.note?.title ?? '';
      isPinned = widget.note!.isPinned;
    }

    isPinned = widget.note?.isPinned ?? false;
    tag = widget.note?.tag ?? 'none';
    final doc =
        isEditing
            ? Document.fromJson(jsonDecode(widget.note!.content))
            : Document();

    _controller = QuillController(
      document: doc,
      selection: TextSelection.collapsed(offset: doc.length - 1),
      config: QuillControllerConfig(
        requireScriptFontFeatures: false,
        clipboardConfig: QuillClipboardConfig(
          enableExternalRichPaste: true,
          onImagePaste: (imageBytes) async {
            final newFileName =
                'image-file-${DateTime.now().toIso8601String()}.png';
            final newPath = path.join(
              io.Directory.systemTemp.path,
              newFileName,
            );
            final file = await io.File(
              newPath,
            ).writeAsBytes(imageBytes, flush: true);
            return file.path;
          },
        ),
      ),
    );

    _controller.addListener(_onControllerChange);
  }

  void _enableToolbar() {
    setState(() {
      toolbarToggle = !toolbarToggle;
      if (!toolbarToggle) {
        selectedToolbar = null;
      }
    });
  }

  void _onControllerChange() {
    setState(() {
      isUndo = _controller.hasUndo;
      isRedo = _controller.hasRedo;
    });
  }

  void toolbarSelection(ToolbarEnum value) {
    setState(() {
      if (selectedToolbar == value) {
        selectedToolbar = null;
      } else {
        selectedToolbar = value;
      }
    });

    if (_focusNode.hasFocus) {
      _focusNode.requestFocus();
    }
  }

  void _saveNotes() {
    final title =
        _titleController.text.trim().isNotEmpty
            ? _titleController.text.trim()
            : "New Note";
    final content = jsonEncode(_controller.document.toDelta());
    final plainText = _controller.document.toPlainText();
    if (plainText.isNotEmpty && plainText != '\n') {
      if (widget.note != null) {
        context.read<NotesBloc>().add(
          UpdateNotes(
            id: widget.note!.id,
            content: content,
            title: title,
            plainText: plainText,
            isPinned: isPinned,
            tag: tag,
          ),
        );
      } else {
        context.read<NotesBloc>().add(
          CreateNotes(title, content, plainText, isPinned, tag),
        );
      }
    }
    Navigator.pop(context);
  }

  void _undoCall() {
    _controller.undo();
  }

  void _redoCall() {
    _controller.redo();
  }

  Widget _buildSelectedToolbar() {
    if (selectedToolbar == null) return SizedBox.shrink();

    switch (selectedToolbar!) {
      case ToolbarEnum.align:
        return AlignmentToolbar(controller: _controller, focusNode: _focusNode);
      case ToolbarEnum.text:
        return TextEditorToolbar(
          controller: _controller,
          focusNode: _focusNode,
        );
      case ToolbarEnum.add:
        return AdditionalToolbar(
          controller: _controller,
          focusNode: _focusNode,
        );
      default:
        return SizedBox.shrink();
    }
  }

  void togglePinned() {
    setState(() {
      isPinned = !isPinned;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: MechanixNavigationBar(
        leadingWidth: 320,
        leadingWidget: Row(
          children: [
            IconButton(
              icon: Image.asset(NotesIcon.backIcon, height: 20, width: 20),
              onPressed: _saveNotes,
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
            ),
            Expanded(
              child: TextFormField(
                autofocus: false,
                maxLines: 1,
                controller: _titleController,
                maxLength: 25,
                style: TextStyle(color: NotesColors.titleTextColor),
                decoration: InputDecoration(
                  counterText: "",
                  hintText: "New Note",
                  hintStyle: TextStyle(color: NotesColors.titleTextColor),
                  border: InputBorder.none,
                  isCollapsed: true,
                ),
              ),
            ),
          ],
        ),
        actionsIconTheme: IconThemeData(size: 20),
        actionWidgets: [
          if (!toolbarToggle)
            IconButton(
              onPressed: isUndo ? _undoCall : null,
              icon: SizedBox(
                height: 20,
                width: 20,
                child: Image.asset(
                  NotesIcon.undoIcon,
                  color:
                      isUndo ? Colors.white : Theme.of(context).disabledColor,
                ),
              ),
            ).padRight(10),
          if (!toolbarToggle) ...[
            IconButton(
              onPressed: isRedo ? _redoCall : null,
              icon: SizedBox(
                height: 20,
                width: 20,
                child: Image.asset(
                  NotesIcon.redoIcon,
                  color:
                      isRedo ? Colors.white : Theme.of(context).disabledColor,
                ),
              ),
            ).padRight(10),
          ],
          IconButton(
            onPressed: () {
              _enableToolbar();
            },
            icon: SizedBox(
              height: 20,
              width: 20,
              child: Image.asset(
                toolbarToggle
                    ? NotesIcon.toolbarEnableIcon
                    : NotesIcon.toolbarDisableIcon,
              ),
            ),
          ),
          CompositedTransformTarget(
            link: optionsLayer,
            child: IconButton(
              onPressed: () {
                _showOptions(context);
              },
              icon: SizedBox(
                height: 20,
                width: 20,
                child: Image.asset(NotesIcon.threeDotIcon),
              ),
            ),
          ),
        ],
      ),
      body: GestureDetector(
        behavior: HitTestBehavior.translucent,
        onTap: () {
          if (selectedToolbar != null) {
            setState(() {
              selectedToolbar = null;
            });
          }
          FocusScope.of(context).unfocus();
        },
        child: Stack(
          children: [
            // Main content
            Positioned.fill(
              child: Container(
                padding: const EdgeInsets.all(4),
                child: Column(
                  children: [
                    Expanded(
                      child: QuillEditor.basic(
                        controller: _controller,
                        focusNode: _focusNode,
                        config: QuillEditorConfig(
                          enableSelectionToolbar: false,
                          onKeyPressed: (event, node) {
                            if (event.logicalKey == LogicalKeyboardKey.escape) {
                              FocusScope.of(context).unfocus();
                              return KeyEventResult.ignored;
                            } else {
                              return KeyEventResult.ignored;
                            }
                          },
                          customStyles: quillEditorStyle,
                          enableScribble: false,
                          autoFocus: false,
                          enableInteractiveSelection: true,
                          placeholder: "Content",
                          embedBuilders: [
                            ...FlutterQuillEmbeds.editorBuilders(
                              imageEmbedConfig: QuillEditorImageEmbedConfig(
                                imageProviderBuilder: (context, imageUrl) {
                                  if (imageUrl.startsWith('assets/')) {
                                    return AssetImage(imageUrl);
                                  }
                                  return null;
                                },
                              ),
                              videoEmbedConfig: QuillEditorVideoEmbedConfig(
                                customVideoBuilder: (videoUrl, readOnly) {
                                  return null;
                                },
                              ),
                            ),
                            AudioEmbedBuilder(),
                          ],
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),

            if (selectedToolbar != null)
              Positioned(
                left: 0,
                right: 0,
                bottom: 90,
                child: Center(
                  child: Container(
                    margin: EdgeInsets.symmetric(horizontal: 16),
                    child: _buildSelectedToolbar(),
                  ),
                ),
              ),

            if (toolbarToggle)
              Positioned(
                left: 0,
                right: 0,
                bottom: 30,
                child: Center(
                  child: SizedBox(
                    width: 380,
                    height: 52,
                    child: MechanixFloatingActionMenu(
                      height: 52,
                      backgroundColor: NotesColors.floatingMenuColor,
                      items: [
                        MechanixFabItem(
                          iconWidget: NotesFabIcon(
                            iconPath: NotesIcon.undoIcon,
                            color:
                                isUndo
                                    ? Colors.white
                                    : Theme.of(context).disabledColor,
                          ),
                          iconSize: 20,
                          onTap: isUndo ? _undoCall : null,
                        ),
                        MechanixFabItem(
                          iconWidget: NotesFabIcon(
                            iconPath: NotesIcon.redoIcon,
                            color:
                                isRedo
                                    ? Colors.white
                                    : Theme.of(context).disabledColor,
                          ),
                          iconSize: 20,
                          onTap: isRedo ? _redoCall : null,
                        ),
                        MechanixFabItem(
                          iconSize: 20,
                          iconWidget: NotesFabIcon(
                            iconPath: NotesIcon.textStyleIcon,
                            iconSize: 20,
                            color:
                                selectedToolbar == ToolbarEnum.text
                                    ? Theme.of(context).disabledColor
                                    : Colors.white,
                          ),
                          onTap: () => toolbarSelection(ToolbarEnum.text),
                        ),
                        MechanixFabItem(
                          iconSize: 20,
                          anchorLink: linkLayer,
                          iconWidget: NotesFabIcon(
                            iconPath: NotesIcon.menuIcon,
                            iconSize: 20,
                            color:
                                selectedToolbar == ToolbarEnum.align
                                    ? Theme.of(context).disabledColor
                                    : Colors.white,
                          ),
                          onTap: () => toolbarSelection(ToolbarEnum.align),
                        ),
                        // MechanixFabItem(
                        //   iconSize: 20,
                        //   iconWidget: NotesFabIcon(
                        //     iconPath: NotesIcon.addIcon,
                        //     iconSize: 20,
                        //     color:
                        //         selectedToolbar == ToolbarEnum.add
                        //             ? Theme.of(context).disabledColor
                        //             : Colors.white,
                        //   ),
                        //   onTap: () => toolbarSelection(ToolbarEnum.add),
                        // ),
                      ],
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }

  void _showOptions(BuildContext context) {
    OverlayEntry? entry;

    entry = OverlayEntry(
      builder:
          (_) => MenuOptions(
            menuLink: optionsLayer,
            entry: entry,
            note: widget.note,
            isPinned: isPinned,
            togglePinned: togglePinned,
          ),
    );

    Overlay.of(context, rootOverlay: true).insert(entry);
  }

  @override
  void dispose() {
    _controller.removeListener(_onControllerChange);
    _controller.dispose();
    _focusNode.dispose();
    _titleController.dispose();
    super.dispose();
  }
}
