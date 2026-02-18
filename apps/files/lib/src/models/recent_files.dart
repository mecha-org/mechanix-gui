import 'package:hive/hive.dart';

part 'recent_files.g.dart';

@HiveType(typeId: 2)
class RecentFile extends HiveObject {
  @HiveField(0)
  String path;

  @HiveField(1)
  DateTime openedAt;

  RecentFile({required this.path, required this.openedAt});

  @override
  String toString() {
    return 'RecentFile(path: $path, openedAt: $openedAt)';
  }

  RecentFile copyWith({String? path, DateTime? openedAt}) {
    return RecentFile(
      path: path ?? this.path,
      openedAt: openedAt ?? this.openedAt,
    );
  }
}
