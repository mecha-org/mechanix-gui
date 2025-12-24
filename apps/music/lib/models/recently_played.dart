import 'package:hive/hive.dart';
import 'package:mechanix_music/models/song_info.dart';

part 'recently_played.g.dart';

@HiveType(typeId: 1)
class RecentlyPlayed extends HiveObject {
  @HiveField(0)
  SongInfo song;

  @HiveField(1)
  DateTime lastPlayedAt;

  RecentlyPlayed({required this.song, required this.lastPlayedAt});

  RecentlyPlayed copyWith({SongInfo? song, DateTime? lastPlayedAt}) {
    return RecentlyPlayed(
      song: song ?? this.song,
      lastPlayedAt: lastPlayedAt ?? this.lastPlayedAt,
    );
  }
}
