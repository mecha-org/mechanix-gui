import 'package:hive/hive.dart';

part 'song_info.g.dart';

@HiveType(typeId: 0)
class SongInfo extends HiveObject {
  @HiveField(0)
  String id;
  @HiveField(1)
  String path;
  @HiveField(2)
  String title;
  @HiveField(3)
  String artist;
  @HiveField(4)
  String? album;
  @HiveField(5)
  String? duration;
  @HiveField(6)
  bool isFavourite;
  @HiveField(7)
  String? artworkPath;
  @HiveField(8)
  int index;
  @HiveField(9)
  List<String> playlistIds = [];

  SongInfo({
    required this.id,
    required this.index,
    required this.path,
    required this.title,
    required this.artist,
    this.album,
    this.duration,
    this.artworkPath,
    this.isFavourite = false,
    this.playlistIds = const [],
  });

  SongInfo copyWith({
    String? id,
    int? index,
    String? path,
    String? title,
    String? artist,
    String? album,
    String? duration,
    String? artworkPath,
    bool? isFavourite,
    List<String>? playlistIds,
  }) {
    return SongInfo(
      id: id ?? this.id,
      index: index ?? this.index,
      path: path ?? this.path,
      title: title ?? this.title,
      artist: artist ?? this.artist,
      album: album ?? this.album,
      duration: duration ?? this.duration,
      isFavourite: isFavourite ?? this.isFavourite,
      artworkPath: artworkPath ?? this.artworkPath,
      playlistIds: playlistIds ?? this.playlistIds,
    );
  }
}
