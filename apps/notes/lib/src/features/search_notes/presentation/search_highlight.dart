import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/styles.dart';
import 'package:widgets/extensions/color.dart';

class SearchHighlight extends StatelessWidget {
  final String text;
  final String query;
  final bool isTitle;
  const SearchHighlight({
    super.key,
    required this.text,
    required this.query,
    required this.isTitle,
  });

  @override
  Widget build(BuildContext context) {
    // Helper Widget to build highlighted text
    if (query.isEmpty) {
      return Text(
        text,
        style: isTitle ? titleStyle(context) : normalStyle(context),
      );
    }

    final lowerText = text.toLowerCase();
    final lowerQuery = query.toLowerCase();
    final spans = <TextSpan>[];
    int start = 0;

    while (true) {
      final index = lowerText.indexOf(lowerQuery, start);
      if (index == -1) {
        // Add remaining text
        if (start < text.length) {
          spans.add(
            TextSpan(
              text: text.substring(start),
              style: isTitle ? titleStyle(context) : normalStyle(context),
            ),
          );
        }
        break;
      }

      // Add text before match
      if (index > start) {
        spans.add(
          TextSpan(
            text: text.substring(start, index),
            style: isTitle ? titleStyle(context) : normalStyle(context),
          ),
        );
      }

      // Add highlighted match
      spans.add(
        TextSpan(
          text: text.substring(index, index + query.length),
          style:
              isTitle
                  ? titleStyle(context).copyWith(
                    backgroundColor: context.primaryContainer.withValues(
                      alpha: 0.8,
                    ),
                  )
                  : normalStyle(context).copyWith(
                    backgroundColor: context.primaryContainer.withValues(
                      alpha: 0.8,
                    ),
                  ),
        ),
      );

      start = index + query.length;
    }

    return RichText(
      overflow: TextOverflow.ellipsis,
      text: TextSpan(children: spans),
    );
  }
}
