import 'package:mechanix_music/models/models.dart';
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
  Future<bool> deletePlaylist(String playlistId);
  Future<List<SongInfo>> addToPlaylist(
    List<String> playlistIds,
    List<String> songIds,
  );
  Future<List<SongInfo>> getPlaylistSongs(String playlistId);
  Future<bool> updatePlaylistSongs(
    String playlistId,
    List<String> orderedSongIds,
    List<String> deletedSongIds,
  );

  Future<SearchResults> searchSongs(String query);
}
