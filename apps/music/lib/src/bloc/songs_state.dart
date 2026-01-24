import 'package:equatable/equatable.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/search_data.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';

class SongsState extends Equatable {
  final List<SongInfo> songs;
  final List<SongInfo> playbackQueue;
  final List<SongInfo> originalQueue;
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
  final bool isMediaKitInitializing;
  final List<SearchInfo> searchItems;
  final List<MusicTabs> tabHistory;
  final PlaylistInfo? selectedPlaylist;
  final List<PlaylistInfo> searchedPlaylist;
  final List<SongInfo> searchedSongs;
  final List<SongInfo> favouriteSongs;
  final int? currentIndex;
  final bool isScrolling;

  const SongsState({
    this.songs = const [],
    this.playbackQueue = const [],
    this.originalQueue = const [],
    this.isLoading = false,
    this.error,
    this.isPlaying = false,
    this.position = Duration.zero,
    this.duration = Duration.zero,
    this.currentSong,
    this.repeatMode = RepeatMode.none,
    this.isShuffled = false,
    this.musicTab = MusicTabs.home,
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
    this.tabHistory = const [],
    this.selectedPlaylist,
    this.searchedPlaylist = const [],
    this.searchedSongs = const [],
    this.favouriteSongs = const [],
    this.currentIndex,
    this.isScrolling = false,
    this.isMediaKitInitializing = true,
  });

  SongsState copyWith({
    List<SongInfo>? songs,
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
    List<MusicTabs>? tabHistory,
    PlaylistInfo? selectedPlaylist,
    List<PlaylistInfo>? searchedPlaylist,
    List<SongInfo>? searchedSongs,
    List<SongInfo>? favouriteSongs,
    int? currentIndex,
    bool? isScrolling,
    bool? isMediaKitInitializing,
  }) {
    return SongsState(
      songs: songs ?? this.songs,
      playbackQueue: playbackQueue ?? this.playbackQueue,
      originalQueue: originalQueue ?? this.originalQueue,
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
      tabHistory: tabHistory ?? this.tabHistory,
      selectedPlaylist: selectedPlaylist ?? this.selectedPlaylist,
      searchedPlaylist: searchedPlaylist ?? this.searchedPlaylist,
      searchedSongs: searchedSongs ?? this.searchedSongs,
      favouriteSongs: favouriteSongs ?? this.favouriteSongs,
      currentIndex: currentIndex ?? this.currentIndex,
      isScrolling: isScrolling ?? this.isScrolling,
      isMediaKitInitializing:
          isMediaKitInitializing ?? this.isMediaKitInitializing,
    );
  }

  @override
  List<Object?> get props => [
    songs,
    playbackQueue,
    originalQueue,
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
    searchItems,
    tabHistory,
    selectedPlaylist,
    searchedPlaylist,
    searchedSongs,
    favouriteSongs,
    currentIndex,
    isScrolling,
    isMediaKitInitializing,
  ];
}
