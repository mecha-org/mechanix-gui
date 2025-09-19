import 'dart:typed_data';
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
  Uint8List? artwork;

  @HiveField(8)
  int index;

  SongInfo({
    required this.id,
    required this.path,
    required this.title,
    required this.artist,
    this.album,
    this.duration,
    this.artwork,
    required this.index,
    this.isFavourite = false,
  });
}
