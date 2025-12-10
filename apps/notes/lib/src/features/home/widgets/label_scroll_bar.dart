import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/home/widgets/section_dots.dart';

class LabelScrollBar extends StatefulWidget {
  final ScrollController scrollController;
  final List<GroupedNotes> groupedNotes;
  final bool isDragging;

  const LabelScrollBar({
    super.key,
    required this.scrollController,
    required this.groupedNotes,
    required this.isDragging,
  });

  @override
  State<LabelScrollBar> createState() => LabelScrollBarState();
}

class LabelScrollBarState extends State<LabelScrollBar> {
  int? _hoveredDotIndex;
  // bool widget.isDragging = false;
  bool _isHoveringBar = false;
  bool _isScrolling = false;
  double? _dragStartY;
  double? _dragStartScroll;

  /// Hide scroller after 1 second
  Timer? _hideTimer;
  final ValueNotifier<List<SectionInfo>> _sectionInfosNotifier = ValueNotifier(
    [],
  );
  final ValueNotifier<double> _totalContentHeightNotifier = ValueNotifier(0.0);

  @override
  void initState() {
    super.initState();
    widget.scrollController.addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _calculateSectionOffsets();
    });
  }

  @override
  void dispose() {
    _hideTimer?.cancel();
    widget.scrollController.removeListener(_onScroll);
    _sectionInfosNotifier.dispose();
    _totalContentHeightNotifier.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (!mounted) return;

    setState(() {
      _isScrolling = true;
    });

    _hideTimer?.cancel();

    _hideTimer = Timer(const Duration(milliseconds: 1000), () {
      if (mounted && !widget.isDragging && !_isHoveringBar) {
        setState(() {
          _isScrolling = false;
        });
      }
    });
  }

  void _calculateSectionOffsets() {
    if (!mounted) return;

    final newSectionInfos = <SectionInfo>[];
    double currentOffset = 0;
    const firstHeaderHeight = 44.0;
    const headerHeight = 68.0;
    const mainAxisSpacing = 12.0;

    for (int i = 0; i < widget.groupedNotes.length; i++) {
      final group = widget.groupedNotes[i];
      if (group.notes.isEmpty) continue;

      final sectionStartOffset = currentOffset;

      // Use different header height for first section
      final currentHeaderHeight = i == 0 ? firstHeaderHeight : headerHeight;

      // Add header height
      currentOffset += currentHeaderHeight;

      // Calculate grid height for this section
      final itemHeights = group.notes.map((note) => note.height).toList();
      final gridHeight = _calculateGridHeight(
        itemHeights: itemHeights,
        crossAxisCount: 2,
        mainAxisSpacing: mainAxisSpacing,
      );

      currentOffset += gridHeight;

      newSectionInfos.add(
        SectionInfo(
          label: group.label,
          offset: sectionStartOffset,
          height: currentHeaderHeight + gridHeight,
        ),
      );
    }

    // Update ValueNotifiers - this won't trigger setState
    _sectionInfosNotifier.value = newSectionInfos;
    _totalContentHeightNotifier.value = currentOffset;

    // Only call setState if not dragging
    if (mounted && !widget.isDragging) {
      setState(() {});
    }
  }

  @override
  void didUpdateWidget(LabelScrollBar oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.groupedNotes != widget.groupedNotes) {
      // Always calculate immediately to get latest data
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _calculateSectionOffsets();
      });
    }
  }

  double _calculateGridHeight({
    required List<double> itemHeights,
    required int crossAxisCount,
    required double mainAxisSpacing,
  }) {
    if (itemHeights.isEmpty) return 0;

    final columnHeights = List<double>.filled(crossAxisCount, 0);

    for (int i = 0; i < itemHeights.length; i++) {
      final columnIndex = i % crossAxisCount;
      if (columnHeights[columnIndex] > 0) {
        columnHeights[columnIndex] += mainAxisSpacing;
      }
      columnHeights[columnIndex] += itemHeights[i];
    }

    return columnHeights.reduce((a, b) => a > b ? a : b);
  }

  void _scrollToSection(int index) {
    final sections = _sectionInfosNotifier.value;
    if (index >= 0 &&
        index < sections.length &&
        widget.scrollController.hasClients) {
      final maxScroll = widget.scrollController.position.maxScrollExtent;
      final targetOffset = sections[index].offset;

      widget.scrollController.animateTo(
        targetOffset > maxScroll ? maxScroll : targetOffset,
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
  }

  int _getCurrentSectionIndex() {
    if (!widget.scrollController.hasClients ||
        _sectionInfosNotifier.value.isEmpty) {
      return 0;
    }

    final scrollOffset = widget.scrollController.offset;
    final sections = _sectionInfosNotifier.value;

    for (int i = sections.length - 1; i >= 0; i--) {
      if (scrollOffset >= sections[i].offset - 10) {
        return i;
      }
    }
    return 0;
  }

  void _handleVerticalDragStart(DragStartDetails details) {
    _hideTimer?.cancel();
    context.read<NotesBloc>().add(DragUpdate(true));

    setState(() {
      _dragStartY = details.localPosition.dy;
      _dragStartScroll = widget.scrollController.offset;
    });
  }

  void _handleVerticalDragUpdate(
    DragUpdateDetails details,
    double availableHeight,
  ) {
    if (!widget.scrollController.hasClients ||
        _dragStartY == null ||
        _dragStartScroll == null) {
      return;
    }

    final maxScroll = widget.scrollController.position.maxScrollExtent;
    final viewportHeight = widget.scrollController.position.viewportDimension;
    final contentHeight = maxScroll + viewportHeight;

    final dragDelta = details.localPosition.dy - _dragStartY!;
    final scrollRatio = contentHeight / availableHeight;
    final scrollDelta = dragDelta * scrollRatio;

    final newScroll = (_dragStartScroll! + scrollDelta).clamp(0.0, maxScroll);
    widget.scrollController.jumpTo(newScroll);
  }

  void _handleVerticalDragEnd(DragEndDetails details) {
    context.read<NotesBloc>().add(DragUpdate(false));

    setState(() {
      _dragStartY = null;
      _dragStartScroll = null;
    });
    _onScroll();
  }

  void _handleVerticalDragCancel() {
    context.read<NotesBloc>().add(DragUpdate(false));
    setState(() {
      _dragStartY = null;
      _dragStartScroll = null;
    });
    _onScroll();
  }

  double _calculateScrollbarHeight(double availableHeight) {
    if (!widget.scrollController.hasClients) return availableHeight * 0.3;

    final viewportHeight = widget.scrollController.position.viewportDimension;
    final contentHeight =
        widget.scrollController.position.maxScrollExtent + viewportHeight;

    final ratio = (viewportHeight / contentHeight).clamp(0.1, 1.0);
    return (availableHeight * ratio).clamp(50.0, availableHeight);
  }

  double _calculateScrollbarPosition(
    double availableHeight,
    double scrollbarHeight,
  ) {
    if (!widget.scrollController.hasClients) return 0;

    final scrollOffset = widget.scrollController.offset;
    final maxScroll = widget.scrollController.position.maxScrollExtent;

    if (maxScroll <= 0) return 0;

    final scrollFraction = (scrollOffset / maxScroll).clamp(0.0, 1.0);
    final maxPosition = availableHeight - scrollbarHeight;

    return scrollFraction * maxPosition;
  }

  @override
  Widget build(BuildContext context) {
    final showScrollbar = _isScrolling || widget.isDragging || _isHoveringBar;

    if (!showScrollbar) {
      return const SizedBox.shrink();
    }

    return ValueListenableBuilder<List<SectionInfo>>(
      valueListenable: _sectionInfosNotifier,
      builder: (context, sections, child) {
        if (sections.isEmpty) {
          return const SizedBox.shrink();
        }

        final currentSection = _getCurrentSectionIndex();
        final screenWidth = MediaQuery.of(context).size.width;

        return Stack(
          children: [
            Container(
              width: 56,
              padding: const EdgeInsets.symmetric(vertical: 24, horizontal: 8),
              child: LayoutBuilder(
                builder: (context, constraints) {
                  final availableHeight = constraints.maxHeight;
                  final scrollbarHeight = _calculateScrollbarHeight(
                    availableHeight,
                  );
                  final scrollbarPosition = _calculateScrollbarPosition(
                    availableHeight,
                    scrollbarHeight,
                  );

                  return Stack(
                    clipBehavior: Clip.none,
                    children: [
                      // Section dots - show during scrolling or dragging
                      if (widget.isDragging || _isScrolling)
                        SectionDotsWidget(
                          sections: sections,
                          totalContentHeight: _totalContentHeightNotifier.value,
                          availableHeight: availableHeight,
                          currentSection: currentSection,
                          hoveredDotIndex: _hoveredDotIndex,
                          onDotTap: (index) {
                            _hideTimer?.cancel();
                            _scrollToSection(index);
                            _onScroll();
                          },
                          onDotHoverEnter: (index) {
                            _hideTimer?.cancel();
                            setState(() => _hoveredDotIndex = index);
                          },
                          onDotHoverExit: () {
                            setState(() => _hoveredDotIndex = null);
                            _onScroll();
                          },
                        ),

                      // Scrollbar thumb with expanded touch area
                      Positioned(
                        right: 0,
                        top: scrollbarPosition,
                        child: MouseRegion(
                          onEnter: (_) {
                            _hideTimer?.cancel();
                            setState(() => _isHoveringBar = true);
                          },
                          onExit: (_) {
                            if (!widget.isDragging) {
                              setState(() => _isHoveringBar = false);
                              _onScroll();
                            }
                          },
                          child: GestureDetector(
                            behavior: HitTestBehavior.opaque,
                            onVerticalDragStart: _handleVerticalDragStart,
                            onVerticalDragUpdate:
                                (details) => _handleVerticalDragUpdate(
                                  details,
                                  availableHeight,
                                ),
                            onVerticalDragEnd: _handleVerticalDragEnd,
                            onVerticalDragCancel: _handleVerticalDragCancel,
                            child: Container(
                              width: 48,
                              height: scrollbarHeight,
                              alignment: Alignment.centerRight,
                              padding: const EdgeInsets.only(right: 10),
                              child: Container(
                                width: 6,
                                height: scrollbarHeight,
                                decoration: BoxDecoration(
                                  color: NotesColors.titleTextColor,
                                  borderRadius: BorderRadius.circular(3),
                                ),
                              ),
                            ),
                          ),
                        ),
                      ),

                      // Current section label - show during scrolling, dragging, or hovering
                      if (widget.isDragging || _isHoveringBar || _isScrolling)
                        Positioned(
                          right: 22,
                          top: (scrollbarPosition + scrollbarHeight / 2 - 18)
                              .clamp(10.0, availableHeight - 46),
                          child: ConstrainedBox(
                            constraints: BoxConstraints(
                              maxWidth: screenWidth - 120,
                            ),
                            child: Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 12,
                                vertical: 4,
                              ),
                              decoration: BoxDecoration(
                                color: NotesColors.titleTextColor,
                                borderRadius: BorderRadius.circular(30),
                              ),
                              child: Text(
                                sections[currentSection].label,
                                style: const TextStyle(
                                  color: NotesColors.cardColor,
                                  fontSize: 16,
                                  fontWeight: FontWeight.w500,
                                  letterSpacing: 0,
                                ),
                                maxLines: 1,
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                          ),
                        ),
                    ],
                  );
                },
              ),
            ),
          ],
        );
      },
    );
  }
}
