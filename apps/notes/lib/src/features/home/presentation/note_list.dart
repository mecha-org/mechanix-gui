import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/home/presentation/app_title.dart';
import 'package:mechanix_notes/src/features/home/presentation/home_floating_button.dart';
import 'package:mechanix_notes/src/features/home/presentation/note_card.dart';
import 'package:mechanix_notes/src/features/home/presentation/notes_group.dart';
import 'package:mechanix_notes/src/features/home/widgets/label_scroll_bar.dart';
import 'package:mechanix_notes/src/features/home/widgets/sliver_grid.dart';

class NoteList extends StatefulWidget {
  final bool isSelectionMode;
  final List<GroupedNotes> groupedNotes;

  const NoteList({
    super.key,
    required this.isSelectionMode,
    required this.groupedNotes,
  });

  @override
  State<NoteList> createState() => _NoteListState();
}

class _NoteListState extends State<NoteList>
    with SingleTickerProviderStateMixin {
  late final ScrollController _scrollController;
  late final AnimationController _fabAnimationController;
  late final Animation<double> _fabAnimation;

  bool _isScrolling = false;
  DateTime? _lastScrollTime;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    _scrollController.addListener(_onScroll);

    // Animation controller for FAB fade
    _fabAnimationController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
      value: 1.0, // Start fully visible
    );

    _fabAnimation = CurvedAnimation(
      parent: _fabAnimationController,
      curve: Curves.easeInOut,
    );
  }

  void _onScroll() {
    if (!_scrollController.hasClients) return;
    final position = _scrollController.position;
    if (!position.hasPixels || !position.hasContentDimensions) return;

    // Track scrolling state
    _lastScrollTime = DateTime.now();

    if (!_isScrolling) {
      _isScrolling = true;
      _fabAnimationController.reverse(); // Fade out
    }

    // Check if scroll stopped after a delay
    Future.delayed(const Duration(milliseconds: 150), () {
      if (_lastScrollTime != null &&
          DateTime.now().difference(_lastScrollTime!) >
              const Duration(milliseconds: 100)) {
        if (_isScrolling) {
          _isScrolling = false;
          _fabAnimationController.forward(); // Fade in
        }
      }
    });

    // Trigger pagination near bottom
    final maxScroll = position.maxScrollExtent;
    final currentScroll = position.pixels;
    if (currentScroll >= 0.8 * maxScroll) {
      context.read<NotesBloc>().add(LoadNextChunk());
    }
  }

  @override
  void dispose() {
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    _fabAnimationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.groupedNotes.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.note_add, size: 64, color: Colors.grey),
            const SizedBox(height: 16),
            TextButton(
              onPressed: () {
                Navigator.pushNamed(context, AppRoutes.createEditNotes);
              },
              child: const Text(
                "Add a new Note",
                style: TextStyle(fontSize: 16),
              ),
            ),
          ],
        ),
      );
    }

    return Stack(
      children: [
        ScrollConfiguration(
          behavior: const ScrollBehavior().copyWith(
            overscroll: false,
            scrollbars: false,
            dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
          ),
          child: CustomScrollView(
            controller: _scrollController,
            physics: const AlwaysScrollableScrollPhysics(),
            cacheExtent: 800,
            slivers: [
              // Add "Notes" title as first sliver if showTitle is true
              if (!widget.isSelectionMode)
                const SliverToBoxAdapter(child: AppTitle()),

              // Add all note groups
              for (int i = 0; i < widget.groupedNotes.length; i++)
                ..._buildGroupSection(widget.groupedNotes[i], i == 0),
            ],
          ),
        ),

        // Floating button with fade animation
        BlocSelector<NotesBloc, NotesState, bool>(
          selector: (state) => state.isSelectionMode,
          builder:
              (context, value) =>
                  !value
                      ? Positioned(
                        bottom: 16,
                        right: 16,
                        child: FadeTransition(
                          opacity: _fabAnimation,
                          child: const HomeFloatingButton(),
                        ),
                      )
                      : const SizedBox.shrink(),
        ),

        // backdrop during dragging
        BlocSelector<NotesBloc, NotesState, bool>(
          selector: (state) => state.isDragging,
          builder:
              (context, value) => Positioned.fill(
                child:
                    value
                        ? IgnorePointer(
                          ignoring: false, // don't block taps
                          child: Container(
                            color: Colors.black.withValues(alpha: 0.5),
                          ),
                        )
                        : const SizedBox.shrink(),
              ),
        ),

        // Custom scrollbar with section indicators
        Positioned(
          right: -15,
          top: 0,
          bottom: 0,
          child: BlocSelector<NotesBloc, NotesState, bool>(
            selector: (state) => state.isDragging,
            builder:
                (context, isDragging) => LabelScrollBar(
                  isDragging: isDragging,
                  scrollController: _scrollController,
                  groupedNotes: widget.groupedNotes,
                ),
          ),
        ),
      ],
    );
  }

  List<Widget> _buildGroupSection(GroupedNotes group, bool isFirst) {
    if (group.notes.isEmpty) return [];

    final itemHeights = group.notes.map((note) => note.height).toList();

    return [
      SliverPersistentHeader(
        pinned: false,
        delegate: GroupHeaderName(title: group.label, isFirst: isFirst),
      ),

      SliverPadding(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        sliver: SliverGrid(
          gridDelegate: SliverStaggeredGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 2,
            mainAxisSpacing: 12,
            crossAxisSpacing: 12,
            itemHeights: itemHeights,
          ),
          delegate: SliverChildBuilderDelegate(
            (context, index) {
              if (index >= group.notes.length) return const SizedBox.shrink();

              final note = group.notes[index];

              return BlocSelector<NotesBloc, NotesState, bool>(
                selector: (state) => state.selectedNoteIds.contains(note.id),
                builder: (context, isSelected) {
                  return NoteCard(
                    key: ValueKey(note.id),
                    note: note,
                    isSelected: isSelected,
                    isSelectionMode: widget.isSelectionMode,
                  );
                },
              );
            },
            childCount: group.notes.length,
            addAutomaticKeepAlives: false,
            addRepaintBoundaries: false,
          ),
        ),
      ),
    ];
  }
}
