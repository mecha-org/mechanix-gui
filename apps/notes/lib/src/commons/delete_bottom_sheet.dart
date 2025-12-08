import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/mechanix.dart';

class DeleteBottomSheet extends StatelessWidget {
  final String title;
  final String message;
  final BuildContext bottomSheetContext;
  const DeleteBottomSheet({
    super.key,
    required this.title,
    required this.message,
    required this.bottomSheetContext,
  });

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          child: Image.asset(NotesIcon.slider, fit: BoxFit.cover),
        ),
        Container(
          padding: const EdgeInsets.only(
            left: 16,
            right: 16,
            bottom: 16,
            top: 32,
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: const TextStyle(
                  fontWeight: FontWeight.w500,
                  fontSize: 18,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 8),
              Text(message, style: const TextStyle(color: Colors.white70)),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: MechanixElevatedButton(
                      padding: const EdgeInsets.symmetric(
                        vertical: 16,
                        horizontal: 12,
                      ),
                      label: "Cancel",
                      textColor: Colors.white,
                      backgroundColor: Colors.grey.shade800,
                      borderRadius: 8,
                      onPressed: () => Navigator.pop(bottomSheetContext),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: MechanixElevatedButton(
                      padding: const EdgeInsets.symmetric(
                        vertical: 16,
                        horizontal: 12,
                      ),
                      label: "Delete",
                      backgroundColor: Colors.red.shade700,
                      textColor: Colors.white,
                      borderRadius: 8,
                      onPressed: () {
                        Navigator.pop(bottomSheetContext);
                        context.read<NotesBloc>().add(
                          DeleteNotes(deleteIds: const []),
                        );
                        context.read<NotesBloc>().add(ClearSelection());
                      },
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ],
    );
  }
}
