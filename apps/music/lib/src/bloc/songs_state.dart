import 'package:equatable/equatable.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:media_kit/media_kit.dart';

class SongsState extends Equatable {
  final List<SongInfo> songs;
  final Playlist? playlist;
  final int currentIndex;
  final List<SongInfo> playbackQueue;
  final List<SongInfo>
  originalQueue; // Keep track of original order for shuffle
  final bool isLoading;
  final String? error;
  final List<SongInfo> searchedSongs;
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

  const SongsState({
    this.songs = const [],
    this.playlist,
    this.currentIndex = -1,
    this.playbackQueue = const [],
    this.originalQueue = const [],
    this.isLoading = false,
    this.error,
    this.searchedSongs = const [],
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
  });

  SongsState copyWith({
    List<SongInfo>? songs,
    Playlist? playlist,
    int? currentIndex,
    List<SongInfo>? playbackQueue,
    List<SongInfo>? originalQueue,
    bool? isLoading,
    String? error,
    List<SongInfo>? searchedSongs,
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
  }) {
    return SongsState(
      songs: songs ?? this.songs,
      playlist: playlist ?? this.playlist,
      currentIndex: currentIndex ?? this.currentIndex,
      playbackQueue: playbackQueue ?? this.playbackQueue,
      originalQueue: originalQueue ?? this.originalQueue,
      isLoading: isLoading ?? this.isLoading,
      error: error ?? this.error,
      searchedSongs: searchedSongs ?? this.searchedSongs,
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
    );
  }

  @override
  List<Object?> get props => [
    songs,
    playlist,
    currentIndex,
    playbackQueue,
    originalQueue,
    isLoading,
    error,
    searchedSongs,
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
  ];
}
