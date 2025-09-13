import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/audio_embed.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';
import 'package:mechanix_notes/src/features/editor/linux_audio_recorder_dialog.dart';
import 'dart:io' as io show File, Directory;
import "package:path/path.dart" as path;
import 'package:path_provider/path_provider.dart';

class AdditionalToolbar extends StatelessWidget {
  final QuillController controller;
  final FocusNode focusNode;

  const AdditionalToolbar({
    super.key,
    required this.controller,
    required this.focusNode,
  });

  @override
  Widget build(BuildContext context) {
    void requestFocus() {
      if (!focusNode.hasFocus) {
        focusNode.requestFocus();
      }
    }

    Future<void> insertAudioRecording() async {
      final result = await showDialog<String?>(
        context: context,
        barrierDismissible: false,
        builder: (context) => const LinuxAudioRecordingDialog(),
      );

      if (result != null && result.isNotEmpty) {
        final index = controller.selection.baseOffset;
        controller.document.insert(index, AudioEmbed.fromPath(result));
        controller.updateSelection(
          TextSelection.collapsed(offset: index + 1),
          ChangeSource.local,
        );
        requestFocus();
      }
    }

    Future<void> insertAudioFile() async {
      try {
        // Pick audio file
        final result = await FilePicker.platform.pickFiles(
          type: FileType.audio,
          allowMultiple: false,
        );

        if (result != null && result.files.single.path != null) {
          final originalPath = result.files.single.path!;
          final originalFile = io.File(originalPath);

          // Copy file to app directory for persistence
          final fileName =
              'audio_${DateTime.now().millisecondsSinceEpoch}${path.extension(originalPath)}';
          final appDir = await getApplicationSupportDirectory();
          final newPath = path.join(appDir.path, fileName);

          // Ensure directory exists
          await io.Directory(path.dirname(newPath)).create(recursive: true);

          final savedAudioFile = await originalFile.copy(newPath);

          // Insert audio embed into document
          final index = controller.selection.baseOffset;
          controller.document.insert(
            index,
            AudioEmbed.fromPath(savedAudioFile.path),
          );
          controller.updateSelection(
            TextSelection.collapsed(offset: index + 1),
            ChangeSource.local,
          );

          requestFocus();
        }
      } catch (e) {
        // ScaffoldMessenger.of(context).showSnackBar(
        //   const SnackBar(
        //     content: Text('Failed to attach audio file'),
        //     backgroundColor: Colors.red,
        //   ),
        // );
      }
    }

    Future<void> insertImageFromFile() async {
      final result = await FilePicker.platform.pickFiles(type: FileType.image);

      if (result != null && result.files.single.path != null) {
        final originalPath = result.files.single.path!;
        final originalFile = io.File(originalPath);

        final fileName =
            'mechanix_notes_${DateTime.now().millisecondsSinceEpoch}${path.extension(originalPath)}';
        final appDir = await getApplicationSupportDirectory();

        final newPath = path.join(appDir.path, fileName);
        final savedImageFile = await originalFile.copy(newPath);

        // Use the saved file path in the Quill document
        final imagePath = savedImageFile.path;

        final index = controller.selection.baseOffset;
        controller.document.insert(index, BlockEmbed.image(imagePath));
        controller.updateSelection(
          TextSelection.collapsed(offset: index + 1),
          ChangeSource.local,
        );
      }
    }

    // Future<void> startAudioRecording() async {
    //   final result = await showDialog<String?>(
    //     context: context,
    //     barrierDismissible: false,
    //     builder: (context) => const LinuxAudioRecordingDialog(),
    //   );
    //   if (result != null && result.isNotEmpty) {
    //     final index = controller.selection.baseOffset;
    //     controller.document.insert(index, AudioEmbed.fromPath(result));
    //     controller.updateSelection(
    //       TextSelection.collapsed(offset: index + 1),
    //       ChangeSource.local,
    //     );
    //     requestFocus();
    //   }
    // }

    return Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: NotesColors.backgroundColor,
      ),
      width: 60,
      padding: EdgeInsets.all(4),
      height: 52,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // EditorIconButton(
          //   isSelected: controller.getSelectionStyle().containsKey(
          //     Attribute.italic.key,
          //   ),
          //   iconPath: NotesIcon.audioIcon,
          //   onPressed: () {
          //     insertAudioFile();
          //   },
          // ),

          // EditorIconButton(
          //   isSelected: controller.getSelectionStyle().containsKey(
          //     Attribute.italic.key,
          //   ),
          //   iconPath: NotesIcon.galleryUploadIcon,
          //   onPressed: () {
          //     insertAudioRecording();
          //   },
          // ),

          // Image insertion button
          EditorIconButton(
            isSelected: false,
            iconPath: NotesIcon.galleryUploadIcon,
            onPressed: () => insertImageFromFile(),
          ),
        ],
      ),
    );
  }
}
