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
  final bool? isShuffle;

  const CurrentPlaylist({
    this.playlistId,
    this.currentIndex,
    this.currentSongId,
    this.isShuffle,
  });

  CurrentPlaylist copyWith({
    String? playlistId,
    List<String>? playlistSongsIds,
    int? currentIndex,
    String? currentSongId,
    bool? isShuffle = false,
  }) {
    return CurrentPlaylist(
      playlistId: playlistId ?? this.playlistId,
      currentIndex: currentIndex,
      currentSongId: currentSongId,
      isShuffle: isShuffle ?? this.isShuffle,
    );
  }

  CurrentPlaylist mergeWith(CurrentPlaylist other) {
    return CurrentPlaylist(
      playlistId: other.playlistId,
      currentIndex: other.currentIndex,
      currentSongId: other.currentSongId,
      isShuffle: other.isShuffle,
    );
  }

  @override
  List<Object?> get props => [
    playlistId,
    currentIndex,
    currentSongId,
    isShuffle,
  ];
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
