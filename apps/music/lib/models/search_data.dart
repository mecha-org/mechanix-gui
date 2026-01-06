import 'package:equatable/equatable.dart';
import 'package:hive/hive.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';

part 'search_data.g.dart';

@HiveType(typeId: 3)
class SearchData extends HiveObject {
  @HiveField(0)
  final String id;

  /// true → playlist, false → song
  @HiveField(1)
  final bool isPlaylist;

  @HiveField(2)
  final String? songId;

  @HiveField(3)
  final String? playlistId;

  @HiveField(4)
  final DateTime createdAt;

  SearchData({
    required this.id,
    this.isPlaylist = false,
    this.songId,
    this.playlistId,
    required this.createdAt,
  }) : assert(
         isPlaylist ? playlistId != null : songId != null,
         'SearchInfo must contain either a song or a playlist',
       );

  SearchData copyWith({
    String? id,
    bool? isPlaylist,
    String? songId,
    String? playlistId,
    DateTime? createdAt,
  }) {
    return SearchData(
      id: id ?? this.id,
      isPlaylist: isPlaylist ?? this.isPlaylist,
      songId: songId ?? this.songId,
      playlistId: playlistId ?? this.playlistId,
      createdAt: createdAt ?? this.createdAt,
    );
  }
}

class SearchInfo extends Equatable {
  final String id;

  final bool isPlaylist;

  final SongInfo? songInfo;

  final PlaylistInfo? playlistInfo;

  final DateTime createdAt;

  const SearchInfo({
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

  @override
  List<Object?> get props => [
    id,
    isPlaylist,
    songInfo,
    playlistInfo,
    createdAt,
  ];
}
