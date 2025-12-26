import 'package:hive/hive.dart';

part 'playlist_info.g.dart';

@HiveType(typeId: 2)
class PlaylistInfo extends HiveObject {
  @HiveField(0)
  String id;

  @HiveField(1)
  String name;

  @HiveField(2)
  List<String> songIds; // Store only song IDs

  @HiveField(3)
  DateTime createdAt;

  @HiveField(4)
  DateTime updatedAt;

  @HiveField(5)
  String? coverImagePath;

  PlaylistInfo({
    required this.id,
    required this.name,
    required this.songIds,
    required this.createdAt,
    required this.updatedAt,
    this.coverImagePath,
  });

  PlaylistInfo copyWith({
    String? id,
    String? name,
    List<String>? songIds,
    DateTime? createdAt,
    DateTime? updatedAt,
    String? description,
    String? coverImagePath,
  }) {
    return PlaylistInfo(
      id: id ?? this.id,
      name: name ?? this.name,
      songIds: songIds ?? this.songIds,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      coverImagePath: coverImagePath,
    );
  }
}
