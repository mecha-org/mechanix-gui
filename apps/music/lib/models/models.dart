import 'package:equatable/equatable.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';

enum MusicTabs { home, search, music, playlists, favorites }

enum BottomBarView { normal, text, search, add }

enum PlaylistViewEnum { grid, list }

enum MusicMode { normal, playlist }

class CurrentPlaylist extends Equatable {
  final String? playlistId;
  final int? currentIndex;
  final String? currentSongId;

  const CurrentPlaylist({
    this.playlistId,
    this.currentIndex,
    this.currentSongId,
  });

  CurrentPlaylist copyWith({
    String? playlistId,
    List<String>? playlistSongsIds,
    int? currentIndex,
    String? currentSongId,
  }) {
    return CurrentPlaylist(
      playlistId: playlistId ?? this.playlistId,
      currentIndex: currentIndex,
      currentSongId: currentSongId,
    );
  }

  CurrentPlaylist mergeWith(CurrentPlaylist other) {
    return CurrentPlaylist(
      playlistId: other.playlistId,
      currentIndex: other.currentIndex,
      currentSongId: other.currentSongId,
    );
  }

  @override
  List<Object?> get props => [playlistId, currentIndex, currentSongId];
}

class SearchResults {
  final String query;
  final List<SongInfo> songs;
  final List<PlaylistInfo> playlists;
  const SearchResults({
    required this.query,
    required this.songs,
    required this.playlists,
  });
}
