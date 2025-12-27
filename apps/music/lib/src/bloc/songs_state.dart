import 'package:equatable/equatable.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/search_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:media_kit/media_kit.dart';

class SongsState extends Equatable {
  final List<SongInfo> songs;
  final Playlist? playlist;
  final List<SongInfo> playbackQueue;

  final bool isLoading;
  final String? error;
  final SearchResults searchResults;
  final bool isPlaying;
  final SongInfo? currentSong;
  final Duration position;
  final Duration duration;
  final RepeatMode repeatMode;
  final bool isShuffled;
  final MusicTabs musicTab;
  final List<SongInfo> recentlyPlayedSongs;
  final BottomBarView bottomBarView;
  final List<PlaylistInfo> playlists;
  final PlaylistViewEnum playlistView;
  final List<SongInfo> playlistSongs;
  final MusicMode musicMode;
  final CurrentPlaylist currentPlaylist;
  final List<SearchInfo> searchItems;

  const SongsState({
    this.songs = const [],
    this.playlist,
    this.playbackQueue = const [],
    this.isLoading = false,
    this.error,
    this.isPlaying = false,
    this.position = Duration.zero,
    this.duration = Duration.zero,
    this.currentSong,
    this.repeatMode = RepeatMode.none,
    this.isShuffled = false,
    this.musicTab = MusicTabs.music,
    this.recentlyPlayedSongs = const [],
    this.bottomBarView = BottomBarView.normal,
    this.playlists = const [],
    this.playlistView = PlaylistViewEnum.list,
    this.playlistSongs = const [],
    this.musicMode = MusicMode.normal,
    this.currentPlaylist = const CurrentPlaylist(),
    this.searchResults = const SearchResults(
      query: "",
      playlists: [],
      songs: [],
    ),
    this.searchItems = const [],
  });

  SongsState copyWith({
    List<SongInfo>? songs,
    Playlist? playlist,
    List<SongInfo>? playbackQueue,
    List<SongInfo>? originalQueue,
    bool? isLoading,
    String? error,
    SearchResults? searchResults,
    bool? isPlaying,
    Duration? position,
    Duration? duration,
    SongInfo? currentSong,
    RepeatMode? repeatMode,
    bool? isShuffled,
    MusicTabs? musicTab,
    List<SongInfo>? recentlyPlayedSongs,
    BottomBarView? bottomBarView,
    List<PlaylistInfo>? playlists,
    PlaylistViewEnum? playlistView,
    List<SongInfo>? playlistSongs,
    MusicMode? musicMode,
    CurrentPlaylist? currentPlaylist,
    List<SearchInfo>? searchItems,
  }) {
    return SongsState(
      songs: songs ?? this.songs,
      playlist: playlist ?? this.playlist,
      playbackQueue: playbackQueue ?? this.playbackQueue,
      isLoading: isLoading ?? this.isLoading,
      error: error ?? this.error,
      searchResults: searchResults ?? this.searchResults,
      isPlaying: isPlaying ?? this.isPlaying,
      position: position ?? this.position,
      duration: duration ?? this.duration,
      currentSong: currentSong ?? this.currentSong,
      repeatMode: repeatMode ?? this.repeatMode,
      isShuffled: isShuffled ?? this.isShuffled,
      musicTab: musicTab ?? this.musicTab,
      recentlyPlayedSongs: recentlyPlayedSongs ?? this.recentlyPlayedSongs,
      bottomBarView: bottomBarView ?? this.bottomBarView,
      playlists: playlists ?? this.playlists,
      playlistView: playlistView ?? this.playlistView,
      playlistSongs: playlistSongs ?? this.playlistSongs,
      musicMode: musicMode ?? this.musicMode,
      currentPlaylist: currentPlaylist ?? this.currentPlaylist,
      searchItems: searchItems ?? this.searchItems,
    );
  }

  @override
  List<Object?> get props => [
    songs,
    playlist,
    playbackQueue,
    isLoading,
    error,
    searchResults,
    isPlaying,
    position,
    duration,
    currentSong,
    repeatMode,
    isShuffled,
    musicTab,
    recentlyPlayedSongs,
    bottomBarView,
    playlists,
    playlistView,
    playlistSongs,
    musicMode,
    currentPlaylist,
    searchItems
  ];
}
