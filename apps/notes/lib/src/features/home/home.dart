import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/bottom_menu/bottom_menu.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/home/presentation/app_title.dart';
import 'package:mechanix_notes/src/features/home/presentation/empty_notes.dart';
import 'package:mechanix_notes/src/features/home/presentation/home_floating_button.dart';
import 'package:mechanix_notes/src/features/home/presentation/loading_notes.dart';
import 'package:mechanix_notes/src/features/home/presentation/note_list.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_notes.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage>
    with SingleTickerProviderStateMixin {
  late AnimationController _animationController;
  late Animation<double> _fadeAnimation;

  @override
  void initState() {
    super.initState();

    // Initialize animation controller
    _animationController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
    );

    _fadeAnimation = CurvedAnimation(
      parent: _animationController,
      curve: Curves.easeInOut,
    );

    _animationController.forward();
    context.read<NotesBloc>().add(LoadNotes());
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<NotesBloc, NotesState, Tuple2<bool, bool>>(
      selector: (state) => Tuple2(state.isSearchPage, state.isSelectionMode),
      builder: (context, data) {
        final isSearchPage = data.item1;
        final isSelectionMode = data.item2;

        return Scaffold(
          // Animated bottom navigation bar
          bottomNavigationBar: AnimatedSwitcher(
            duration: const Duration(milliseconds: 300),
            transitionBuilder: (child, animation) {
              return SlideTransition(
                position: Tween<Offset>(
                  begin: const Offset(0, 1),
                  end: Offset.zero,
                ).animate(
                  CurvedAnimation(parent: animation, curve: Curves.easeInOut),
                ),
                child: FadeTransition(opacity: animation, child: child),
              );
            },
            child:
                (isSelectionMode && !isSearchPage)
                    ? BottomMenu(
                      key: const ValueKey('bottom_menu'),
                      isSelectionMode: isSelectionMode,
                    )
                    : const SizedBox.shrink(key: ValueKey('no_menu')),
          ),
          floatingActionButton: BlocSelector<NotesBloc, NotesState, bool>(
            selector: (state) => state.groupedNotes.isEmpty,
            builder:
                (context, hasNotes) =>
                    hasNotes ? HomeFloatingButton() : const SizedBox.shrink(),
          ),

          // App bar only shows in selection mode
          appBar:
              ((isSelectionMode && !isSearchPage)
                  ? PreferredSize(
                    preferredSize: const Size.fromHeight(48),
                    child: BlocSelector<NotesBloc, NotesState, List<String>>(
                      selector: (state) => state.selectedNoteIds,
                      builder: (context, selectedNoteIds) {
                        return AnimatedSwitcher(
                          duration: const Duration(milliseconds: 250),
                          transitionBuilder: (child, animation) {
                            return FadeTransition(
                              opacity: animation,
                              child: SlideTransition(
                                position: Tween<Offset>(
                                  begin: const Offset(0, -0.2),
                                  end: Offset.zero,
                                ).animate(
                                  CurvedAnimation(
                                    parent: animation,
                                    curve: Curves.easeOut,
                                  ),
                                ),
                                child: child,
                              ),
                            );
                          },
                          child: MechanixNavigationBar(
                            key: const ValueKey('selection_mode'),
                            height: 48,
                            automaticallyImplyLeading: false,
                            theme: MechanixNavigationBarThemeData(
                              titleStyle: TextStyle(
                                color: context.onSurface,
                                fontSize: 20,
                                fontWeight: FontWeight.w500,
                                height: 1.3,
                                letterSpacing: -1.1,
                              ),
                            ),
                            title: "${selectedNoteIds.length} Selected",
                          ),
                        );
                      },
                    ),
                  )
                  : null),

          // Animated body with smooth vertical slide transition
          body: AnimatedSwitcher(
            duration: const Duration(milliseconds: 400),
            switchInCurve: Curves.easeInOutCubic,
            switchOutCurve: Curves.easeInOutCubic,
            transitionBuilder: (child, animation) {
              // Check which page is being shown
              final isSearchChild =
                  (child.key as ValueKey).value == 'search_page';

              // Reverse animation for exit
              final offsetAnimation = Tween<Offset>(
                begin:
                    isSearchChild
                        ? const Offset(0, 1) // Search slides up from bottom
                        : const Offset(
                          0,
                          -1,
                        ), // Notes slides down from top when returning
                end: Offset.zero,
              ).animate(
                CurvedAnimation(
                  parent: animation,
                  curve: Curves.easeInOutCubic,
                ),
              );

              return SlideTransition(
                position: offsetAnimation,
                child: FadeTransition(
                  opacity: Tween<double>(begin: 0.0, end: 1.0).animate(
                    CurvedAnimation(
                      parent: animation,
                      curve: const Interval(0.3, 1.0, curve: Curves.easeIn),
                    ),
                  ),
                  child: child,
                ),
              );
            },
            child:
                isSearchPage
                    ? const SearchNotes(key: ValueKey('search_page'))
                    : BlocSelector<
                      NotesBloc,
                      NotesState,
                      Tuple2<bool, List<GroupedNotes>>
                    >(
                      key: const ValueKey('notes_content'),
                      selector:
                          (state) =>
                              Tuple2(state.isLoading, state.groupedNotes),
                      builder: (context, data) {
                        final isLoading = data.item1;
                        final groupedNotes = data.item2;

                        // Animated loading state - show "Notes" title
                        if (isLoading) {
                          return Center(
                            child: FadeTransition(
                              opacity: _fadeAnimation,
                              child: Column(
                                children: [
                                  if (!isSelectionMode) const AppTitle(),
                                  const Expanded(
                                    child: Center(child: LoadingNotes()),
                                  ),
                                ],
                              ),
                            ),
                          );
                        }

                        // Animated empty state - show "Notes" title
                        if (groupedNotes.isEmpty) {
                          return FadeTransition(
                            opacity: _fadeAnimation,
                            child: Column(
                              children: [
                                if (!isSelectionMode) const AppTitle(),
                                const Expanded(child: EmptyNotes()),
                              ],
                            ),
                          );
                        }

                        // Animated note list with scrollable title
                        return FadeTransition(
                          opacity: _fadeAnimation,
                          child: NoteList(
                            isSelectionMode: isSelectionMode,
                            groupedNotes: groupedNotes,
                          ),
                        );
                      },
                    ),
          ),
        );
      },
    );
  }
}
