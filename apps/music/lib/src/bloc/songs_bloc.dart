import 'dart:async';
import 'dart:io';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/features/home/data/songs_repository.dart';
import 'package:media_kit/media_kit.dart';
import 'songs_state.dart';

class SongsBloc extends Bloc<SongsEvent, SongsState> {
  final Player player = Player();
  StreamSubscription<FileSystemEvent>? _dirSubscription;

  final logger = Logger();
  final SongsRepository songsRepository;

  SongsBloc({required this.songsRepository}) : super(const SongsState()) {
    on<ScanSongs>(_onScanSongs);
    on<LoadSongsFromHive>(_onLoadSongsFromHive);
    on<SearchSong>(_onSearch);
    on<PlaySong>(_onPlaySong);
    on<TogglePlayPause>(_onTogglePlayPause);
    on<PlayNext>(_onPlayNext);
    on<PlayPrevious>(_onPlayPrevious);
    on<MusicTabSwitch>(_musicTabSwitch);
    on<FavouriteToggle>(_onToggleFavourite);
    on<DeleteSong>(_onDeleteSong);
    on<PlaybackCompleted>(_onPlaybackComplete);
    on<AddToQueue>(_addToQueue);
    on<RecentSongs>(recentlyPlayed);
    on<BottomBarToggle>(bottomBarView);
    on<CreateUpdatePlaylist>(createUpdatePlaylist);
    on<LoadPlaylist>(loadPlaylist);
    on<PlaylistViewMode>(playlistViewMode);
    on<DeletePlaylist>(deletePlaylist);
    on<AddToPlaylist>(addToPlaylist);
    on<GetPlaylistSongs>(getPlaylistSongs);
    on<UpdatedPlaylistSongs>(updatedPlaylistSongs);
    on<PlayPlaylistSongs>(playPlaylist);
    on<PausePlaylistSongs>(pausePlaylist);
    on<StoreSearchItem>(storeSearchItem);
    on<GetSearchedItems>(getStoreSearchItems);
    on<ClearSerachItems>(clearSearchItem);
    on<BackTabEvent>(onBackTab);
    on<SelectedPlaylist>(selectedPlaylist);
    _initializePlayerListeners();
    add(LoadSongsFromHive());
    on<SearchPlaylist>(searchPlaylist);
    on<SearchedSong>(searchedSong);
    on<GetFavouritesSongs>(favouritesSongs);
    on<AddPlaylistToQueue>(_addPlaylistToQueue);
    on<PlayFavoriteSongs>(playFavouriteSongs);
    on<ToggleScrolling>(toggleScrolling);
    on<SetRepeatMode>(_onSetRepeatMode);
    on<ToggleRepeat>(_onToggleRepeat);
    on<OnSongComplete>(_onSongComplete);
    on<ShuffleToggle>(_shuffleToggle);
    on<StartDirectoryWatch>(_onStartWatch);
    on<StopDirectoryWatch>(_onStopWatch);
    on<AudioFileCreated>(_onAudioCreated);
    on<AudioFileModified>(_onAudioModified);
    on<AudioFileDeleted>(_onAudioDeleted);
    on<PlaylistShuffle>(shufflePlaylist);
    add(ScanSongs());
    add(StartDirectoryWatch('/home/mecha'));
  }

  void _initializePlayerListeners() {
    // Listen to playback completion
    player.stream.completed.listen((completed) {
      if (completed) {
        logger.i("Playback completed: ${state.currentSong?.title}");
        add(const OnSongComplete());
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
    // Don't add duplicate if already on this tab
    if (state.musicTab == event.musicTab) {
      return;
    }

    final updatedHistory = state.tabHistory.toList();

    // Handle tab-specific initialization
    switch (event.musicTab) {
      case MusicTabs.home:
        {
          add(LoadPlaylist());
          // Clear history when going to home
          return emit(
            state.copyWith(
              musicTab: MusicTabs.home,
              tabHistory: [],
              bottomBarView: BottomBarView.normal,
            ),
          );
        }

      case MusicTabs.playlists:
        {
          add(LoadPlaylist());
          updatedHistory.add(event.musicTab);
          return emit(
            state.copyWith(
              musicTab: MusicTabs.playlists,
              bottomBarView: BottomBarView.normal,
              tabHistory: updatedHistory,
            ),
          );
        }

      case MusicTabs.search:
        {
          add(GetSearchedItems());
          // Add current tab to history before switching to search
          updatedHistory.add(event.musicTab);
          return emit(
            state.copyWith(
              musicTab: MusicTabs.search,
              bottomBarView: BottomBarView.search,
              tabHistory: updatedHistory,
            ),
          );
        }

      case MusicTabs.favorites:
        {
          add(GetFavouritesSongs());
          updatedHistory.add(event.musicTab);
          return emit(
            state.copyWith(
              musicTab: MusicTabs.favorites,
              bottomBarView: BottomBarView.normal,
              tabHistory: updatedHistory,
            ),
          );
        }

      default:
        {
          updatedHistory.add(event.musicTab);

          logger.w(
            "Switching to tab: ${event.musicTab}, History length: ${updatedHistory.length}",
          );
          return emit(
            state.copyWith(
              musicTab: event.musicTab,
              tabHistory: updatedHistory,
              bottomBarView: BottomBarView.normal,
            ),
          );
        }
    }
  }

  Future<void> onBackTab(BackTabEvent event, Emitter<SongsState> emit) async {
    logger.i("Going back to previous tab");

    // If history is empty, go to home
    if (state.tabHistory.isEmpty || state.tabHistory.length == 1) {
      return emit(
        state.copyWith(
          musicTab: MusicTabs.home,
          tabHistory: [],
          bottomBarView: BottomBarView.normal,
        ),
      );
    }

    // Create a mutable copy
    final updatedHistory = List<MusicTabs>.of(state.tabHistory);

    // Get the previous tab (last item in history)
    if (updatedHistory.isNotEmpty) updatedHistory.removeLast();

    final previousTab = updatedHistory.last;
    logger.i(
      "Navigating back to: $previousTab, Remaining history: ${updatedHistory.length}",
    );

    // Handle tab-specific logic when going back
    switch (previousTab) {
      case MusicTabs.home:
        add(LoadPlaylist());
        break;
      case MusicTabs.playlists:
        add(LoadPlaylist());
        break;
      case MusicTabs.search:
        add(GetSearchedItems());
        break;
      default:
        break;
    }
    // Update state with previous tab and updated history
    emit(
      state.copyWith(
        tabHistory: updatedHistory,
        musicTab: previousTab,
        bottomBarView:
            previousTab == MusicTabs.search
                ? BottomBarView.search
                : BottomBarView.normal,
      ),
    );
  }

  Future<void> _onScanSongs(ScanSongs event, Emitter<SongsState> emit) async {
    emit(state.copyWith(isLoading: true, error: null));
    try {
      final songsScan = await songsRepository.scanAllSongs();

      emit(
        state.copyWith(
          songs: songsScan,
          playbackQueue: [],
          isLoading: false,
          error: null,
        ),
      );
      add(LoadPlaylist());
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
        emit(
          state.copyWith(
            songs: songs,
            playbackQueue: [],
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
            isLoading: false,
            error: null,
          ),
        );
        logger.i("No songs found in Hive");
      }
      add(RecentSongs());
      add(LoadPlaylist());
    } catch (e) {
      logger.e("Error loading songs from Hive: $e");
      emit(state.copyWith(isLoading: false, error: "Failed to load songs: $e"));
    }
  }

  Future<void> _onSearch(SearchSong event, Emitter<SongsState> emit) async {
    try {
      if (event.searchQuery.trim().isEmpty || event.searchQuery.length < 3) {
        return emit(
          state.copyWith(
            searchResults: SearchResults(
              query: event.searchQuery.trim(),
              songs: [],
              playlists: [],
            ),
          ),
        );
      }
      logger.i("Searching for ${event.searchQuery}");
      final result = await songsRepository.searchSongs(event.searchQuery);
      emit(state.copyWith(searchResults: result, error: null));
      logger.i(
        "Search results: ${result.playlists.length + result.songs.length}",
      );
    } catch (_) {}
  }

  Future<void> _onPlaySong(PlaySong event, Emitter<SongsState> emit) async {
    try {
      logger.i("Playing song: ${event.song.title}");

      final media = Media(event.song.path);
      await player.open(media, play: true);

      await songsRepository.addToRecentlyPlayed(event.song);
      add(RecentSongs());

      emit(
        state.copyWith(
          playbackQueue: [],
          originalQueue: [],
          currentIndex: null,
          currentSong: event.song,
          isPlaying: true,
          musicMode: MusicMode.normal,
          currentPlaylist: const CurrentPlaylist(),
          error: null,
        ),
      );

      logger.i("Song playing in normal mode");
    } catch (e) {
      logger.e("Error playing song: $e");
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
        // if (state.musicMode == MusicMode.playlist) {
        await player.play();
        emit(state.copyWith(isPlaying: true));
        // }
      }
    } catch (e) {
      logger.e("Error toggling play/pause: $e");
      emit(state.copyWith(error: "Failed to toggle playback: $e"));
    }
  }

  Future<void> _onPlayNext(PlayNext event, Emitter<SongsState> emit) async {
    try {
      if (state.currentSong == null) {
        logger.w("No current song");
        return;
      }

      // Repeat One + Manual Skip → Switch to Repeat All
      if (state.repeatMode == RepeatMode.one) {
        logger.i("Repeat one detected, switching to repeat all");
        emit(state.copyWith(repeatMode: RepeatMode.all));
      }

      // PLAYLIST/FAVORITE MODE
      if (state.musicMode == MusicMode.playlist ||
          state.musicMode == MusicMode.favorite) {
        if (state.playbackQueue.isEmpty) {
          logger.w("Empty queue");
          return;
        }

        final currentIndex = state.currentIndex ?? 0;

        // At end
        if (currentIndex >= state.playbackQueue.length - 1) {
          if (state.repeatMode == RepeatMode.all) {
            // Loop to first
            final nextSong = state.playbackQueue.first;

            final media = Media(nextSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(nextSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentIndex: 0,
                currentSong: nextSong,
                isPlaying: player.state.playing,
                currentPlaylist:
                    state.musicMode == MusicMode.playlist
                        ? state.currentPlaylist.copyWith(currentIndex: 0)
                        : null,
              ),
            );
            return;
          } else {
            // Move to normal mode (both playlist and favorite)
            if (state.musicMode == MusicMode.playlist ||
                state.musicMode == MusicMode.favorite) {
              final lastSong = state.playbackQueue.last;
              final lastSongIndex = state.songs.indexWhere(
                (s) => s.id == lastSong.id,
              );

              if (lastSongIndex != -1 &&
                  lastSongIndex < state.songs.length - 1) {
                final nextSong = state.songs[lastSongIndex + 1];

                final media = Media(nextSong.path);
                await player.open(media, play: true);

                await songsRepository.addToRecentlyPlayed(nextSong);
                add(RecentSongs());

                emit(
                  state.copyWith(
                    playbackQueue: [],
                    originalQueue: [],
                    currentIndex: null,
                    currentSong: nextSong,
                    isPlaying: player.state.playing,
                    musicMode: MusicMode.normal,
                    currentPlaylist: const CurrentPlaylist(),
                  ),
                );
                return;
              }
            }

            // Stop
            if (state.isPlaying) {
              await player.pause();
              emit(state.copyWith(isPlaying: false));
            }
            return;
          }
        }

        // Play next
        final nextIndex = currentIndex + 1;
        final nextSong = state.playbackQueue[nextIndex];

        final media = Media(nextSong.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(nextSong);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentIndex: nextIndex,
            currentSong: nextSong,
            isPlaying: player.state.playing,
            currentPlaylist:
                state.musicMode == MusicMode.playlist
                    ? state.currentPlaylist.copyWith(currentIndex: nextIndex)
                    : null,
          ),
        );

        logger.i("Playing next");
        return;
      }

      // NORMAL MODE
      if (state.playbackQueue.isNotEmpty && state.currentIndex != null) {
        final currentIndex = state.currentIndex!;

        if (currentIndex < state.playbackQueue.length - 1) {
          // Next from queue
          final nextIndex = currentIndex + 1;
          final nextSong = state.playbackQueue[nextIndex];

          final media = Media(nextSong.path);
          await player.open(media, play: true);

          await songsRepository.addToRecentlyPlayed(nextSong);
          add(RecentSongs());

          emit(
            state.copyWith(
              currentIndex: nextIndex,
              currentSong: nextSong,
              isPlaying: player.state.playing,
            ),
          );

          logger.i("Next from queue");
          return;
        } else {
          // End of queue
          if (state.repeatMode == RepeatMode.all) {
            // Loop to FIRST SONG (the song that was playing when queue was added)
            final nextSong = state.playbackQueue.first;

            final media = Media(nextSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(nextSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentIndex: 0,
                currentSong: nextSong,
                isPlaying: player.state.playing,
              ),
            );

            logger.i("Repeat all: Looping to first song in queue");
            return;
          } else {
            // Exit queue
            final lastQueueSong = state.playbackQueue.last;
            final lastSongIndex = state.songs.indexWhere(
              (s) => s.id == lastQueueSong.id,
            );

            if (lastSongIndex != -1) {
              final nextIndex = (lastSongIndex + 1) % state.songs.length;
              final nextSong = state.songs[nextIndex];

              final media = Media(nextSong.path);
              await player.open(media, play: true);

              await songsRepository.addToRecentlyPlayed(nextSong);
              add(RecentSongs());

              emit(
                state.copyWith(
                  playbackQueue: [],
                  originalQueue: [],
                  currentIndex: null,
                  currentSong: nextSong,
                  isPlaying: player.state.playing,
                ),
              );

              logger.i("Exited queue");
              return;
            }
          }
        }
      } else {
        // No queue - main list
        final currentInList = state.songs.indexWhere(
          (s) => s.id == state.currentSong!.id,
        );

        if (currentInList != -1) {
          final nextIndex = (currentInList + 1) % state.songs.length;
          final nextSong = state.songs[nextIndex];

          final media = Media(nextSong.path);
          await player.open(media, play: true);

          await songsRepository.addToRecentlyPlayed(nextSong);
          add(RecentSongs());

          emit(
            state.copyWith(
              currentSong: nextSong,
              isPlaying: player.state.playing,
            ),
          );

          logger.i("Next from main list");
          return;
        }
      }

      logger.w("Could not determine next song");
    } catch (e) {
      logger.e("Error playing next: $e");
      emit(state.copyWith(error: "Failed to play next: $e"));
    }
  }

  Future<void> _onPlayPrevious(
    PlayPrevious event,
    Emitter<SongsState> emit,
  ) async {
    try {
      if (state.currentSong == null) {
        logger.w("No current song");
        return;
      }
      if (player.state.position.inSeconds > 5) {
        logger.w("Cannot skip, song is less than 5 seconds");
        await player.seek(Duration(seconds: 0));
        return;
      }

      // Repeat One + Manual Skip → Switch to Repeat All
      if (state.repeatMode == RepeatMode.one) {
        logger.i("Repeat one detected, switching to repeat all");
        emit(state.copyWith(repeatMode: RepeatMode.all));
      }

      // PLAYLIST/FAVORITE MODE
      if (state.musicMode == MusicMode.playlist ||
          state.musicMode == MusicMode.favorite) {
        if (state.playbackQueue.isEmpty) {
          logger.w("Empty queue");
          return;
        }

        final currentIndex = state.currentIndex ?? 0;

        // At start
        if (currentIndex == 0) {
          if (state.repeatMode == RepeatMode.all) {
            // Loop to last
            final prevSong = state.playbackQueue.last;

            final media = Media(prevSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(prevSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentIndex: state.playbackQueue.length - 1,
                currentSong: prevSong,
                isPlaying: player.state.playing,
                currentPlaylist:
                    state.musicMode == MusicMode.playlist
                        ? state.currentPlaylist.copyWith(
                          currentIndex: state.playbackQueue.length - 1,
                        )
                        : null,
              ),
            );
          } else {
            logger.i("At start");
          }
          return;
        }

        // Play previous
        final prevIndex = currentIndex - 1;
        final prevSong = state.playbackQueue[prevIndex];

        final media = Media(prevSong.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(prevSong);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentIndex: prevIndex,
            currentSong: prevSong,
            isPlaying: player.state.playing,
            currentPlaylist:
                state.musicMode == MusicMode.playlist
                    ? state.currentPlaylist.copyWith(currentIndex: prevIndex)
                    : null,
          ),
        );

        logger.i("Playing previous");
        return;
      }

      // NORMAL MODE
      if (state.playbackQueue.isNotEmpty && state.currentIndex != null) {
        final currentIndex = state.currentIndex!;

        if (currentIndex > 0) {
          // Previous from queue
          final prevIndex = currentIndex - 1;
          final prevSong = state.playbackQueue[prevIndex];

          final media = Media(prevSong.path);
          await player.open(media, play: true);

          await songsRepository.addToRecentlyPlayed(prevSong);
          add(RecentSongs());

          emit(
            state.copyWith(
              currentIndex: prevIndex,
              currentSong: prevSong,
              isPlaying: player.state.playing,
            ),
          );

          logger.i("Previous from queue");
          return;
        } else {
          // At start of queue (index 0)
          if (state.repeatMode == RepeatMode.all) {
            // Repeat all: Loop to last song in queue
            final prevSong = state.playbackQueue.last;

            final media = Media(prevSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(prevSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentIndex: state.playbackQueue.length - 1,
                currentSong: prevSong,
                isPlaying: player.state.playing,
              ),
            );

            logger.i("Repeat all: Looping to last in queue");
            return;
          } else {
            // No repeat: Replay first song (don't exit queue)
            final firstSong = state.playbackQueue.first;

            final media = Media(firstSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(firstSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentIndex: 0,
                currentSong: firstSong,
                isPlaying: player.state.playing,
              ),
            );

            logger.i("At first song in queue, replaying");
            return;
          }
        }
      } else {
        // No queue - main list
        final currentInList = state.songs.indexWhere(
          (s) => s.id == state.currentSong!.id,
        );

        if (currentInList > 0) {
          final prevSong = state.songs[currentInList - 1];

          final media = Media(prevSong.path);
          await player.open(media, play: true);

          await songsRepository.addToRecentlyPlayed(prevSong);
          add(RecentSongs());

          emit(
            state.copyWith(
              currentSong: prevSong,
              isPlaying: player.state.playing,
            ),
          );

          logger.i("Previous from main list");
          return;
        } else {
          logger.i("At start of main list");
          return;
        }
      }
    } catch (e) {
      logger.e("Error playing previous: $e");
      emit(state.copyWith(error: "Failed to play previous: $e"));
    }
  }

  Future<void> _shuffleToggle(
    ShuffleToggle event,
    Emitter<SongsState> emit,
  ) async {
    try {
      final bool newShuffleState = event.isShuffle;
      logger.i("Toggling shuffle: $newShuffleState");

      if (state.currentSong == null) {
        emit(state.copyWith(isShuffled: newShuffleState));
        return;
      }

      // NORMAL MODE - Don't shuffle if queue exists (add to queue/play next)
      if (state.musicMode == MusicMode.normal) {
        if (state.playbackQueue.isNotEmpty) {
          // Queue exists - shuffle should not affect it
          logger.i("Queue exists in normal mode, shuffle ignored");
          emit(state.copyWith(isShuffled: newShuffleState));
          return;
        }

        // No queue - shuffle all songs
        if (newShuffleState) {
          final remaining =
              List<SongInfo>.from(state.songs)
                ..removeWhere((s) => s.id == state.currentSong!.id)
                ..shuffle();

          final shuffledQueue = [state.currentSong!, ...remaining];

          emit(
            state.copyWith(
              playbackQueue: shuffledQueue,
              originalQueue: [],
              currentIndex: 0,
              isShuffled: true,
            ),
          );

          logger.i("All songs shuffled");
        } else {
          // Shuffle off - clear queue
          emit(
            state.copyWith(
              playbackQueue: [],
              currentIndex: null,
              isShuffled: false,
            ),
          );

          logger.i("Shuffle off");
        }
        return;
      }

      // PLAYLIST/FAVORITE MODE - Same as before
      final currentIndex = state.currentIndex ?? 0;
      final currentSong =
          state.playbackQueue.isNotEmpty &&
                  currentIndex < state.playbackQueue.length
              ? state.playbackQueue[currentIndex]
              : state.currentSong;

      if (newShuffleState) {
        final remaining =
            List<SongInfo>.from(state.playbackQueue)
              ..removeWhere((s) => s.id == currentSong?.id)
              ..shuffle();

        final shuffledQueue = [currentSong!, ...remaining];

        emit(
          state.copyWith(
            playbackQueue: shuffledQueue,
            originalQueue: List.from(state.playbackQueue),
            currentIndex: 0,
            isShuffled: true,
            currentPlaylist:
                state.musicMode == MusicMode.playlist
                    ? state.currentPlaylist.copyWith(
                      currentIndex: 0,
                      isShuffle: true,
                    )
                    : null,
          ),
        );

        logger.i("Shuffled");
      } else {
        if (state.originalQueue.isNotEmpty) {
          final originalIndex = state.originalQueue.indexWhere(
            (s) => s.id == currentSong?.id,
          );

          emit(
            state.copyWith(
              playbackQueue: List.from(state.originalQueue),
              originalQueue: [],
              currentIndex: originalIndex >= 0 ? originalIndex : 0,
              isShuffled: false,
              currentPlaylist:
                  state.musicMode == MusicMode.playlist
                      ? state.currentPlaylist.copyWith(
                        currentIndex: originalIndex >= 0 ? originalIndex : 0,
                        isShuffle: false,
                      )
                      : null,
            ),
          );

          logger.i("Restored");
        }
      }
    } catch (e) {
      logger.e("Error toggling shuffle: $e");
      emit(state.copyWith(error: "Failed to toggle shuffle: $e"));
    }
  }
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
      logger.i("Updating Favourites ${event.isFavourite}");
      final isUpdated = await songsRepository.toggleFavouriteSong(
        event.songIds,
        event.isFavourite,
      );

      if (!isUpdated) return;

      if (state.playlistSongs.isNotEmpty) {
        final updatedSongs =
            state.playlistSongs.map((song) {
              return song.copyWith(isFavourite: event.isFavourite);
            }).toList();

        emit(state.copyWith(playlistSongs: updatedSongs, error: null));
      }
      emit(
        state.copyWith(
          favouriteSongs:
              state.favouriteSongs.isNotEmpty
                  ? state.favouriteSongs
                      //  remove if unfavourited
                      .where(
                        (song) =>
                            !(event.songIds.contains(song.id) &&
                                event.isFavourite == false),
                      )
                      .map(
                        (song) =>
                            event.songIds.contains(song.id)
                                ? song.copyWith(isFavourite: event.isFavourite)
                                : song,
                      )
                      .toList()
                  : state.favouriteSongs,

          songs:
              state.songs
                  .map(
                    (song) =>
                        event.songIds.contains(song.id)
                            ? song = song.copyWith(
                              isFavourite: event.isFavourite,
                            )
                            : song,
                  )
                  .toList(),
          currentSong:
              state.currentSong != null
                  ? event.songIds.contains(state.currentSong?.id)
                      ? state.currentSong?.copyWith(
                        isFavourite: event.isFavourite,
                      )
                      : state.currentSong
                  : null,
        ),
      );
      logger.i("Favourite updated: ${event.isFavourite}");
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

      add(RecentSongs());
      emit(state.copyWith(songs: updatedSongs, error: null));
    } catch (e) {
      logger.e("Error deleting song: $e");
      emit(state.copyWith(error: "Failed to delete song: $e"));
    }
  }

  Future<void> _addToQueue(AddToQueue event, Emitter<SongsState> emit) async {
    try {
      logger.i(
        "Adding to queue: ${event.songInfo.title}, playNext: ${event.playNext}",
      );

      if (state.currentSong == null) {
        final media = Media(event.songInfo.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(event.songInfo);
        add(RecentSongs());

        emit(
          state.copyWith(
            playbackQueue: [event.songInfo],
            currentIndex: 0,
            currentSong: event.songInfo,
            isPlaying: true,
            musicMode: MusicMode.normal,
            currentPlaylist: const CurrentPlaylist(),
            error: null,
          ),
        );
        return;
      }

      // Convert from Playlist/Favorite mode to Normal mode
      if (state.musicMode == MusicMode.playlist ||
          state.musicMode == MusicMode.favorite) {
        logger.i("Converting ${state.musicMode} to normal mode with queue");

        final currentIndex = state.currentIndex ?? 0;
        final remainingSongs =
            state.playbackQueue.isNotEmpty &&
                    currentIndex < state.playbackQueue.length
                ? state.playbackQueue.sublist(currentIndex)
                : [state.currentSong!];

        List<SongInfo> updatedQueue = List.from(remainingSongs);

        if (event.playNext) {
          updatedQueue.insert(1, event.songInfo);
        } else {
          updatedQueue.add(event.songInfo);
        }

        emit(
          state.copyWith(
            playbackQueue: updatedQueue,
            originalQueue: [],
            currentIndex: 0,
            musicMode: MusicMode.normal,
            currentPlaylist: const CurrentPlaylist(),
            isShuffled: false,
            error: null,
          ),
        );

        logger.i("Converted to normal mode with queue");
        return;
      }

      // Normal mode - add to existing queue
      List<SongInfo> updatedQueue = List.from(state.playbackQueue);
      int newCurrentIndex = state.currentIndex ?? 0;

      if (updatedQueue.isEmpty) {
        updatedQueue.add(state.currentSong!);
        newCurrentIndex = 0;
      }

      if (event.playNext) {
        updatedQueue.insert(newCurrentIndex + 1, event.songInfo);
      } else {
        updatedQueue.add(event.songInfo);
      }

      emit(
        state.copyWith(
          playbackQueue: updatedQueue,
          currentIndex: newCurrentIndex,
          error: null,
        ),
      );

      logger.i("Queue updated: ${updatedQueue.length} songs");
    } catch (e) {
      logger.e("Error adding to queue: $e");
      emit(state.copyWith(error: "Failed to add to queue: $e"));
    }
  }

  Future<void> recentlyPlayed(
    RecentSongs event,
    Emitter<SongsState> emit,
  ) async {
    final recentlyPlayed = await songsRepository.getRecentlyPlayed();
    emit(state.copyWith(recentlyPlayedSongs: recentlyPlayed));
  }

  Future<void> bottomBarView(
    BottomBarToggle event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Toggling bottom bar view to ${event.bottomBarView}");
    emit(state.copyWith(bottomBarView: event.bottomBarView));
  }

  Future<void> createUpdatePlaylist(
    CreateUpdatePlaylist event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Creating playlist: ${event.playlistName}");
    await songsRepository.createUpdatePlaylist(
      event.playlistName,
      event.playlistId,
    );

    emit(
      state.copyWith(
        bottomBarView: BottomBarView.normal,
        selectedPlaylist:
            state.selectedPlaylist != null &&
                    event.playlistId == state.selectedPlaylist?.id
                ? state.selectedPlaylist?.copyWith(
                  name: event.playlistName,
                  coverImagePath: state.selectedPlaylist?.coverImagePath,
                )
                : state.selectedPlaylist,
      ),
    );
    logger.i("Playlist created successfully");
    add(LoadPlaylist());
  }

  Future<void> loadPlaylist(
    LoadPlaylist event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Loading playlist");

    final playlists = await songsRepository.getPlaylist();
    final selectedPlaylist = state.selectedPlaylist;

    // Try to find updated version of selected playlist (if any)
    final PlaylistInfo? updatedSelectedPlaylist =
        selectedPlaylist == null
            ? null
            : playlists.firstWhere(
              (p) => p.id == selectedPlaylist.id,
              orElse: () => selectedPlaylist,
            );

    emit(
      state.copyWith(
        playlists: playlists,
        selectedPlaylist: updatedSelectedPlaylist,
        bottomBarView: BottomBarView.normal,
      ),
    );

    logger.i("Playlist loaded successfully");
  }

  Future<void> playlistViewMode(
    PlaylistViewMode event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Changing playlist view mode to ${event.playlistViewMode}");
    emit(state.copyWith(playlistView: event.playlistViewMode));
  }

  Future<void> deletePlaylist(
    DeletePlaylist event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Deleting playlist: ${event.playlistId}");
    final isDeleted = await songsRepository.deletePlaylist(event.playlistId);
    if (isDeleted) {
      emit(
        state.copyWith(
          playlists:
              state.playlists.where((p) => p.id != event.playlistId).toList(),
        ),
      );
      if (state.musicTab == MusicTabs.playlistInfo) {
        add(BackTabEvent());
      }
    }
    logger.i("Playlist deleted successfully");
  }

  Future<void> addToPlaylist(
    AddToPlaylist event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Adding songs to playlist: ${event.songIds}");

    final updatedSongs = await songsRepository.addToPlaylist(
      event.playlistIds,
      event.songIds,
    );

    if (updatedSongs.isNotEmpty) {
      // Create a map of updated songs for O(1) lookup
      final updatedSongsMap = {for (var song in updatedSongs) song.id: song};

      emit(
        state.copyWith(
          selectedPlaylist:
              state.selectedPlaylist != null &&
                      event.playlistIds.contains(state.selectedPlaylist!.id)
                  ? state.selectedPlaylist!.copyWith(
                    songIds: [
                      ...state.selectedPlaylist!.songIds,
                      ...event.songIds,
                    ],
                  )
                  : state.selectedPlaylist,

          playlistSongs:
              event.isMusicList
                  ? [...state.playlistSongs, ...updatedSongs]
                  : state.playlistSongs,

          songs:
              state.songs
                  .map((song) => updatedSongsMap[song.id] ?? song)
                  .toList(),
        ),
      );
      logger.i("${updatedSongs.length} song(s) added to playlist successfully");
      add(LoadPlaylist());
    } else {
      logger.w("No songs were added to playlist");
    }
  }

  Future<void> getPlaylistSongs(
    GetPlaylistSongs event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Getting playlist songs: ${event.playlistId}");
    final playlistSongs = await songsRepository.getPlaylistSongs(
      event.playlistId,
    );
    emit(state.copyWith(playlistSongs: playlistSongs));
    logger.i("Playlist songs loaded successfully");
  }

  Future<void> updatedPlaylistSongs(
    UpdatedPlaylistSongs event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Updating playlist songs: ${event.playlistId}");

    final isUpdated = await songsRepository.updatePlaylistSongs(
      event.playlistId,
      event.orderedSongIds,
      event.deletedSongIds,
    );

    if (isUpdated) {
      add(GetPlaylistSongs(event.playlistId));
      add(LoadPlaylist());
      add(SelectedPlaylist(event.playlistId));
    }
    // emit(state.copyWith(playlistSongs: playlistSongs));
    logger.i("Playlist songs updated successfully");
  }

  Future<void> shufflePlaylist(
    PlaylistShuffle event,
    Emitter<SongsState> emit,
  ) async {
    try {
      final isUpdated = await songsRepository.shuffleToggle(
        event.playlistId,
        event.isShuffle,
      );

      if (isUpdated) {
        add(LoadPlaylist());

        final isCurrentplaylist =
            event.playlistId == state.currentPlaylist.playlistId;
        final isSelectedPlaylist =
            state.selectedPlaylist?.id == event.playlistId;
        add(ShuffleToggle(event.isShuffle));
        emit(
          state.copyWith(
            currentPlaylist:
                isCurrentplaylist
                    ? state.currentPlaylist.copyWith(isShuffle: event.isShuffle)
                    : state.currentPlaylist,
            selectedPlaylist:
                isSelectedPlaylist
                    ? state.selectedPlaylist?.copyWith(
                      isShuffle: event.isShuffle,
                    )
                    : state.selectedPlaylist,
          ),
        );
      }
    } catch (_) {}
  }

  Future<void> playPlaylist(
    PlayPlaylistSongs event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i("Playing playlist: ${event.playlistId}");

      if (state.musicMode == MusicMode.playlist &&
          state.currentPlaylist.playlistId == event.playlistId &&
          event.songIndex == null) {
        return add(TogglePlayPause());
      }

      final playlist = await songsRepository.getPlaylistSongs(event.playlistId);

      if (playlist.isEmpty) {
        logger.w("Playlist is empty");
        emit(state.copyWith(error: "Playlist is empty"));
        return;
      }

      final startIndex = event.songIndex ?? 0;

      if (startIndex < 0 || startIndex >= playlist.length) {
        logger.w("Invalid playlist index");
        return;
      }

      List<SongInfo> queue;
      int queueIndex;
      List<SongInfo> originalQueue = [];

      if (event.isShuffle) {
        final currentSong = playlist[startIndex];
        final remaining =
            List<SongInfo>.from(playlist)
              ..removeWhere((s) => s.id == currentSong.id)
              ..shuffle();

        queue = [currentSong, ...remaining];
        queueIndex = 0;
        originalQueue = List.from(playlist);

        logger.i("Playlist shuffled");
      } else {
        queue = List.from(playlist);
        queueIndex = startIndex;
      }

      final songToPlay = queue[queueIndex];
      final media = Media(songToPlay.path);
      await player.open(media, play: true);

      await songsRepository.addToRecentlyPlayed(songToPlay);
      add(RecentSongs());

      emit(
        state.copyWith(
          playlistSongs: playlist,
          playbackQueue: queue,
          originalQueue: originalQueue.isEmpty ? [] : originalQueue,
          currentIndex: queueIndex,
          currentSong: songToPlay,
          isPlaying: true,
          musicMode: MusicMode.playlist,
          isShuffled: event.isShuffle,
          currentPlaylist: CurrentPlaylist(
            playlistId: event.playlistId,
            currentIndex: queueIndex,
            isShuffle: event.isShuffle,
          ),
          error: null,
        ),
      );

      logger.i("Playlist loaded successfully");
    } catch (e) {
      logger.e("Error playing playlist: $e");
      emit(state.copyWith(error: "Failed to play playlist: $e"));
    }
  }

  Future<void> pausePlaylist(
    PausePlaylistSongs event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Pausing playlist");
    await player.pause();
    emit(state.copyWith(isPlaying: false));
    logger.i("Playlist paused successfully");
  }

  Future<void> storeSearchItem(
    StoreSearchItem event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Storing Search event");
    try {
      final isUpdated = await songsRepository.storeSearchItem(
        playlistInfo: event.playlist,
        songInfo: event.song,
      );
      if (isUpdated) {
        add(GetSearchedItems());
      }
    } catch (_) {}
  }

  Future<void> getStoreSearchItems(
    GetSearchedItems event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Getting search history");
    final storeSearchItems = await songsRepository.getStoredSearchItems();
    emit(state.copyWith(searchItems: storeSearchItems));
    logger.i("Search history loaded successfully");
  }

  Future<void> clearSearchItem(
    ClearSerachItems event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Clearing search history");
    try {
      final isUpdated = await songsRepository.clearSearchItems(
        searchId: event.clearId,
        clearAll: event.clearAll,
      );
      if (isUpdated) {
        add(GetSearchedItems());
      }
    } catch (_) {}
  }

  Future<void> selectedPlaylist(
    SelectedPlaylist event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i("Selected playlist: ${event.playlistId}");
      final playlist = await songsRepository.getSelectedPlaylist(
        playlistId: event.playlistId,
      );

      add(MusicTabSwitch(MusicTabs.playlistInfo));
      emit(state.copyWith(selectedPlaylist: playlist));
      if (playlist != null) {
        add(GetPlaylistSongs(playlist.id));
      }
    } catch (e) {
      logger.e("Error getting playlist: $e");
    }
  }

  Future<void> searchPlaylist(
    SearchPlaylist event,
    Emitter<SongsState> emit,
  ) async {
    try {
      if (event.searchQuery.trim().isNotEmpty) {
        logger.i("Searching playlist: ${event.searchQuery}");
        final playlist = await songsRepository.searchedPlaylist(
          query: event.searchQuery,
        );
        emit(state.copyWith(searchedPlaylist: playlist));
      } else {
        emit(state.copyWith(searchedPlaylist: []));
      }
    } catch (e) {
      logger.e("Error searching playlist: $e");
    }
  }

  Future<void> searchedSong(
    SearchedSong event,
    Emitter<SongsState> emit,
  ) async {
    try {
      if (event.searchQuery.trim().isNotEmpty) {
        logger.i("Searching song: ${event.searchQuery}");
        final songs = await songsRepository.searchedSong(
          query: event.searchQuery,
        );
        emit(state.copyWith(searchedSongs: songs));
      } else {
        emit(state.copyWith(searchedSongs: []));
      }
    } catch (e) {
      logger.e("Error searching song: $e");
    }
  }

  Future<void> favouritesSongs(
    GetFavouritesSongs event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i("Getting favourite songs");
      final favouriteSongs = await songsRepository.getFavouriteSongs();
      emit(state.copyWith(favouriteSongs: favouriteSongs));
      logger.i("Favourite songs loaded successfully ${favouriteSongs.length}");
    } catch (e) {
      logger.e("Error getting favourite songs: $e");
    }
  }

  Future<void> playFavouriteSongs(
    PlayFavoriteSongs event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i("Playing favourite song: ${event.song.title}");

      final startIndex = state.favouriteSongs.indexWhere(
        (s) => s.id == event.song.id,
      );

      if (startIndex == -1) {
        logger.w("Song not found in favorites");
        return;
      }

      List<SongInfo> queue;
      int queueIndex;
      List<SongInfo> originalQueue = [];

      if (state.isShuffled) {
        final remaining =
            List<SongInfo>.from(state.favouriteSongs)
              ..removeWhere((s) => s.id == event.song.id)
              ..shuffle();

        queue = [event.song, ...remaining];
        queueIndex = 0;
        originalQueue = List.from(state.favouriteSongs);

        logger.i("Favorites shuffled");
      } else {
        queue = List.from(state.favouriteSongs);
        queueIndex = startIndex;
      }

      final media = Media(event.song.path);
      await player.open(media, play: true);

      await songsRepository.addToRecentlyPlayed(event.song);
      add(RecentSongs());

      emit(
        state.copyWith(
          playbackQueue: queue,
          originalQueue: originalQueue.isEmpty ? [] : originalQueue,
          currentIndex: queueIndex,
          currentSong: event.song,
          isPlaying: true,
          musicMode: MusicMode.favorite,
          currentPlaylist: const CurrentPlaylist(),
          error: null,
        ),
      );

      logger.i("Favorite playing successfully");
    } catch (e) {
      logger.e("Error playing favourite: $e");
      emit(state.copyWith(error: "Failed to play favourite: $e"));
    }
  }

  Future<void> _addPlaylistToQueue(
    AddPlaylistToQueue event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i("Adding playlist to queue: ${event.playlistId}");

      final playlistSongs = await songsRepository.getPlaylistSongs(
        event.playlistId,
      );

      if (playlistSongs.isEmpty) {
        logger.w("Playlist is empty");
        return;
      }

      if (state.currentSong == null) {
        final media = Media(playlistSongs.first.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(playlistSongs.first);
        add(RecentSongs());

        emit(
          state.copyWith(
            playbackQueue: playlistSongs,
            currentIndex: 0,
            currentSong: playlistSongs.first,
            isPlaying: true,
            musicMode: MusicMode.normal,
            error: null,
          ),
        );
        return;
      }

      // Convert to normal mode if needed
      if (state.musicMode == MusicMode.playlist ||
          state.musicMode == MusicMode.favorite) {
        final currentIndex = state.currentIndex ?? 0;
        final remainingSongs =
            state.playbackQueue.isNotEmpty &&
                    currentIndex < state.playbackQueue.length
                ? state.playbackQueue.sublist(currentIndex)
                : [state.currentSong!];

        List<SongInfo> updatedQueue = List.from(remainingSongs);

        if (event.playNext) {
          updatedQueue.insertAll(1, playlistSongs);
        } else {
          updatedQueue.addAll(playlistSongs);
        }

        emit(
          state.copyWith(
            playbackQueue: updatedQueue,
            originalQueue: [],
            currentIndex: 0,
            musicMode: MusicMode.normal,
            currentPlaylist: const CurrentPlaylist(),
            isShuffled: false,
            error: null,
          ),
        );
      } else {
        // Normal mode
        List<SongInfo> updatedQueue = List.from(state.playbackQueue);
        int newCurrentIndex = state.currentIndex ?? 0;

        if (updatedQueue.isEmpty) {
          updatedQueue.add(state.currentSong!);
          newCurrentIndex = 0;
        }

        if (event.playNext) {
          updatedQueue.insertAll(newCurrentIndex + 1, playlistSongs);
        } else {
          updatedQueue.addAll(playlistSongs);
        }

        emit(
          state.copyWith(
            playbackQueue: updatedQueue,
            currentIndex: newCurrentIndex,
            error: null,
          ),
        );
      }

      logger.i("Playlist added to queue");
    } catch (e) {
      logger.e("Error adding playlist to queue: $e");
      emit(state.copyWith(error: "Failed to add playlist to queue: $e"));
    }
  }

  Future<void> _onToggleRepeat(
    ToggleRepeat event,
    Emitter<SongsState> emit,
  ) async {
    try {
      final RepeatMode nextMode;

      switch (state.repeatMode) {
        case RepeatMode.none:
          nextMode = RepeatMode.one;
          break;
        case RepeatMode.one:
          nextMode = RepeatMode.all;
          break;
        case RepeatMode.all:
          nextMode = RepeatMode.none;
          break;
      }

      emit(state.copyWith(repeatMode: nextMode));
      logger.i("Repeat mode: $nextMode");
    } catch (e) {
      logger.e("Error toggling repeat: $e");
    }
  }

  Future<void> _onSetRepeatMode(
    SetRepeatMode event,
    Emitter<SongsState> emit,
  ) async {
    try {
      emit(state.copyWith(repeatMode: event.mode));
      logger.i("Repeat mode set to: ${event.mode}");
    } catch (e) {
      logger.e("Error setting repeat mode: $e");
    }
  }

  Future<void> _onSongComplete(
    OnSongComplete event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i("Song completed: ${state.currentSong?.title}");

      // Repeat One - replay
      if (state.repeatMode == RepeatMode.one && state.currentSong != null) {
        logger.i("Repeat one, replaying");
        final media = Media(state.currentSong!.path);
        await player.open(media, play: true);
        emit(state.copyWith(isPlaying: true));
        return;
      }

      // Play next
      add(PlayNext());
    } catch (e) {
      logger.e("Error handling completion: $e");
    }
  }

  Future<void> toggleScrolling(
    ToggleScrolling event,
    Emitter<SongsState> emit,
  ) async {
    try {
      emit(state.copyWith(isScrolling: event.isScrolling));
    } catch (_) {}
  }

  bool isAudioFile(String path) {
    final lower = path.toLowerCase();
    return audioExt.any((ext) => lower.endsWith(ext));
  }

  Future<void> _onStartWatch(
    StartDirectoryWatch event,
    Emitter<SongsState> emit,
  ) async {
    await _dirSubscription?.cancel();

    final directory = Directory(event.directoryPath);
    if (!directory.existsSync()) return;

    _dirSubscription = directory.watch(recursive: true).listen((fsEvent) {
      final path = fsEvent.path;

      if (!isAudioFile(path)) return;

      if (fsEvent is FileSystemCreateEvent) {
        add(AudioFileCreated(path));
      } else if (fsEvent is FileSystemModifyEvent) {
        add(AudioFileModified(path));
      } else if (fsEvent is FileSystemDeleteEvent) {
        add(AudioFileDeleted(path));
      }
    });
  }

  Future<void> _onStopWatch(
    StopDirectoryWatch event,
    Emitter<SongsState> emit,
  ) async {
    await _dirSubscription?.cancel();
    _dirSubscription = null;
  }

  // ---------------- FILE EVENTS ----------------

  Future<void> _onAudioCreated(
    AudioFileCreated event,
    Emitter<SongsState> emit,
  ) async {
    // 👉 Scan metadata + add song
    print("audio file create path: ${event.path}");
    await songsRepository.addSongFromPath(event.path);
  }

  Future<void> _onAudioModified(
    AudioFileModified event,
    Emitter<SongsState> emit,
  ) async {
    // 👉 Re-read metadata / duration / artwork
    await songsRepository.updateSongFromPath(event.path);
  }

  Future<void> _onAudioDeleted(
    AudioFileDeleted event,
    Emitter<SongsState> emit,
  ) async {
    // 👉 Remove song from DB / Hive / playlists
    await songsRepository.removeSongByPath(event.path);
  }

  @override
  Future<void> close() async {
    await _dirSubscription?.cancel();
    player.dispose();
    return super.close();
  }
}
