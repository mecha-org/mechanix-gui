import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:widgets/mechanix.dart';

class HomeFloatingButton extends StatelessWidget {
  const HomeFloatingButton({super.key});
  @override
  Widget build(BuildContext context) {
    void onSearch() {
      context.read<NotesBloc>().add(SearchPageToggle(isSearchPage: true));
    }

    void createNote() {
      Navigator.pushNamed(context, AppRoutes.createEditNotes);
    }

    return Container(
      margin: EdgeInsets.only(bottom: 30),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.end,
        crossAxisAlignment: CrossAxisAlignment.center,
        spacing: 16,
        children: [
          BlocSelector<NotesBloc, NotesState, bool>(
            selector: (state) => state.groupedNotes.isNotEmpty,
            builder:
                (context, hasNotes) => Container(
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(8),
                    color: context.surfaceContainer,
                  ),
                  child: IconButton(
                    padding: const EdgeInsets.all(10),
                    onPressed: hasNotes ? onSearch : null,
                    icon: IconWidget(
                      iconColor:
                          hasNotes ? null : Theme.of(context).disabledColor,
                      iconPath: NotesIcon.searchIcon,
                      iconHeight: 24,
                      iconWidth: 24,
                      boxHeight: 24,
                      boxWidth: 24,
                    ),
                  ),
                ),
          ),
          Container(
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(8),
              color: context.primaryContainer,
            ),
            child: IconButton(
              iconSize: 56,
              padding: const EdgeInsets.all(15),
              onPressed: createNote,
              icon: const IconWidget(
                iconPath: NotesIcon.addIcon,
                iconHeight: 24,
                iconWidth: 24,
                boxHeight: 24,
                boxWidth: 24,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
