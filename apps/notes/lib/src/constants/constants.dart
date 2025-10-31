class Images {
  static const String add = 'assets/images/add_icon.png';
  static const String back = 'assets/images/back_icon.png';
  static const String delete = 'assets/images/delete_icon.png';
  static const String selectAll = 'assets/images/select_all.png';
}

class Constants {
  static const String tableName = "notesTablev3";
  static const String t1Size = '14';
  static const String t2Size = '12';
  static const Duration debounceDuration = Duration(milliseconds: 300);
}

class ColorItem {
  final String name;
  final String color;

  const ColorItem({required this.name, required this.color});
}

final List<ColorItem> colorItems = [
  const ColorItem(name: "Yellow", color: "0xFFFFEA2D"),
  const ColorItem(name: "Red", color: "0xFFFF2D2D"),
  const ColorItem(name: "Green", color: "0xFF2DFF65"),
  const ColorItem(name: "Blue", color: "0xFF2D8AFF"),
  const ColorItem(name: "White", color: "0xFFD9D9D9"),
];
