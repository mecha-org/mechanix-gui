import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:flutter_quill_extensions/flutter_quill_extensions.dart';
import 'package:logger/logger.dart';
import 'package:file_picker/file_picker.dart';
import 'dart:io' as io show File, Directory;
import "package:path/path.dart" as path;
import 'package:path_provider/path_provider.dart';

class ContentEdit extends StatefulWidget {
  final String? initialDeltaJson;
  final bool toggleToolBar;

  final Function(String, String) onContentChanged;

  const ContentEdit({
    super.key,
    this.initialDeltaJson,
    required this.onContentChanged,
    required this.toggleToolBar,
  });

  @override
  State<ContentEdit> createState() => _ContentEditState();
}

class _ContentEditState extends State<ContentEdit> {
  final Logger logger = Logger();
  late final QuillController _controller;
  final FocusNode _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();

    final doc =
        widget.initialDeltaJson != null && widget.initialDeltaJson!.isNotEmpty
            ? Document.fromJson(jsonDecode(widget.initialDeltaJson!))
            : Document();

    _controller = QuillController(
      document: doc,
      selection: const TextSelection.collapsed(offset: 0),
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
    _controller.addListener(() {
      final json = jsonEncode(_controller.document.toDelta());
      final plainText = _controller.document.toPlainText();
      widget.onContentChanged(json, plainText);
    });
  }

  /// Inserts an image from file picker
  Future<void> _insertImageFromFile() async {
    final result = await FilePicker.platform.pickFiles(type: FileType.image);

    if (result != null && result.files.single.path != null) {
      final originalPath = result.files.single.path!;
      final originalFile = io.File(originalPath);

      // Generate a new file path in the app's temp directory
      final fileName =
          'mechanix_notes_${DateTime.now().millisecondsSinceEpoch}${path.extension(originalPath)}';
      final appDir = await getApplicationSupportDirectory();

      final newPath = path.join(appDir.path, fileName);
      final savedImageFile = await originalFile.copy(newPath);

      // Use the saved file path in the Quill document
      final imagePath = savedImageFile.path;

      final index = _controller.selection.baseOffset;
      _controller.document.insert(index, BlockEmbed.image(imagePath));
      _controller.updateSelection(
        TextSelection.collapsed(offset: index + 1),
        ChangeSource.local,
      );
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        if (widget.toggleToolBar) ...[
          // QuillSimpleToolbar(
          //   controller: _controller,
          //   config: QuillSimpleToolbarConfig(
          //     customButtons: [
          //       QuillToolbarCustomButtonOptions(
          //         icon: const Icon(Icons.image),
          //         onPressed: _insertImageFromFile,
          //       ),
          //     ],

          //     buttonOptions: QuillSimpleToolbarButtonOptions(
          //       base: QuillToolbarBaseButtonOptions(
          //         afterButtonPressed: () {
          //           final isDesktop = {
          //             TargetPlatform.linux,
          //             TargetPlatform.windows,
          //             TargetPlatform.macOS,
          //           }.contains(defaultTargetPlatform);
          //           if (isDesktop) {
          //             _focusNode.requestFocus();
          //           }
          //         },
          //       ),
          //     ),
          //     showBackgroundColorButton: false,
          //     showFontFamily: false,
          //     showFontSize: false,
          //     showSearchButton: false,
          //   ),
          // ),
          const Divider(height: 1),
        ],
        Expanded(
          child: Container(
            padding: const EdgeInsets.all(12),
            child: QuillEditor.basic(
              controller: _controller,
              focusNode: _focusNode,
              config: QuillEditorConfig(
                enableScribble: false,
                
                autoFocus: false,
                enableInteractiveSelection: true,
                embedBuilders: [
                  ...FlutterQuillEmbeds.editorBuilders(
                    imageEmbedConfig: const QuillEditorImageEmbedConfig(
                      // imageProviderBuilder: (context, imageUrl) {
                      //   print("image content $imageUrl");
                      //   if (imageUrl.startsWith('assets/')) {
                      //     return AssetImage(imageUrl);
                      //   }
                      //   return null;
                      // },
                    ),
                    videoEmbedConfig: QuillEditorVideoEmbedConfig(
                      customVideoBuilder: (videoUrl, readOnly) {
                        return null;
                      },
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}
