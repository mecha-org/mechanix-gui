import 'package:equatable/equatable.dart';

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

  @override
  List<Object?> get props => [searchQuery];
}

class PlaySong extends SongsEvent {
  final int index;
  const PlaySong(this.index);

  @override
  List<Object?> get props => [index];
}

class TogglePlayPause extends SongsEvent {}

class PlayNext extends SongsEvent {}

class ShuffleToggle extends SongsEvent {}

// class  ToggleRepeat extends SongsEvent {}
class FavouriteToggle extends SongsEvent {}

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

class ToggleShuffle extends SongsEvent {}

// Enum for repeat modes
enum RepeatMode { none, one, all }
