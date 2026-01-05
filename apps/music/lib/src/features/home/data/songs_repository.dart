import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/search_data.dart';
import 'package:mechanix_music/models/song_info.dart';

abstract class SongsRepository {
  Future<List<SongInfo>> getAllSongs();
  Future<List<SongInfo>> scanAllSongs();
  Future<SongInfo> deleteSong(SongInfo songInfo);
  Future<bool> toggleFavouriteSong(List<String> songInfo, bool isFavourite);
  Future<void> addToRecentlyPlayed(SongInfo songInfo);
  Future<List<SongInfo>> getRecentlyPlayed();
  Future<bool> createUpdatePlaylist(String playlistName, String? playlistId);
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

  Future<bool> shuffleToggle(String playlistId, bool isShuffle);
  Future<bool> storeSearchItem({
    PlaylistInfo? playlistInfo,
    SongInfo? songInfo,
  });

  Future<List<SearchInfo>> getStoredSearchItems();
  Future<bool> clearSearchItems({String? searchId, required bool clearAll});
  Future<PlaylistInfo?> getSelectedPlaylist({required String playlistId});
  Future<List<PlaylistInfo>> searchedPlaylist({required String query});
  Future<List<SongInfo>> searchedSong({required String query});
  Future<List<SongInfo>> getFavouriteSongs();

  Future<void> addSongFromPath(String path);
  Future<void> updateSongFromPath(String path);
  Future<void> removeSongByPath(String path);
}
