import 'package:intl/intl.dart';

class CommonHelper {
  static String formatDateTime(DateTime dateTime) {
    final now = DateTime.now();
    final diff = now.difference(dateTime);

    // Less than 1 hour → Just now
    if (diff.inMinutes < 60) {
      return "Just now";
    }

    final yesterday = now.subtract(const Duration(days: 1));

    final isToday =
        now.year == dateTime.year &&
        now.month == dateTime.month &&
        now.day == dateTime.day;

    final isYesterday =
        yesterday.year == dateTime.year &&
        yesterday.month == dateTime.month &&
        yesterday.day == dateTime.day;

    final time24 = DateFormat('HH:mm').format(dateTime);

    if (isToday) {
      return "Today, $time24";
    } else if (isYesterday) {
      return "Yesterday, $time24";
    }

    // If same year → "Nov 2"
    if (now.year == dateTime.year) {
      return DateFormat('MMM d').format(dateTime);
    }

    // If different year → "Nov 2, 2024"
    return DateFormat('MMM d, yyyy').format(dateTime);
  }

  static String formatSectionLabel(String label) {
    final now = DateTime.now();
    final currentYear = now.year;

    // These labels should remain as they are
    const fixedLabels = {
      "Today",
      "Yesterday",
      "Recent",
      "This Week",
      "This Month",
      "Last Month",
    };

    if (fixedLabels.contains(label)) {
      return label; // Do not change
    }

    // Handle labels like: January 2025
    try {
      final parsedDate = DateFormat("MMMM yyyy").parse(label);
      final monthShort = DateFormat("MMM").format(parsedDate);

      if (parsedDate.year == currentYear) {
        return monthShort; // Jan
      } else {
        return "$monthShort ${parsedDate.year}"; // Jan 2024
      }
    } catch (_) {
      // If parsing fails, just return the label
      return label;
    }
  }
}
