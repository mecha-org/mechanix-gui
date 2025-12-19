import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/commons/styles/styles.dart';

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
      return Text(text, style: isTitle ? titleStyle : normalStyle);
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
              style: isTitle ? titleStyle : normalStyle,
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
            style: isTitle ? titleStyle : normalStyle,
          ),
        );
      }

      // Add highlighted match
      spans.add(
        TextSpan(
          text: text.substring(index, index + query.length),
          style:
              isTitle
                  ? titleStyle.copyWith(
                    backgroundColor: NotesColors.highlightTextColor.withValues(
                      alpha: 0.65,
                    ),
                  )
                  : normalStyle.copyWith(
                    backgroundColor: NotesColors.highlightTextColor.withValues(
                      alpha: 0.65,
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
