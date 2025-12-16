// Enhanced height calculation that precisely matches UI rendering
// Add this to your backend logic file

import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

class NoteHeightCalculator {
  // UI Constants - Match exactly with your NotePreview widget
  static const double baseFontSize = 16.0;
  static const double lineHeight = 1.4;
  static const double baseLineHeight = baseFontSize * lineHeight; // 21px

  // Preview constraints
  static const int maxPreviewLines = 2; // From NotePreview maxLines parameter
  static const double maxWidth = 216.0; // Available width

  // Heading sizes (from NotePreview _buildHeadingSpans)
  static const double h1FontSize = 18.0;
  static const double h2FontSize = 16.0;

  // Code block styling (from NotePreview _buildCodeBlock)
  static const double codeFontSize = 16.0;
  static const double codeLineHeight = 1.4;
  static const double codePadding = 30.0; // 8px top + 8px bottom
  static const double codeMarginVertical =
      0.0; // No margin in current implementation
  static const double codeBorderRadius = 8.0;

  // Checkbox size (from NotePreview checkbox rendering)
  static const double checkboxSize = 18.0;
  static const double checkboxSpacing = 6.0;

  // Card layout constants (from _NoteGridItem)
  static const double cardPaddingVertical = 32.0; // 16px top + 16px bottom
  static const double titleHeight = 26.0; // SizedBox height
  static const double dateHeight = 18.0; // SizedBox height
  static const double columnSpacing = 16.0; // spacing: 16 in Column

  // Calculate total note card height
  static double calculateNoteHeight(String title, List<NoteLine> preview) {
    const double minHeight = 30.0;
    const double maxHeight = 200.0;

    double totalHeight = cardPaddingVertical;

    // Add title height if exists
    if (title.isNotEmpty) {
      totalHeight += titleHeight;
      totalHeight += columnSpacing; // Spacing after title
    }

    // Calculate preview height
    final previewHeight = calculatePreviewHeight(preview);
    if (previewHeight > 0) {
      totalHeight += previewHeight;
      totalHeight += columnSpacing; // Spacing after preview
    }

    // Add date height
    totalHeight += dateHeight;

    return totalHeight.clamp(minHeight, maxHeight);
  }

  // Calculate preview height matching Text.rich rendering logic
  static double calculatePreviewHeight(List<NoteLine> lines) {
    if (lines.isEmpty) return 0;

    double totalHeight = 0;
    int renderedLines = 0;
    bool reachedLimit = false;

    for (int i = 0; i < lines.length && !reachedLimit; i++) {
      final line = lines[i];
      final lineResult = _calculateLineHeight(
        line,
        renderedLines,
        maxPreviewLines,
      );

      totalHeight += lineResult.height;
      renderedLines += lineResult.lineCount;

      // Check if we've reached the line limit
      if (renderedLines >= maxPreviewLines) {
        reachedLimit = true;
        break;
      }

      // Add newline spacing between lines (except for last line and code blocks)
      if (i < lines.length - 1 && line.type != "code" && !reachedLimit) {
        // Newline is already factored into lineHeight, but we need to account
        // for the actual rendered spacing
        totalHeight += 0; // Already in lineHeight calculation
      }
    }

    return totalHeight;
  }

  static LineHeightResult _calculateLineHeight(
    NoteLine line,
    int currentLineCount,
    int maxLines,
  ) {
    final remainingLines = maxLines - currentLineCount;
    if (remainingLines <= 0) {
      return LineHeightResult(height: 0, lineCount: 0);
    }

    switch (line.type) {
      case "h1":
        return _calculateHeadingHeight(line, h1FontSize, remainingLines);

      case "h2":
        return _calculateHeadingHeight(line, h2FontSize, remainingLines);

      case "code":
        return _calculateCodeBlockHeight(line, remainingLines);

      case "bullet":
      case "number":
        return _calculateListItemHeight(line, remainingLines);

      case "checkbox":
        return _calculateCheckboxHeight(line, remainingLines);

      case "quote":
        return _calculateQuoteHeight(line, remainingLines);

      case "text":
      default:
        return _calculateTextHeight(line, remainingLines);
    }
  }

  static LineHeightResult _calculateTextHeight(
    NoteLine line,
    int remainingLines,
  ) {
    final text = line.spans.map((s) => s.text).join();
    final wrappedLines = _estimateWrappedLines(
      text,
      maxWidth,
      baseFontSize,
      isBold: line.spans.any((s) => s.bold),
    );

    final actualLines = wrappedLines.clamp(1, remainingLines);
    final height = actualLines * baseLineHeight;

    return LineHeightResult(height: height, lineCount: actualLines);
  }

  static LineHeightResult _calculateHeadingHeight(
    NoteLine line,
    double fontSize,
    int remainingLines,
  ) {
    final text = line.spans.map((s) => s.text).join();
    final wrappedLines = _estimateWrappedLines(
      text,
      maxWidth,
      fontSize,
      isBold: true,
    );

    final actualLines = wrappedLines.clamp(1, remainingLines);
    final height = actualLines * (fontSize * lineHeight);

    return LineHeightResult(height: height, lineCount: actualLines);
  }

  static LineHeightResult _calculateCodeBlockHeight(
    NoteLine line,
    int remainingLines,
  ) {
    final codeText = line.spans.map((s) => s.text).join();

    // Code blocks are rendered in a Container with padding
    // The text inside can wrap
    final effectiveWidth = maxWidth - 16; // 8px padding on each side

    final wrappedLines = _estimateWrappedLines(
      codeText,
      effectiveWidth,
      codeFontSize,
      isMonospace: true,
    );

    // Code blocks can take multiple lines but are constrained by maxLines
    final actualLines = wrappedLines.clamp(1, remainingLines);

    // Height = content height + padding
    final contentHeight = actualLines * (codeFontSize * codeLineHeight);
    final totalHeight = contentHeight + codePadding + codeMarginVertical;

    return LineHeightResult(height: totalHeight, lineCount: actualLines);
  }

  static LineHeightResult _calculateListItemHeight(
    NoteLine line,
    int remainingLines,
  ) {
    final text = line.spans.map((s) => s.text).join();

    // Account for bullet/number prefix width
    final prefixWidth = line.type == "bullet" ? 16.0 : 24.0;
    final effectiveWidth = maxWidth - prefixWidth;

    final wrappedLines = _estimateWrappedLines(
      text,
      effectiveWidth,
      baseFontSize,
      isBold: line.spans.any((s) => s.bold),
    );

    final actualLines = wrappedLines.clamp(1, remainingLines);
    final height = actualLines * baseLineHeight;

    return LineHeightResult(height: height, lineCount: actualLines);
  }

  static LineHeightResult _calculateCheckboxHeight(
    NoteLine line,
    int remainingLines,
  ) {
    final text = line.spans.map((s) => s.text).join();

    // Account for checkbox width + spacing
    final effectiveWidth = maxWidth - checkboxSize - checkboxSpacing;

    final wrappedLines = _estimateWrappedLines(
      text,
      effectiveWidth,
      baseFontSize,
      isBold: line.spans.any((s) => s.bold),
    );

    final actualLines = wrappedLines.clamp(1, remainingLines);

    // Height is max of checkbox size and text height
    final textHeight = actualLines * baseLineHeight;
    final height = textHeight.clamp(checkboxSize, double.infinity);

    return LineHeightResult(height: height, lineCount: actualLines);
  }

  static LineHeightResult _calculateQuoteHeight(
    NoteLine line,
    int remainingLines,
  ) {
    final text = line.spans.map((s) => s.text).join();

    // Account for quote bar prefix
    const double quoteBarWidth = 16.0;
    final effectiveWidth = maxWidth - quoteBarWidth;

    final wrappedLines = _estimateWrappedLines(
      text,
      effectiveWidth,
      baseFontSize,
      isBold: line.spans.any((s) => s.bold),
    );

    final actualLines = wrappedLines.clamp(1, remainingLines);
    final height = actualLines * baseLineHeight;

    return LineHeightResult(height: height, lineCount: actualLines);
  }

  // Enhanced text wrapping estimation
  static int _estimateWrappedLines(
    String text,
    double maxWidth,
    double fontSize, {
    bool isBold = false,
    bool isMonospace = false,
  }) {
    if (text.isEmpty) return 1;

    // Character width factors based on font type
    // These are empirical values - adjust based on your actual fonts
    double charWidthFactor;
    if (isMonospace) {
      charWidthFactor = 0.6; // Monospace fonts are wider
    } else if (isBold) {
      charWidthFactor = 0.52; // Bold text is slightly wider
    } else {
      charWidthFactor = 0.48; // Regular text
    }

    final double avgCharWidth = fontSize * charWidthFactor;
    final int charsPerLine = (maxWidth / avgCharWidth).floor();

    if (charsPerLine <= 0) return 1;

    // Account for word wrapping by reducing effective characters per line
    final int effectiveCharsPerLine = (charsPerLine * 0.85).floor();

    // Split by actual newlines first
    final lines = text.split('\n');
    int totalLines = 0;

    for (final line in lines) {
      if (line.isEmpty) {
        totalLines += 1;
      } else {
        totalLines += (line.length / effectiveCharsPerLine).ceil();
      }
    }

    return totalLines.clamp(1, maxPreviewLines);
  }
}

// Helper class to return both height and line count
class LineHeightResult {
  final double height;
  final int lineCount;

  LineHeightResult({required this.height, required this.lineCount});
}
