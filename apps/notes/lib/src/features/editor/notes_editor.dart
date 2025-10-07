import 'dart:convert';
import 'dart:io' as io;
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/notes_fab_icon.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/commons/styles/quill_editor_styles.dart';
import 'package:mechanix_notes/src/features/editor/audio_embed.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/editor_bar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/additional_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/alignment_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/text_editor_toolbar.dart';
import 'package:mechanix_notes/src/features/home/models/toolbar_model.dart';
import "package:path/path.dart" as path;
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:flutter_quill_extensions/flutter_quill_extensions.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:tuple/tuple.dart';
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
  final LayerLink linkLayer = LayerLink();
  final TextEditingController _titleController = TextEditingController();

  ToolbarEnum? selectedToolbar;

  @override
  void initState() {
    super.initState();

    bool isEditing = widget.note != null;

    if (isEditing) {
      _titleController.text = widget.note?.title ?? '';
    }
    context.read<EditorBloc>().add(
      PinnedUpdate(isPinned: widget.note?.isPinned ?? false),
    );

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

  void _onControllerChange() {
    context.read<EditorBloc>().add(UndoUpdate(isUndo: _controller.hasUndo));
    context.read<EditorBloc>().add(RedoUpdate(isRedo: _controller.hasRedo));
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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: Size.fromHeight(50),
        child: EditorBar(controller: _controller, note: widget.note),
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

            BlocSelector<EditorBloc, EditorBlocState, Tuple3<bool, bool, bool>>(
              selector:
                  (state) =>
                      Tuple3(state.isUndo, state.isRedo, state.toolbarToggle),
              builder: (context, tuple) {
                final isUndo = tuple.item1;
                final isRedo = tuple.item2;
                final toolbarToggle = tuple.item3;
                // try to solve this to not get return when toolbarToggle is false i.e without returning Container
                if (!toolbarToggle) return Container();

                return Positioned(
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
                          MechanixFabItem(
                            iconSize: 20,
                            iconWidget: NotesFabIcon(
                              iconPath: NotesIcon.addIcon,
                              iconSize: 20,
                              color:
                                  selectedToolbar == ToolbarEnum.add
                                      ? Theme.of(context).disabledColor
                                      : Colors.white,
                            ),
                            onTap: () => toolbarSelection(ToolbarEnum.add),
                          ),
                        ],
                      ),
                    ),
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
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
