
import 'package:hive/hive.dart';

part 'note_hive.g.dart';

@HiveType(typeId: 0)
class NoteHive extends HiveObject {
  @HiveField(0)
  String id;

  @HiveField(1)
  String title;

  @HiveField(2)
  String content;

  @HiveField(3)
  DateTime createdAt;

  @HiveField(4)
  DateTime updatedAt;

  @HiveField(5)
  String plainText;

  @HiveField(6)
  bool isPinned;

  @HiveField(7)
  String tag;

  @HiveField(8)
  String preview;
  
  @HiveField(9)
  double height;

  NoteHive({
    required this.id,
    required this.title,
    required this.content,
    required this.createdAt,
    required this.updatedAt,
    required this.plainText,
    this.isPinned = false,
    this.tag = 'none',
    required this.preview,
    required this.height,
  });
}
