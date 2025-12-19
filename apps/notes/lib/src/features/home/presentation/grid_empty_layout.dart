import 'package:flutter/material.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:widgets/widgets.dart';

class _NoteCard extends StatelessWidget {
  final double height;
  final bool isLoading;
  final Animation<double>? shimmerAnimation;
  final VoidCallback? onTap;
  final Widget? child;

  const _NoteCard({
    required this.height,
    this.isLoading = false,
    this.shimmerAnimation,
    this.onTap,
    this.child,
  });

  @override
  Widget build(BuildContext context) {
    final container = Container(
      height: height,
      decoration: BoxDecoration(
        color: isLoading ? null : NotesColors.cardColor,
        borderRadius: BorderRadius.circular(8),
        gradient:
            isLoading && shimmerAnimation != null
                ? LinearGradient(
                  begin: Alignment.centerLeft,
                  end: Alignment.centerRight,
                  stops: const [0.0, 0.5, 1.0],
                  colors: const [
                    Color(0xFF1A1A1A),
                    Color(0xFF2A2A2A),
                    Color(0xFF1A1A1A),
                  ],
                  transform: _SlideGradientTransform(shimmerAnimation!.value),
                )
                : null,
      ),
      child: child,
    );

    if (onTap != null) {
      return GestureDetector(onTap: onTap, child: container);
    }

    return container;
  }
}

class NotesGridLayout extends StatelessWidget {
  final List<double> leftColumnHeights;
  final List<double> rightColumnHeights;
  final bool isLoading;
  final Animation<double>? shimmerAnimation;
  final int? interactiveCardIndex; // Index of card that should be tappable

  const NotesGridLayout({
    super.key,
    required this.leftColumnHeights,
    required this.rightColumnHeights,
    this.isLoading = false,
    this.shimmerAnimation,
    this.interactiveCardIndex,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: LayoutBuilder(
        builder: (context, constraints) {
          final columnWidth = (constraints.maxWidth - 12) / 2;

          return Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Left Column
              SizedBox(
                width: columnWidth,
                child: Column(
                  children: List.generate(leftColumnHeights.length, (index) {
                    final isFirstCard = index == 0 && interactiveCardIndex == 0;
                    return Padding(
                      padding: EdgeInsets.only(
                        bottom: index < leftColumnHeights.length - 1 ? 12 : 0,
                      ),
                      child:
                          isLoading
                              ? AnimatedBuilder(
                                animation: shimmerAnimation!,
                                builder:
                                    (context, child) => _NoteCard(
                                      height: leftColumnHeights[index],
                                      isLoading: true,
                                      shimmerAnimation: shimmerAnimation,
                                    ),
                              )
                              : _NoteCard(
                                height: leftColumnHeights[index],
                                onTap:
                                    isFirstCard
                                        ? () => Navigator.pushNamed(
                                          context,
                                          AppRoutes.createEditNotes,
                                        )
                                        : null,
                                child:
                                    isFirstCard
                                        ? const Center(
                                          child: IconWidget(
                                            iconColor:
                                                NotesColors.secondaryCardColor,
                                            iconPath: NotesIcon.addIcon,
                                            boxHeight: 48,
                                            boxWidth: 48,
                                            iconHeight: 36,
                                            iconWidth: 36,
                                          ),
                                        )
                                        : null,
                              ),
                    );
                  }),
                ),
              ),
              const SizedBox(width: 12),
              // Right Column
              SizedBox(
                width: columnWidth,
                child: Column(
                  children: List.generate(
                    rightColumnHeights.length,
                    (index) => Padding(
                      padding: EdgeInsets.only(
                        bottom: index < rightColumnHeights.length - 1 ? 12 : 0,
                      ),
                      child:
                          isLoading
                              ? AnimatedBuilder(
                                animation: shimmerAnimation!,
                                builder:
                                    (context, child) => _NoteCard(
                                      height: rightColumnHeights[index],
                                      isLoading: true,
                                      shimmerAnimation: shimmerAnimation,
                                    ),
                              )
                              : _NoteCard(height: rightColumnHeights[index]),
                    ),
                  ),
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}
 
// GRADIENT TRANSFORM - Reusable for shimmer effect
class _SlideGradientTransform extends GradientTransform {
  final double slidePercent;

  const _SlideGradientTransform(this.slidePercent);

  @override
  Matrix4? transform(Rect bounds, {TextDirection? textDirection}) {
    return Matrix4.translationValues(bounds.width * slidePercent, 0.0, 0.0);
  }
}
