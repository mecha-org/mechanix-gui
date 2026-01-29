import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/common_helper.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc_provider.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/home/presentation/note_preview.dart';
import 'package:widgets/mechanix.dart';

class NoteCard extends StatefulWidget {
  final NoteMetaData note;
  final bool isSelectionMode;
  final bool isSelected;
  const NoteCard({
    super.key,
    required this.note,
    required this.isSelectionMode,
    required this.isSelected,
  });

  @override
  State<NoteCard> createState() => NoteCardState();
}

class NoteCardState extends State<NoteCard> with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  void _handleTap() {
    if (widget.isSelectionMode) {
      context.read<NotesBloc>().add(SelectNote(widget.note.id));
    } else {
      _openNote(context, widget.note.id);
    }
  }

  void _handleLongPress() {
    context.read<NotesBloc>().add(SelectNote(widget.note.id));
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);

    return RepaintBoundary(
      child: InkWell(
        borderRadius: BorderRadius.circular(12),
        onTap: _handleTap,
        onLongPress: _handleLongPress,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOutCubic,
          decoration: BoxDecoration(
            color: context.secondary,
            borderRadius: BorderRadius.circular(12),
            border:
                widget.isSelected
                    ? Border.all(
                      color: context.primaryContainer.withValues(alpha: 0.8),
                      width: 1,
                    )
                    : null,
          ),
          child: Stack(
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                spacing: 16,
                children: [
                  if (widget.note.title.isNotEmpty)
                    SizedBox(
                      height: 26,
                      child: Text(
                        widget.note.title,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(
                          fontSize: 20,
                          color: context.onSurface,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                    ),
                  if (widget.note.preview.isNotEmpty) ...[
                    Flexible(
                      child: NotePreview(
                        key: ValueKey(widget.note.id),
                        lines: widget.note.preview,
                        maxLines: 2,
                      ),
                    ),
                  ],
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      SizedBox(
                        height: 18,
                        child: Text(
                          CommonHelper.formatDateTime(widget.note.updatedAt),
                          style: TextStyle(
                            color: context.onSecondaryFixed,
                            fontSize: 16,
                            fontWeight: FontWeight.w400,
                          ),
                        ),
                      ),
                    ],
                  ),
                ],
              ).padAll(16),
              if (widget.isSelectionMode)
                Positioned(
                  bottom: 8,
                  right: 8,
                  child: TweenAnimationBuilder<double>(
                    tween: Tween(
                      begin: 0.0,
                      end: widget.isSelected ? 1.0 : 0.0,
                    ),
                    duration: const Duration(milliseconds: 200),
                    curve: Curves.easeOutBack,
                    builder: (context, value, child) {
                      return Transform.scale(
                        scale: 0.8 + (value * 0.2),
                        child: MechanixCircleCheckbox(
                          value: widget.isSelected,
                          onChanged: (a) {},
                          activeColor: context.primaryContainer,
                          width: 24,
                          height: 24,
                        ),
                      );
                    },
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}

void _openNote(BuildContext context, String noteId) {
  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (_) => EditorBlocProvider(child: NotesEditor(noteId: noteId)),
    ),
  );
}
