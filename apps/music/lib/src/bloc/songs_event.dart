import 'package:equatable/equatable.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';

abstract class SongsEvent extends Equatable {
  const SongsEvent();

  @override
  List<Object?> get props => [];
}

class ScanSongs extends SongsEvent {
  const ScanSongs();
}

class LoadSongsFromHive extends SongsEvent {}

class SearchSong extends SongsEvent {
  final String searchQuery;

  const SearchSong(this.searchQuery);
}

class PlaySong extends SongsEvent {
  final SongInfo song;
  const PlaySong(this.song);
}

class TogglePlayPause extends SongsEvent {}

class PlayNext extends SongsEvent {}

class ShuffleToggle extends SongsEvent {
  final String playlistId;
  final bool isShuffle;
  const ShuffleToggle({required this.playlistId, required this.isShuffle});
}

// class  ToggleRepeat extends SongsEvent {}
// class FavouriteToggle extends SongsEvent {}

class PlayPrevious extends SongsEvent {}

class SeekSong extends SongsEvent {
  final Duration position;
  const SeekSong(this.position);

  @override
  List<Object?> get props => [position];
}

// Additional events for better player state management
class UpdatePosition extends SongsEvent {
  final Duration position;
  const UpdatePosition(this.position);

  @override
  List<Object?> get props => [position];
}

class UpdateDuration extends SongsEvent {
  final Duration duration;
  const UpdateDuration(this.duration);

  @override
  List<Object?> get props => [duration];
}

class SetRepeatMode extends SongsEvent {
  final RepeatMode mode;
  const SetRepeatMode(this.mode);

  @override
  List<Object?> get props => [mode];
}

// Enum for repeat modes
enum RepeatMode { none, one, all }

class MusicTabSwitch extends SongsEvent {
  final MusicTabs musicTab;
  const MusicTabSwitch(this.musicTab);
}

class FavouriteToggle extends SongsEvent {
  final List<String> songIds;
  final bool isFavourite;
  const FavouriteToggle({required this.songIds, required this.isFavourite});
}

class DeleteSong extends SongsEvent {
  final SongInfo songInfo;
  const DeleteSong(this.songInfo);
}

class PlaybackCompleted extends SongsEvent {}

class AddToQueue extends SongsEvent {
  final SongInfo songInfo;
  final bool playNext;
  const AddToQueue(this.songInfo, {this.playNext = false});
}

class RecentSongs extends SongsEvent {}

class BottomBarToggle extends SongsEvent {
  final BottomBarView bottomBarView;
  const BottomBarToggle(this.bottomBarView);
}

class CreatePlaylist extends SongsEvent {
  final String playlistName;
  const CreatePlaylist(this.playlistName);
}

class LoadPlaylist extends SongsEvent {}

class PlaylistViewMode extends SongsEvent {
  final PlaylistViewEnum playlistViewMode;
  const PlaylistViewMode(this.playlistViewMode);
}

class DeletePlaylist extends SongsEvent {
  final String playlistId;
  const DeletePlaylist(this.playlistId);
}

class AddToPlaylist extends SongsEvent {
  final List<String> songIds;
  final List<String> playlistIds;
  final bool isMusicList;
  const AddToPlaylist({
    required this.songIds,
    required this.playlistIds,
    this.isMusicList = false,
  });
}

class GetPlaylistSongs extends SongsEvent {
  final String playlistId;
  const GetPlaylistSongs(this.playlistId);
}

class UpdatedPlaylistSongs extends SongsEvent {
  final String playlistId;
  final List<String> orderedSongIds;
  final List<String> deletedSongIds;

  const UpdatedPlaylistSongs({
    required this.playlistId,
    required this.orderedSongIds,
    required this.deletedSongIds,
  });
}

class PlayPlaylistSongs extends SongsEvent {
  final String playlistId;
  final bool isShuffle;
  final int? songIndex;
  const PlayPlaylistSongs({
    required this.playlistId,
    required this.isShuffle,
    this.songIndex,
  });
}

class PausePlaylistSongs extends SongsEvent {}

class StoreSearchItem extends SongsEvent {
  final SongInfo? song;
  final PlaylistInfo? playlist;

  const StoreSearchItem({this.song, this.playlist});
}

class GetSearchedItems extends SongsEvent {}

class ClearSerachItems extends SongsEvent {
  final String? clearId;
  final bool clearAll;

  const ClearSerachItems({this.clearId, this.clearAll = false});
}
