import 'package:mechanix_music/models/song_info.dart';

abstract class SongsRepository {
  Future<List<SongInfo>> getAllSongs();
  Future<List<SongInfo>> scanAllSongs();
  Future<SongInfo> deleteSong(SongInfo songInfo);
  Future<bool> toggleFavouriteSong(SongInfo songInfo, bool isFavourite);
}
