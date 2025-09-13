import 'package:hive/hive.dart';


@HiveType(typeId: 0)
class Note extends HiveObject {
  @HiveField(0)
  int id;

  @HiveField(1)
  String title;

  @HiveField(2)
  String content;

  Note({
    required this.id,
    required this.title,
    required this.content,
  });
}
