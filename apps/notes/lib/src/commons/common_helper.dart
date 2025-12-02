import 'package:intl/intl.dart';

class CommonHelper {
  static String formatDateTime(DateTime dateTime) {
    final now = DateTime.now();
    final diff = now.difference(dateTime);

    if (diff.inHours < 3) {
      if (diff.inMinutes < 1) {
        return "Just now";
      } else if (diff.inMinutes < 60) {
        return "${diff.inMinutes} minutes ago";
      } else {
        final hours = diff.inHours;
        return "$hours hour${hours > 1 ? 's' : ''} ago";
      }
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

    final time = DateFormat('h.mm a').format(dateTime);

    if (isToday) {
      return "Today, $time";
    } else if (isYesterday) {
      return "Yesterday, $time";
    } else {
      final date = DateFormat('MM d, yyyy').format(dateTime);
      return "$date, $time";
    }
  }
}
