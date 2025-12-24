import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/features/home/data/songs_repository.dart';
import 'package:media_kit/media_kit.dart';
import 'songs_state.dart';

class SongsBloc extends Bloc<SongsEvent, SongsState> {
  final Player player = Player();
  final logger = Logger();
  final SongsRepository songsRepository;

  SongsBloc({required this.songsRepository}) : super(const SongsState()) {
    on<ScanSongs>(_onScanSongs);
    on<LoadSongsFromHive>(_onLoadSongsFromHive);
    // on<SearchSong>(_onSearch);
    on<PlaySong>(_onPlaySong);
    on<TogglePlayPause>(_onTogglePlayPause);
    on<PlayNext>(_onPlayNext);
    on<PlayPrevious>(_onPlayPrevious);
    // on<SeekSong>(_onSeekSong);
    // on<UpdateDuration>(_onUpdateDuration);
    // on<UpdatePosition>(_onUpdatePosition);
    // on<ShuffleToggle>(_shuffleToggle);
    on<MusicTabSwitch>(_musicTabSwitch);
    on<FavouriteToggle>(_onToggleFavourite);
    on<DeleteSong>(_onDeleteSong);
    on<PlaybackCompleted>(_onPlaybackComplete);
    on<AddToQueue>(_addToQueue);

    _initializePlayerListeners();
    add(LoadSongsFromHive());
  }

  void _initializePlayerListeners() {
    // Listen to position changes
    // player.stream.position.listen((position) {
    //   if (!isClosed) {
    //     add(UpdatePosition(position));
    //   }
    // });

    // // Listen to duration changes
    // player.stream.duration.listen((duration) {
    //   if (!isClosed) {
    //     add(UpdateDuration(duration));
    //   }
    // });

    // Listen to playback completion
    player.stream.completed.listen((completed) {
      logger.i("Playback completed");

      // emit(state.copyWith(isPlaying: false, currentSong: null));
      if (completed) {
        add(PlayNext());
      }
    });
  }

  void _onPlaybackComplete(PlaybackCompleted event, Emitter<SongsState> emit) {
    emit(state.copyWith(isPlaying: false));
  }

  Future<void> _musicTabSwitch(
    MusicTabSwitch event,
    Emitter<SongsState> emit,
  ) async {
    if (event.musicTab == MusicTabs.home) {
      add(ScanSongs());
    }
    emit(state.copyWith(musicTab: event.musicTab));
  }

  Future<void> _onScanSongs(ScanSongs event, Emitter<SongsState> emit) async {
    emit(state.copyWith(isLoading: true, error: null));
    try {
      final songsScan = await songsRepository.scanAllSongs();

      emit(
        state.copyWith(
          songs: songsScan,
          currentIndex: -1,
          playbackQueue: songsScan,
          playlist: null,
          isLoading: false,
          error: null,
        ),
      );
      logger.i("Songs scanning completed");
    } catch (e) {
      logger.e("Error scanning songs: $e");
      emit(state.copyWith(isLoading: false, error: "Failed to scan songs: $e"));
    }
  }

  Future<void> _onLoadSongsFromHive(
    LoadSongsFromHive event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Loading songs from Hive");
    emit(state.copyWith(isLoading: true, error: null));

    try {
      final songs = await songsRepository.getAllSongs();

      if (songs.isNotEmpty) {
        final playlist = Playlist(songs.map((s) => Media(s.path)).toList());
        await player.open(playlist, play: false);

        emit(
          state.copyWith(
            songs: songs,
            playbackQueue: songs,
            playlist: playlist,
            isLoading: false,
            error: null,
          ),
        );
        logger.i("Loaded ${songs.length} songs from Hive");
      } else {
        emit(
          state.copyWith(
            songs: [],
            playbackQueue: [],
            playlist: null,
            isLoading: false,
            error: null,
          ),
        );
        logger.i("No songs found in Hive");
      }
    } catch (e) {
      logger.e("Error loading songs from Hive: $e");
      emit(state.copyWith(isLoading: false, error: "Failed to load songs: $e"));
    }
  }

  // Future<void> _onSearch(SearchSong event, Emitter<SongsState> emit) async {
  //   final logger = Logger();
  //   logger.i("Search started: ${event.searchQuery}");

  //   if (event.searchQuery.isEmpty) {
  //     emit(state.copyWith(searchedSongs: []));
  //     return;
  //   }

  //   final searchSongs =
  //       state.songs.where((element) {
  //         final title = element.title.toLowerCase();
  //         final artist = element.artist.toLowerCase();
  //         final album = element.album?.toLowerCase() ?? "";
  //         final query = event.searchQuery.toLowerCase();

  //         return title.contains(query) ||
  //             artist.contains(query) ||
  //             album.contains(query);
  //       }).toList();

  //   logger.i("Search completed: ${searchSongs.length} results");
  //   emit(state.copyWith(searchedSongs: searchSongs));
  // }

  Future<void> _onPlaySong(PlaySong event, Emitter<SongsState> emit) async {
    try {
      // Create Media object from the song's file path
      final media = Media(event.song.path);

      // Open and play the media
      await player.open(media);
      // Update state with currently playing song
      emit(
        state.copyWith(currentSong: event.song, isPlaying: true, error: null),
      );

      logger.i("Playing song: ${event.song.title}");
    } catch (e) {
      logger.e("Error playing song ${event.song.id}: $e");
      emit(state.copyWith(error: "Failed to play song: $e", isPlaying: false));
    }
  }

  Future<void> _onTogglePlayPause(
    TogglePlayPause event,
    Emitter<SongsState> emit,
  ) async {
    try {
      // if (state.currentIndex == -1 && state.playbackQueue.isNotEmpty) {
      //   // No song selected, play the first one
      //   add(PlaySong(0));
      //   return;
      // }

      if (state.isPlaying) {
        await player.pause();
        emit(state.copyWith(isPlaying: false));
      } else {
        await player.play();
        emit(state.copyWith(isPlaying: true));
      }
    } catch (e) {
      logger.e("Error toggling play/pause: $e");
      emit(state.copyWith(error: "Failed to toggle playback: $e"));
    }
  }

  Future<void> _onPlayNext(PlayNext event, Emitter<SongsState> emit) async {
    try {
      if (state.currentSong == null) {
        logger.w("No current song to skip from");
        return;
      }

      SongInfo? nextSong;

      // Check if current song is in queue
      final queueIndex = state.playbackQueue.indexWhere(
        (song) => song.id == state.currentSong!.id,
      );

      if (queueIndex != -1) {
        // Currently in queue
        if (queueIndex < state.playbackQueue.length - 1) {
          // Not last in queue, play next from queue
          nextSong = state.playbackQueue[queueIndex + 1];
          logger.i("Next from queue: ${nextSong.title}");
        } else {
          // Last in queue, continue from last queue song's position in main list
          final lastQueueSongInList = state.songs.indexWhere(
            (song) => song.id == state.playbackQueue.last.id,
          );
          if (lastQueueSongInList != -1) {
            final nextIndex = (lastQueueSongInList + 1) % state.songs.length;
            nextSong = state.songs[nextIndex];
            logger.i(
              "Queue ended, continuing from main list: ${nextSong.title}",
            );
          }
        }
      } else {
        // Not in queue, check if next song should be from queue
        final currentInList = state.songs.indexWhere(
          (song) => song.id == state.currentSong!.id,
        );

        if (currentInList != -1) {
          final naturalNext =
              state.songs[(currentInList + 1) % state.songs.length];

          // Check if natural next is the first queue song
          if (state.playbackQueue.isNotEmpty &&
              naturalNext.id == state.playbackQueue.first.id) {
            // Enter the queue
            nextSong = state.playbackQueue.first;
            logger.i("Entering queue: ${nextSong.title}");
          } else {
            // Continue in main list
            nextSong = naturalNext;
            logger.i("Next from main list: ${nextSong.title}");
          }
        }
      }

      if (nextSong == null) {
        logger.w("Could not determine next song");
        return;
      }

      final media = Media(nextSong.path);
      await player.open(media);

      emit(state.copyWith(currentSong: nextSong, isPlaying: true, error: null));
    } catch (e) {
      logger.e("Error playing next song: $e");
      emit(state.copyWith(error: "Failed to play next song: $e"));
    }
  }

  Future<void> _onPlayPrevious(
    PlayPrevious event,
    Emitter<SongsState> emit,
  ) async {
    try {
      if (state.currentSong == null) {
        logger.w("No current song to go back from");
        return;
      }

      SongInfo? prevSong;

      // Check if current song is in queue
      final queueIndex = state.playbackQueue.indexWhere(
        (song) => song.id == state.currentSong!.id,
      );

      if (queueIndex != -1) {
        // Currently in queue
        if (queueIndex > 0) {
          // Not first in queue, play previous from queue
          prevSong = state.playbackQueue[queueIndex - 1];
          logger.i("Previous from queue: ${prevSong.title}");
        } else {
          // First in queue, go to song before first queue song in main list
          final firstQueueSongInList = state.songs.indexWhere(
            (song) => song.id == state.playbackQueue.first.id,
          );
          if (firstQueueSongInList != -1) {
            final prevIndex =
                (firstQueueSongInList - 1 + state.songs.length) %
                state.songs.length;
            prevSong = state.songs[prevIndex];
            logger.i("Exiting queue, going to main list: ${prevSong.title}");
          }
        }
      } else {
        // Not in queue, check if previous song should be from queue
        final currentInList = state.songs.indexWhere(
          (song) => song.id == state.currentSong!.id,
        );

        if (currentInList != -1) {
          final naturalPrev =
              state.songs[(currentInList - 1 + state.songs.length) %
                  state.songs.length];

          // Check if natural previous is the last queue song
          if (state.playbackQueue.isNotEmpty &&
              naturalPrev.id == state.playbackQueue.last.id) {
            // Enter the queue from the end
            prevSong = state.playbackQueue.last;
            logger.i("Entering queue from end: ${prevSong.title}");
          } else {
            // Continue in main list
            prevSong = naturalPrev;
            logger.i("Previous from main list: ${prevSong.title}");
          }
        }
      }

      if (prevSong == null) {
        logger.w("Could not determine previous song");
        return;
      }

      final media = Media(prevSong.path);
      await player.open(media);

      emit(state.copyWith(currentSong: prevSong, isPlaying: true, error: null));
    } catch (e) {
      logger.e("Error playing previous song: $e");
      emit(state.copyWith(error: "Failed to play previous song: $e"));
    }
  }

  // Future<void> _shuffleToggle(
  //   ShuffleToggle event,
  //   Emitter<SongsState> emit,
  // ) async {
  //   player.setShuffle(state.isShuffled);
  //   emit(state.copyWith(isShuffled: !state.isShuffled));
  // }

  // // Future<void> _onToggleRepeat(
  // //   ToggleRepeat event,
  // //   Emitter<SongsState> emit,
  // // ) async {
  // // }

  // Future<void> _onPlayPrevious(
  //   PlayPrevious event,
  //   Emitter<SongsState> emit,
  // ) async {
  //   if (state.playbackQueue.isEmpty) return;

  //   try {
  //     final prevIndex =
  //         state.currentIndex <= 0
  //             ? state.playbackQueue.length - 1
  //             : state.currentIndex - 1;

  //     final prevSong = state.playbackQueue[prevIndex];

  //     // If we're at the first song, we need to jump to the last song
  //     // Otherwise, just go to previous
  //     if (state.currentIndex <= 0) {
  //       // We're at the first song, jump to last song
  //       await player.jump(state.playbackQueue.length - 1);
  //     } else {
  //       // Normal previous song
  //       await player.previous();
  //     }

  //     // Ensure the song actually starts playing
  //     await player.play();

  //     // Initialize listeners if needed
  //     _initializePlayerListeners();

  //     emit(
  //       state.copyWith(
  //         currentIndex: prevIndex,
  //         currentSong: prevSong,
  //         isPlaying: true,
  //         error: null,
  //       ),
  //     );

  //     logger.i("Playing previous song: ${prevSong.title} at index $prevIndex");
  //   } catch (e) {
  //     logger.e("Error playing previous song: $e");
  //     emit(state.copyWith(error: "Failed to play previous song: $e"));
  //   }
  // }

  // Future<void> _onSeekSong(SeekSong event, Emitter<SongsState> emit) async {
  //   try {
  //     await player.seek(event.position);
  //     emit(state.copyWith(position: event.position));
  //   } catch (e) {
  //     logger.e("Error seeking to position ${event.position}: $e");
  //     emit(state.copyWith(error: "Failed to seek: $e"));
  //   }
  // }

  // Future<void> _onUpdatePosition(
  //   UpdatePosition event,
  //   Emitter<SongsState> emit,
  // ) async {
  //   emit(state.copyWith(position: event.position));
  // }

  // Future<void> _onUpdateDuration(
  //   UpdateDuration event,
  //   Emitter<SongsState> emit,
  // ) async {
  //   emit(state.copyWith(duration: event.duration));
  // }

  Future<void> _onToggleFavourite(
    FavouriteToggle event,
    Emitter<SongsState> emit,
  ) async {
    try {
      final newFavouriteValue = !event.songInfo.isFavourite;

      final isUpdated = await songsRepository.toggleFavouriteSong(
        event.songInfo,
        newFavouriteValue,
      );

      if (!isUpdated) return;

      final updatedSongs =
          state.songs.map((song) {
            if (song.id == event.songInfo.id) {
              return song.copyWith(isFavourite: newFavouriteValue);
            }
            return song;
          }).toList();

      emit(state.copyWith(songs: updatedSongs, error: null));

      logger.i(
        "Favourite updated: ${event.songInfo.title} -> $newFavouriteValue",
      );
    } catch (e) {
      logger.e("Error updating favourite: $e");
      emit(state.copyWith(error: "Failed to update favourite"));
    }
  }

  Future<void> _onDeleteSong(DeleteSong event, Emitter<SongsState> emit) async {
    try {
      logger.i("Deleted song: ${event.songInfo.title}");
      await songsRepository.deleteSong(event.songInfo);
      final updatedSongs =
          state.songs.where((s) => s.id != event.songInfo.id).toList();
      emit(state.copyWith(songs: updatedSongs, error: null));
    } catch (e) {
      logger.e("Error deleting song: $e");
      emit(state.copyWith(error: "Failed to delete song: $e"));
    }
  }

  Future<void> _addToQueue(AddToQueue event, Emitter<SongsState> emit) async {
    try {
      logger.i("Adding song to queue: ${event.songInfo.title}");

      emit(
        state.copyWith(
          playbackQueue:
              state.playbackQueue.isEmpty
                    ? [event.songInfo]
                    : state.playbackQueue
                ..add(event.songInfo),
          error: null,
        ),
      );
    } catch (e) {
      logger.e("Adding song to queue: ${event.songInfo.title}");
      emit(state.copyWith(error: "Failed to add song to queue: $e"));
    }
  }

  @override
  Future<void> close() {
    player.dispose();
    return super.close();
  }
}
