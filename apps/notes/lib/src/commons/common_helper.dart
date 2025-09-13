class CommonHelper {

  static String formatDateTime(DateTime dateTime) {
    final now = DateTime.now();
    final isToday =
        now.year == dateTime.year &&
        now.month == dateTime.month &&
        now.day == dateTime.day;

    final date =
        isToday
            ? "Today"
            : "${dateTime.day.toString().padLeft(2, '0')}/"
                "${dateTime.month.toString().padLeft(2, '0')}/"
                "${dateTime.year}";

    final time =
        "${dateTime.hour.toString().padLeft(2, '0')}:"
        "${dateTime.minute.toString().padLeft(2, '0')}";

    return "$date at $time";
  }
}