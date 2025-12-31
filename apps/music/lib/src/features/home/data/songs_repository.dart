import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';

abstract class SongsRepository {
  Future<List<SongInfo>> getAllSongs();
  Future<List<SongInfo>> scanAllSongs();
  Future<SongInfo> deleteSong(SongInfo songInfo);
  Future<bool> toggleFavouriteSong(SongInfo songInfo, bool isFavourite);
  Future<void> addToRecentlyPlayed(SongInfo songInfo);
  Future<List<SongInfo>> getRecentlyPlayed();
  Future<bool> createPlaylist(String playlistName);
  Future<List<PlaylistInfo>> getPlaylist();
}
