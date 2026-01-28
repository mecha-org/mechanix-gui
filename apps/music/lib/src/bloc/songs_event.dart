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
  final bool isShuffle;

  const ShuffleToggle(this.isShuffle);

  @override
  List<Object?> get props => [isShuffle];
}

// class  ToggleRepeat extends SongsEvent {}
class ToggleRepeat extends SongsEvent {
  const ToggleRepeat();
}
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

class CreateUpdatePlaylist extends SongsEvent {
  final String playlistName;
  final String? playlistId;
  const CreateUpdatePlaylist({required this.playlistName, this.playlistId});
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

class BackTabEvent extends SongsEvent {}

class SelectedPlaylist extends SongsEvent {
  final String playlistId;
  const SelectedPlaylist(this.playlistId);
}

class SearchPlaylist extends SongsEvent {
  final String searchQuery;
  const SearchPlaylist(this.searchQuery);
}

class SearchedSong extends SongsEvent {
  final String searchQuery;
  const SearchedSong(this.searchQuery);
}

class GetFavouritesSongs extends SongsEvent {}

class PlayFavoriteSongs extends SongsEvent {
  final SongInfo song;
  const PlayFavoriteSongs({required this.song});
}

class AddPlaylistToQueue extends SongsEvent {
  final String playlistId;
  final bool playNext; // true = play next, false = add to end

  const AddPlaylistToQueue({required this.playlistId, this.playNext = false});
}

class ToggleScrolling extends SongsEvent {
  final bool isScrolling;
  const ToggleScrolling(this.isScrolling);
}

class OnSongComplete extends SongsEvent {
  const OnSongComplete();
}

class StartDirectoryWatch extends SongsEvent {
  final String directoryPath;
  const StartDirectoryWatch(this.directoryPath);
}

class StopDirectoryWatch extends SongsEvent {}

class AudioFileCreated extends SongsEvent {
  final String path;
  const AudioFileCreated(this.path);
}

class AudioFileModified extends SongsEvent {
  final String path;
  const AudioFileModified(this.path);
}

class AudioFileDeleted extends SongsEvent {
  final String path;
  const AudioFileDeleted(this.path);
}

class PlaylistShuffle extends SongsEvent {
  final bool isShuffle;
  final String playlistId;
  const PlaylistShuffle({required this.isShuffle, required this.playlistId});
}

class JumpToIndex extends SongsEvent {
  final int index;

  const JumpToIndex(this.index);
}

class MediaKitInitialised extends SongsEvent {}

class AddPlaylistToLiked extends SongsEvent {
  final String playlistId;
  final bool isLiked;
  const AddPlaylistToLiked(this.playlistId, this.isLiked);
}
