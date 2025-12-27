import 'package:hive/hive.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';

part 'search_info.g.dart';

@HiveType(typeId: 3)
class SearchInfo extends HiveObject {
  @HiveField(0)
  final String id;

  /// true → playlist, false → song
  @HiveField(1)
  final bool isPlaylist;

  @HiveField(2)
  final SongInfo? songInfo;

  @HiveField(3)
  final PlaylistInfo? playlistInfo;

  @HiveField(4)
  final DateTime createdAt;

  SearchInfo({
    required this.id,
    this.isPlaylist = false,
    this.songInfo,
    this.playlistInfo,
    required this.createdAt,
  }) : assert(
         isPlaylist ? playlistInfo != null : songInfo != null,
         'SearchInfo must contain either a song or a playlist',
       );

  SearchInfo copyWith({
    String? id,
    bool? isPlaylist,
    SongInfo? songInfo,
    PlaylistInfo? playlistInfo,
    DateTime? createdAt,
  }) {
    return SearchInfo(
      id: id ?? this.id,
      isPlaylist: isPlaylist ?? this.isPlaylist,
      songInfo: songInfo ?? this.songInfo,
      playlistInfo: playlistInfo ?? this.playlistInfo,
      createdAt: createdAt ?? this.createdAt,
    );
  }
}
