import 'package:intl/intl.dart';

class CommonHelper {
  static String formatDateTime(DateTime dateTime) {
    final now = DateTime.now();
    final yesterday = now.subtract(Duration(days: 1));

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
      final date = DateFormat('MMMM d, yyyy').format(dateTime);
      return "$date, $time";
    }
  }
}
