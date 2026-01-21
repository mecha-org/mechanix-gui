import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:widgets/mechanix.dart';

class SectionDotsWidget extends StatelessWidget {
  final List<SectionInfo> sections;
  final double totalContentHeight;
  final double availableHeight;
  final int currentSection;
  final int? hoveredDotIndex;
  final void Function(int index) onDotTap;
  final void Function(int index) onDotHoverEnter;
  final void Function() onDotHoverExit;

  const SectionDotsWidget({
    super.key,
    required this.sections,
    required this.totalContentHeight,
    required this.availableHeight,
    required this.currentSection,
    required this.hoveredDotIndex,
    required this.onDotTap,
    required this.onDotHoverEnter,
    required this.onDotHoverExit,
  });

  @override
  Widget build(BuildContext context) {
    if (totalContentHeight <= 0) return const SizedBox.shrink();

    return Stack(
      clipBehavior: Clip.none,
      children: [
        for (int i = 0; i < sections.length; i++) _buildSectionDot(i, context),
      ],
    );
  }

  Widget _buildSectionDot(int index, BuildContext context) {
    final section = sections[index];
    final relativePosition = section.offset / totalContentHeight;
    final topPosition = relativePosition * availableHeight;
    // for equal height section
    // double gap = sections.length > 1
    //     ? availableHeight / (sections.length - 1)
    //     : 0.0;

    // double y = i * gap;

    final isActive = index == currentSection;
    final isHovered = hoveredDotIndex == index;

    return Positioned(
      top: topPosition.clamp(0.0, availableHeight - 40),
      left: 0,
      right: 10,
      child: GestureDetector(
        onTap: () => onDotTap(index),
        child: MouseRegion(
          onEnter: (_) => onDotHoverEnter(index),
          onExit: (_) => onDotHoverExit(),
          child: Container(
            height: 40,
            alignment: Alignment.centerRight,
            child: Row(
              mainAxisSize: MainAxisSize.min,
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                // Dot - 4x4 square
                Container(
                  width: 4,
                  height: 4,
                  decoration: BoxDecoration(
                    color:
                        isActive
                            ? context.primary
                            : isHovered
                            ? context.surfaceContainerHigh.withValues(
                              alpha: 0.7,
                            )
                            : context.surfaceContainerHigh.withValues(
                              alpha: 0.4,
                            ),
                    borderRadius: BorderRadius.circular(1),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
