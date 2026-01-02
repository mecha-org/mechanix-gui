import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
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

    add(ScanSongs());
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
      // Create Media object from the song's file path
      final media = Media(event.song.path);

      // Open and play the media
      await player.open(media);

      // Update state with currently playing song
      // Clear queue and reset currentIndex when playing a new song directly
      emit(
        state.copyWith(
          currentSong: event.song,
          isPlaying: true,
          error: null,
          playbackQueue: [],
          currentIndex: null, // Reset currentIndex
          currentPlaylist: const CurrentPlaylist(),
          playlistSongs: [], // Clear playlist songs
          musicMode: MusicMode.normal,
        ),
      );

      await songsRepository.addToRecentlyPlayed(event.song);
      add(RecentSongs());
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
        logger.w("No current song to skip from");
        return;
      }

      // ============================================================================
      // REPEAT ONE + Manual Skip - Switch to Repeat All
      // ============================================================================
      if (state.repeatMode == RepeatMode.one) {
        logger.i("Repeat one detected on manual skip, switching to repeat all");
        emit(state.copyWith(repeatMode: RepeatMode.all));
        // Continue to play next song with repeat all behavior
      }

      // ============================================================================
      // PLAYLIST MODE - Manual navigation with repeat
      // ============================================================================
      if (state.musicMode == MusicMode.playlist) {
        logger.i("Next song in playlist");

        final int currentIndex = state.currentPlaylist.currentIndex ?? 0;

        // Safety checks
        if (currentIndex < 0 || currentIndex >= state.playlistSongs.length) {
          logger.w("Invalid playlist index: $currentIndex");
          return;
        }

        // If last song
        if (currentIndex == state.playlistSongs.length - 1) {
          if (state.repeatMode == RepeatMode.all) {
            // Repeat all - loop to first song
            logger.i("Repeat all enabled, looping to first song in playlist");
            final nextSong = state.playlistSongs.first;

            final media = Media(nextSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(nextSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentSong: nextSong,
                isPlaying: player.state.playing,
                currentPlaylist: state.currentPlaylist.copyWith(
                  currentIndex: 0,
                  currentSongId: nextSong.id,
                ),
              ),
            );

            logger.i("First song in playlist: ${nextSong.title}");
            return;
          } else {
            // No repeat - stop at end
            logger.i("Reached end of playlist");
            if (state.isPlaying) {
              // await player.pause();
              // emit(state.copyWith(isPlaying: false));
            }
            return;
          }
        }

        // Play next song in playlist
        final nextIndex = currentIndex + 1;
        final nextSong = state.playlistSongs[nextIndex];

        final media = Media(nextSong.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(nextSong);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentSong: nextSong,
            isPlaying: player.state.playing,
            currentPlaylist: state.currentPlaylist.copyWith(
              currentIndex: nextIndex,
              currentSongId: nextSong.id,
            ),
          ),
        );

        logger.i("Next song in playlist: ${nextSong.title}");
        return;
      }

      // ============================================================================
      // FAVORITES MODE - Navigate with repeat
      // ============================================================================
      if (state.musicMode == MusicMode.favorite) {
        logger.i("Next song in favorites");

        final currentInFavorites = state.favouriteSongs.indexWhere(
          (song) => song.id == state.currentSong!.id,
        );

        if (currentInFavorites == -1) {
          logger.w("Current song not found in favorites");
          return;
        }

        // If last song in favorites
        if (currentInFavorites == state.favouriteSongs.length - 1) {
          if (state.repeatMode == RepeatMode.all) {
            // Repeat all - loop to first favorite
            logger.i("Repeat all enabled, looping to first favorite");
            final nextSong = state.favouriteSongs.first;

            final media = Media(nextSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(nextSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentSong: nextSong,
                isPlaying: player.state.playing,
                error: null,
              ),
            );

            logger.i("Playing first favorite: ${nextSong.title}");
            return;
          } else {
            // No repeat - stop at end
            logger.i("Reached end of favorites");
            if (state.isPlaying) {
              // await player.pause();
              // emit(state.copyWith(isPlaying: false));
            }
            return;
          }
        }

        // Play next from favorites
        final nextIndex = currentInFavorites + 1;
        final nextSong = state.favouriteSongs[nextIndex];

        final media = Media(nextSong.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(nextSong);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentSong: nextSong,
            isPlaying: player.state.playing,
            error: null,
          ),
        );

        logger.i("Next from favorites: ${nextSong.title}");
        return;
      }

      // ============================================================================
      // NORMAL MODE - Queue/main list navigation with repeat and shuffle
      // ============================================================================
      SongInfo? nextSong;
      List<SongInfo> updatedQueue = state.playbackQueue;
      int? newCurrentIndex = state.currentIndex;

      // Check if we're currently in queue mode (queue is not empty)
      if (state.playbackQueue.isNotEmpty && newCurrentIndex != null) {
        // Currently in queue - use currentIndex
        if (newCurrentIndex < state.playbackQueue.length - 1) {
          // Not last in queue, play next from queue
          newCurrentIndex = newCurrentIndex + 1;
          nextSong = state.playbackQueue[newCurrentIndex];
          logger.i(
            "Next from queue at index $newCurrentIndex: ${nextSong.title}",
          );
        } else {
          // Last in queue
          if (state.repeatMode == RepeatMode.all) {
            // Repeat all - loop to first in queue
            logger.i("Repeat all enabled, looping to first song in queue");
            newCurrentIndex = 0;
            nextSong = state.playbackQueue[0];
            logger.i("First song in queue: ${nextSong.title}");
          } else {
            // No repeat - EXIT queue, CLEAR IT, and continue from last queue song's position
            final lastQueueSongInList = state.songs.indexWhere(
              (song) => song.id == state.playbackQueue.last.id,
            );

            if (lastQueueSongInList != -1) {
              if (state.isShuffled) {
                // Shuffle mode - pick random song from main list
                logger.i("Exited queue, shuffle enabled, picking random song");
                final availableSongs = List<SongInfo>.from(state.songs);
                availableSongs.shuffle();
                nextSong = availableSongs.first;
              } else {
                // Normal mode - continue sequential
                final nextIndex =
                    (lastQueueSongInList + 1) % state.songs.length;
                nextSong = state.songs[nextIndex];
              }
              updatedQueue = []; // CLEAR THE QUEUE after exiting
              newCurrentIndex = null; // Reset currentIndex
              logger.i(
                "Exited queue forward, CLEARING queue, continuing in main list: ${nextSong.title}",
              );
            }
          }
        }
      } else {
        // Currently NOT in queue - stay in main list
        if (state.isShuffled) {
          // Shuffle mode - pick random song excluding current
          logger.i("Shuffle enabled, picking random next song from main list");

          final availableSongs = List<SongInfo>.from(state.songs)
            ..removeWhere((song) => song.id == state.currentSong!.id);

          if (availableSongs.isEmpty) {
            logger.w("No more songs available for shuffle");
            return;
          }

          availableSongs.shuffle();
          nextSong = availableSongs.first;
          logger.i("Shuffled next from main list: ${nextSong.title}");
        } else {
          // Normal sequential mode
          final currentInList = state.songs.indexWhere(
            (song) => song.id == state.currentSong!.id,
          );

          if (currentInList != -1) {
            // Check if at last song and repeat all is enabled
            if (currentInList == state.songs.length - 1 &&
                state.repeatMode == RepeatMode.all) {
              logger.i(
                "Repeat all enabled, looping to first song in main list",
              );
              nextSong = state.songs.first;
            } else {
              final nextIndex = (currentInList + 1) % state.songs.length;
              nextSong = state.songs[nextIndex];
            }
            logger.i("Next from main list: ${nextSong.title}");
          }
        }
      }

      if (nextSong == null) {
        logger.w("Could not determine next song");
        return;
      }

      final media = Media(nextSong.path);
      await player.open(media, play: true);

      await songsRepository.addToRecentlyPlayed(nextSong);
      add(RecentSongs());
      emit(
        state.copyWith(
          currentSong: nextSong,
          playbackQueue: updatedQueue, // Update queue (might be cleared)
          currentIndex: newCurrentIndex, // Update currentIndex
          isPlaying: player.state.playing,
          error: null,
        ),
      );

      logger.i(
        "Playing: ${nextSong.title}, Queue length: ${updatedQueue.length}, currentIndex: $newCurrentIndex",
      );
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

      // ============================================================================
      // REPEAT ONE + Manual Skip - Switch to Repeat All
      // ============================================================================
      if (state.repeatMode == RepeatMode.one) {
        logger.i("Repeat one detected on manual skip, switching to repeat all");
        emit(state.copyWith(repeatMode: RepeatMode.all));
        // Continue to play previous song with repeat all behavior
      }

      // ============================================================================
      // PLAYLIST MODE - Manual navigation
      // ============================================================================
      if (state.musicMode == MusicMode.playlist) {
        final int currentIndex = state.currentPlaylist.currentIndex ?? 0;

        // Safety checks
        if (currentIndex < 0 || currentIndex >= state.playlistSongs.length) {
          logger.w("Invalid playlist index: $currentIndex");
          return;
        }

        // If first song
        if (currentIndex == 0) {
          if (state.repeatMode == RepeatMode.all) {
            // Loop to last song if repeat all is enabled
            logger.i(
              "At first song with repeat all, looping to last song in playlist",
            );
            final lastIndex = state.playlistSongs.length - 1;
            final prevSong = state.playlistSongs[lastIndex];

            final media = Media(prevSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(prevSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentSong: prevSong,
                isPlaying: player.state.playing,
                currentPlaylist: state.currentPlaylist.copyWith(
                  currentIndex: lastIndex,
                  currentSongId: prevSong.id,
                ),
              ),
            );

            logger.i("Last song in playlist: ${prevSong.title}");
            return;
          } else {
            logger.i("Reached start of playlist");
            return;
          }
        }

        // Play previous song in playlist
        final prevIndex = currentIndex - 1;
        final prevSong = state.playlistSongs[prevIndex];

        final media = Media(prevSong.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(prevSong);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentSong: prevSong,
            isPlaying: player.state.playing,
            currentPlaylist: state.currentPlaylist.copyWith(
              currentIndex: prevIndex,
              currentSongId: prevSong.id,
            ),
          ),
        );

        logger.i("Previous song in playlist: ${prevSong.title}");
        return;
      }

      // ============================================================================
      // FAVORITES MODE - Navigate backwards
      // ============================================================================
      if (state.musicMode == MusicMode.favorite) {
        logger.i("Previous song in favorites");

        final currentInFavorites = state.favouriteSongs.indexWhere(
          (song) => song.id == state.currentSong!.id,
        );

        if (currentInFavorites == -1) {
          logger.w("Current song not found in favorites, cannot go back");
          return;
        }

        // If first song in favorites
        if (currentInFavorites == 0) {
          if (state.repeatMode == RepeatMode.all) {
            // Loop to last favorite if repeat all is enabled
            logger.i(
              "At first favorite with repeat all, looping to last favorite",
            );
            final prevSong = state.favouriteSongs.last;

            final media = Media(prevSong.path);
            await player.open(media, play: true);

            await songsRepository.addToRecentlyPlayed(prevSong);
            add(RecentSongs());

            emit(
              state.copyWith(
                currentSong: prevSong,
                isPlaying: player.state.playing,
                error: null,
              ),
            );

            logger.i("Playing last favorite: ${prevSong.title}");
            return;
          } else {
            logger.i("At first song in favorites, cannot go previous");
            return;
          }
        }

        // Play previous from favorites
        final prevIndex = currentInFavorites - 1;
        final prevSong = state.favouriteSongs[prevIndex];

        final media = Media(prevSong.path);
        await player.open(media, play: true);

        await songsRepository.addToRecentlyPlayed(prevSong);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentSong: prevSong,
            isPlaying: player.state.playing,
            error: null,
          ),
        );

        logger.i("Previous from favorites: ${prevSong.title}");
        return;
      }

      // ============================================================================
      // NORMAL MODE - Using currentIndex
      // ============================================================================
      SongInfo? prevSong;
      int? newCurrentIndex = state.currentIndex;

      // Check if we're currently in queue mode (queue is not empty)
      if (state.playbackQueue.isNotEmpty && newCurrentIndex != null) {
        // Currently IN queue - use currentIndex
        if (newCurrentIndex == 0) {
          if (state.repeatMode == RepeatMode.all) {
            // Loop to last song in queue if repeat all is enabled
            logger.i("At first song in queue with repeat all, looping to last");
            newCurrentIndex = state.playbackQueue.length - 1;
            prevSong = state.playbackQueue[newCurrentIndex];
            logger.i("Last song in queue: ${prevSong.title}");
          } else {
            // First song in queue → EXIT queue and go to previous in main list
            final firstQueueSongInList = state.songs.indexWhere(
              (song) => song.id == state.playbackQueue.first.id,
            );

            if (firstQueueSongInList > 0) {
              final prevIndex = firstQueueSongInList - 1;
              prevSong = state.songs[prevIndex];
              newCurrentIndex = null; // Reset currentIndex (exiting queue)

              final media = Media(prevSong.path);
              await player.open(media);
              await songsRepository.addToRecentlyPlayed(prevSong);
              add(RecentSongs());

              emit(
                state.copyWith(
                  currentSong: prevSong,
                  playbackQueue: [], // CLEAR THE QUEUE
                  currentIndex: null,
                  isPlaying: player.state.playing,
                  error: null,
                ),
              );

              logger.i(
                "Exited queue backward, CLEARING queue, continuing in main list: ${prevSong.title}",
              );
              return;
            } else {
              // First song in both queue and main list → do nothing
              logger.i(
                "At first song in queue and main list, cannot go previous",
              );
              return;
            }
          }
        } else {
          // Play previous from queue
          newCurrentIndex = newCurrentIndex - 1;
          prevSong = state.playbackQueue[newCurrentIndex];
          logger.i(
            "Previous from queue at index $newCurrentIndex: ${prevSong.title}",
          );
        }
      } else {
        // Currently NOT in queue → main list
        final currentIndex = state.songs.indexWhere(
          (song) => song.id == state.currentSong!.id,
        );

        if (currentIndex <= 0) {
          if (state.repeatMode == RepeatMode.all && state.songs.isNotEmpty) {
            // Loop to last song if repeat all is enabled
            logger.i(
              "At first song in main list with repeat all, looping to last",
            );
            prevSong = state.songs.last;
          } else {
            // First song in main list → do nothing
            logger.i("At first song in main list, cannot go previous");
            return;
          }
        } else {
          prevSong = state.songs[currentIndex - 1];
          logger.i("Previous from main list: ${prevSong.title}");
        }
      }

      final media = Media(prevSong.path);
      await player.open(media);
      await songsRepository.addToRecentlyPlayed(prevSong);
      add(RecentSongs());
      emit(
        state.copyWith(
          currentSong: prevSong,
          currentIndex: newCurrentIndex, // Update currentIndex
          isPlaying: player.state.playing,
          error: null,
        ),
      );

      logger.i("Playing: ${prevSong.title}, currentIndex: $newCurrentIndex");
    } catch (e) {
      logger.e("Error playing previous song: $e");
      emit(state.copyWith(error: "Failed to play previous song: $e"));
    }
  }

  Future<void> _shuffleToggle(
    ShuffleToggle event,
    Emitter<SongsState> emit,
  ) async {
    try {
      final bool newShuffleState = event.isShuffle;
      logger.i("Toggling shuffle: $newShuffleState");

      // ============================================================================
      // PLAYLIST MODE - Shuffle playlist songs
      // ============================================================================
      if (state.musicMode == MusicMode.playlist) {
        if (newShuffleState) {
          // Enable shuffle - create shuffled version of playlist
          logger.i("Enabling shuffle for playlist");

          final currentSong = state.currentSong;
          if (currentSong == null) {
            logger.w("No current song to shuffle around");
            return;
          }

          // Create shuffled list excluding current song
          final remainingSongs = List<SongInfo>.from(state.playlistSongs)
            ..removeWhere((song) => song.id == currentSong.id);
          remainingSongs.shuffle();

          // Put current song at the beginning
          final shuffledPlaylist = [currentSong, ...remainingSongs];

          emit(
            state.copyWith(
              playlistSongs: shuffledPlaylist,
              isShuffled: true,
              currentPlaylist: state.currentPlaylist.copyWith(
                currentIndex: 0, // Current song is now at index 0
                isShuffle: true,
              ),
            ),
          );

          logger.i("Playlist shuffled with ${shuffledPlaylist.length} songs");
        } else {
          // Disable shuffle - restore original order
          logger.i("Disabling shuffle for playlist, restoring original order");

          // Fetch original playlist order from repository
          final originalPlaylist = await songsRepository.getPlaylistSongs(
            state.currentPlaylist.playlistId!,
          );

          // Find current song in original playlist
          final currentSongIndex = originalPlaylist.indexWhere(
            (song) => song.id == state.currentSong?.id,
          );

          emit(
            state.copyWith(
              playlistSongs: originalPlaylist,
              isShuffled: false,
              currentPlaylist: state.currentPlaylist.copyWith(
                currentIndex: currentSongIndex >= 0 ? currentSongIndex : 0,
                isShuffle: false,
              ),
            ),
          );

          logger.i("Playlist restored to original order");
        }
        return;
      }

      // ============================================================================
      // FAVORITES MODE - Shuffle favorites
      // ============================================================================
      if (state.musicMode == MusicMode.favorite) {
        if (newShuffleState) {
          // Enable shuffle - create shuffled version of favorites
          logger.i("Enabling shuffle for favorites");

          final currentSong = state.currentSong;
          if (currentSong == null) {
            logger.w("No current song to shuffle around");
            return;
          }

          // Create shuffled list excluding current song
          final remainingFavorites = List<SongInfo>.from(state.favouriteSongs)
            ..removeWhere((song) => song.id == currentSong.id);
          remainingFavorites.shuffle();

          // Put current song at the beginning
          final shuffledFavorites = [currentSong, ...remainingFavorites];

          emit(
            state.copyWith(favouriteSongs: shuffledFavorites, isShuffled: true),
          );

          logger.i("Favorites shuffled with ${shuffledFavorites.length} songs");
        } else {
          // Disable shuffle - restore original order
          logger.i("Disabling shuffle for favorites, restoring original order");

          // Fetch original favorites order from repository
          final originalFavorites = await songsRepository.getFavouriteSongs();

          emit(
            state.copyWith(
              favouriteSongs: originalFavorites,
              isShuffled: false,
            ),
          );

          logger.i("Favorites restored to original order");
        }
        return;
      }

      // ============================================================================
      // NORMAL MODE - Shuffle queue or main list
      // ============================================================================
      if (newShuffleState) {
        // Enable shuffle
        logger.i("Enabling shuffle for normal mode");

        final currentSong = state.currentSong;
        if (currentSong == null) {
          logger.w("No current song to shuffle around");
          emit(state.copyWith(isShuffled: true));
          return;
        }

        // If queue exists, shuffle the queue
        if (state.playbackQueue.isNotEmpty && state.currentIndex != null) {
          logger.i("Shuffling queue");

          final currentIndex = state.currentIndex!;

          // Split queue into played and remaining
          final remainingSongs =
              state.playbackQueue.sublist(currentIndex + 1).toList()..shuffle();

          // Rebuild queue: already played + current + shuffled remaining
          final shuffledQueue = [
            ...state.playbackQueue.sublist(0, currentIndex + 1),
            ...remainingSongs,
          ];

          emit(state.copyWith(playbackQueue: shuffledQueue, isShuffled: true));

          logger.i("Queue shuffled with ${shuffledQueue.length} songs");
        } else {
          // No queue - just mark as shuffled, will apply when playing next
          logger.i("No active queue, marking shuffle state");
          emit(state.copyWith(isShuffled: true));
        }
      } else {
        // Disable shuffle
        logger.i("Disabling shuffle for normal mode");

        // Just mark as not shuffled - original order is maintained
        // (We can't "unshuffle" without knowing original order)
        emit(state.copyWith(isShuffled: false));

        logger.i("Shuffle disabled");
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
        "Adding song to queue: ${event.songInfo.title}, "
        "playNext: ${event.playNext}",
      );

      // If no current song is playing and queue is empty, play this song immediately
      if (state.currentSong == null && state.playbackQueue.isEmpty) {
        final media = Media(event.songInfo.path);
        await player.open(media);
        await songsRepository.addToRecentlyPlayed(event.songInfo);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentSong: event.songInfo,
            playbackQueue: [event.songInfo],
            currentIndex: 0, // Initialize at index 0
            isPlaying: true,
            error: null,
            musicMode: MusicMode.normal,
          ),
        );

        logger.i("No current song, started playing: ${event.songInfo.title}");
        return;
      }

      // ============================================================================
      // PLAYLIST MODE → Convert to Queue Mode
      // ============================================================================
      if (state.musicMode == MusicMode.playlist) {
        logger.i("Converting playlist mode to queue mode");

        // Get current position in playlist
        final currentPlaylistIndex = state.currentPlaylist.currentIndex ?? 0;

        // Build new queue from remaining playlist songs (from current onwards)
        final remainingPlaylistSongs =
            state.playlistSongs.sublist(currentPlaylistIndex).toList();

        List<SongInfo> updatedQueue = List<SongInfo>.from(
          remainingPlaylistSongs,
        );
        int newCurrentIndex = 0; // Current song is now at index 0

        if (event.playNext) {
          // Insert after current song (index 0)
          updatedQueue.insert(1, event.songInfo);
          logger.i(
            "Inserted song as Play Next at index 1 after converting from playlist",
          );
        } else {
          // Append to end
          updatedQueue.add(event.songInfo);
          logger.i(
            "Appended song to end of queue after converting from playlist",
          );
        }

        emit(
          state.copyWith(
            playbackQueue: updatedQueue,
            currentIndex: newCurrentIndex,
            musicMode: MusicMode.normal, // Switch to normal mode
            currentPlaylist: const CurrentPlaylist(), // Clear playlist state
            playlistSongs: [], // Clear playlist songs
            error: null,
          ),
        );

        logger.i(
          "Converted playlist to queue. Queue length: ${updatedQueue.length}, currentIndex: $newCurrentIndex",
        );
        return;
      }

      // ============================================================================
      // FAVORITES MODE → Convert to Queue Mode
      // ============================================================================
      if (state.musicMode == MusicMode.favorite) {
        logger.i("Converting favorites mode to queue mode");

        // Find current song position in favorites
        final currentFavoriteIndex = state.favouriteSongs.indexWhere(
          (song) => song.id == state.currentSong?.id,
        );

        List<SongInfo> updatedQueue;
        int newCurrentIndex;

        if (currentFavoriteIndex != -1) {
          // Build queue from remaining favorites (from current onwards)
          final remainingFavorites =
              state.favouriteSongs.sublist(currentFavoriteIndex).toList();
          updatedQueue = List<SongInfo>.from(remainingFavorites);
          newCurrentIndex = 0; // Current song is now at index 0
        } else {
          // Current song not in favorites, start fresh with current song
          updatedQueue = [state.currentSong!];
          newCurrentIndex = 0;
        }

        if (event.playNext) {
          // Insert after current song (index 0)
          updatedQueue.insert(1, event.songInfo);
          logger.i(
            "Inserted song as Play Next at index 1 after converting from favorites",
          );
        } else {
          // Append to end
          updatedQueue.add(event.songInfo);
          logger.i(
            "Appended song to end of queue after converting from favorites",
          );
        }

        emit(
          state.copyWith(
            playbackQueue: updatedQueue,
            currentIndex: newCurrentIndex,
            musicMode: MusicMode.normal, // Switch to normal mode
            error: null,
          ),
        );

        logger.i(
          "Converted favorites to queue. Queue length: ${updatedQueue.length}, currentIndex: $newCurrentIndex",
        );
        return;
      }

      // ============================================================================
      // NORMAL MODE - Regular queue handling
      // ============================================================================
      final updatedQueue = List<SongInfo>.from(state.playbackQueue);
      int newCurrentIndex = state.currentIndex ?? 0;

      // Initialize queue with current song if empty
      if (updatedQueue.isEmpty && state.currentSong != null) {
        updatedQueue.add(state.currentSong!);
        newCurrentIndex = 0;
        logger.i(
          "Queue was empty, added current song first: ${state.currentSong!.title}",
        );
      }

      if (event.playNext) {
        // ▶️ PLAY NEXT → insert after current position
        updatedQueue.insert(newCurrentIndex + 1, event.songInfo);
        logger.i("Inserted song as Play Next at index ${newCurrentIndex + 1}");
      } else {
        // ➕ ADD TO QUEUE → append to end
        updatedQueue.add(event.songInfo);
        logger.i("Appended song to end of queue");
      }

      emit(
        state.copyWith(
          playbackQueue: updatedQueue,
          currentIndex: newCurrentIndex,
          error: null,
        ),
      );

      logger.i(
        "Queue updated successfully. Queue length: ${updatedQueue.length}, currentIndex: $newCurrentIndex",
      );
    } catch (e) {
      logger.e(
        "Error adding song to queue: ${event.songInfo.title}, error: $e",
      );
      emit(state.copyWith(error: "Failed to add song to queue: $e"));
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

  Future<void> playPlaylist(
    PlayPlaylistSongs event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i("Playing playlist: ${event.playlistId}");

      final index = event.songIndex ?? 0;

      // If same playlist is already playing and no specific index, just toggle
      if (state.currentPlaylist.playlistId == event.playlistId &&
          event.songIndex == null) {
        return add(TogglePlayPause());
      }

      final playlist = await songsRepository.getPlaylistSongs(event.playlistId);

      if (playlist.isEmpty) {
        logger.w("Playlist is empty");
        return;
      }

      if (index < 0 || index >= playlist.length) {
        logger.w("Invalid playlist index: $index");
        return;
      }

      // Play single song using Media (not Playlist)
      final songToPlay = playlist[index];
      final media = Media(songToPlay.path);
      await player.open(media, play: true);

      emit(
        state.copyWith(
          currentSong: songToPlay,
          playlistSongs: playlist, // Store playlist songs
          currentPlaylist: CurrentPlaylist(
            playlistId: event.playlistId,
            currentIndex: index,
            currentSongId: songToPlay.id,
            isShuffle: event.isShuffle,
          ),
          isPlaying: true,
          error: null,
          playbackQueue: [], // Clear queue
          currentIndex: null, // Not using currentIndex in playlist mode
          musicMode: MusicMode.playlist,
        ),
      );

      logger.i(
        "Playlist played successfully: ${songToPlay.title} at index $index",
      );
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

      await player.open(Media(event.song.path));

      await songsRepository.addToRecentlyPlayed(event.song);
      add(RecentSongs());

      // If shuffle is enabled, shuffle favorites list with current song first
      List<SongInfo> favoritesToUse = state.favouriteSongs;

      if (state.isShuffled && favoritesToUse.isNotEmpty) {
        logger.i("Shuffling favorites with current song first");

        final remainingFavorites =
            List<SongInfo>.from(favoritesToUse)
              ..removeWhere((song) => song.id == event.song.id)
              ..shuffle();

        favoritesToUse = [event.song, ...remainingFavorites];
        logger.i("Favorites shuffled");
      }

      emit(
        state.copyWith(
          isPlaying: true,
          currentSong: event.song,
          favouriteSongs: favoritesToUse,
          musicMode: MusicMode.favorite,
          playbackQueue: [], // Clear queue
          currentIndex: null, // Not using currentIndex in favorites mode
          currentPlaylist: const CurrentPlaylist(), // Clear playlist
          playlistSongs: [], // Clear playlist songs
          error: null,
        ),
      );

      logger.i("Favourite song playing successfully in favorites mode");
    } catch (e) {
      logger.e("Error playing favourite song: $e");
      emit(state.copyWith(error: "Failed to play favourite song: $e"));
    }
  }

  Future<void> _addPlaylistToQueue(
    AddPlaylistToQueue event,
    Emitter<SongsState> emit,
  ) async {
    try {
      logger.i(
        "Adding playlist to queue: ${event.playlistId}, playNext: ${event.playNext}",
      );

      // Fetch playlist songs from repository
      final playlistSongs = await songsRepository.getPlaylistSongs(
        event.playlistId,
      );

      if (playlistSongs.isEmpty) {
        logger.w("Playlist is empty: ${event.playlistId}");
        emit(state.copyWith(error: "Playlist is empty"));
        return;
      }

      // If no current song is playing, start playing the first song of playlist
      if (state.currentSong == null && state.playbackQueue.isEmpty) {
        final firstSong = playlistSongs.first;
        final media = Media(firstSong.path);
        await player.open(media);
        await songsRepository.addToRecentlyPlayed(firstSong);
        add(RecentSongs());

        emit(
          state.copyWith(
            currentSong: firstSong,
            playbackQueue: playlistSongs,
            currentIndex: 0,
            isPlaying: true,
            error: null,
            musicMode: MusicMode.normal,
          ),
        );

        logger.i(
          "No current song, started playing playlist. Queue length: ${playlistSongs.length}",
        );
        return;
      }

      // ============================================================================
      // PLAYLIST MODE → Convert to Queue Mode and add playlist songs
      // ============================================================================
      if (state.musicMode == MusicMode.playlist) {
        logger.i(
          "Converting current playlist mode to queue mode and adding new playlist",
        );

        // Get current position in existing playlist
        final currentPlaylistIndex = state.currentPlaylist.currentIndex ?? 0;

        // Build new queue from remaining current playlist songs (from current onwards)
        final remainingPlaylistSongs =
            state.playlistSongs.sublist(currentPlaylistIndex).toList();

        List<SongInfo> updatedQueue = List<SongInfo>.from(
          remainingPlaylistSongs,
        );
        int newCurrentIndex = 0; // Current song is now at index 0

        if (event.playNext) {
          // Insert entire playlist after current song (index 0)
          updatedQueue.insertAll(1, playlistSongs);
          logger.i(
            "Inserted ${playlistSongs.length} songs as Play Next at index 1 after converting from playlist",
          );
        } else {
          // Append entire playlist to end
          updatedQueue.addAll(playlistSongs);
          logger.i(
            "Appended ${playlistSongs.length} songs to end of queue after converting from playlist",
          );
        }

        emit(
          state.copyWith(
            playbackQueue: updatedQueue,
            currentIndex: newCurrentIndex,
            musicMode: MusicMode.normal, // Switch to normal mode
            currentPlaylist: const CurrentPlaylist(), // Clear playlist state
            playlistSongs: [], // Clear playlist songs
            error: null,
          ),
        );

        logger.i(
          "Converted playlist to queue and added new playlist. Queue length: ${updatedQueue.length}",
        );
        return;
      }

      // ============================================================================
      // FAVORITES MODE → Convert to Queue Mode and add playlist songs
      // ============================================================================
      if (state.musicMode == MusicMode.favorite) {
        logger.i("Converting favorites mode to queue mode and adding playlist");

        // Find current song position in favorites
        final currentFavoriteIndex = state.favouriteSongs.indexWhere(
          (song) => song.id == state.currentSong?.id,
        );

        List<SongInfo> updatedQueue;
        int newCurrentIndex;

        if (currentFavoriteIndex != -1) {
          // Build queue from remaining favorites (from current onwards)
          final remainingFavorites =
              state.favouriteSongs.sublist(currentFavoriteIndex).toList();
          updatedQueue = List<SongInfo>.from(remainingFavorites);
          newCurrentIndex = 0;
        } else {
          // Current song not in favorites, start fresh with current song
          updatedQueue = [state.currentSong!];
          newCurrentIndex = 0;
        }

        if (event.playNext) {
          // Insert entire playlist after current song (index 0)
          updatedQueue.insertAll(1, playlistSongs);
          logger.i(
            "Inserted ${playlistSongs.length} songs as Play Next at index 1 after converting from favorites",
          );
        } else {
          // Append entire playlist to end
          updatedQueue.addAll(playlistSongs);
          logger.i(
            "Appended ${playlistSongs.length} songs to end of queue after converting from favorites",
          );
        }

        emit(
          state.copyWith(
            playbackQueue: updatedQueue,
            currentIndex: newCurrentIndex,
            musicMode: MusicMode.normal, // Switch to normal mode
            error: null,
          ),
        );

        logger.i(
          "Converted favorites to queue and added playlist. Queue length: ${updatedQueue.length}",
        );
        return;
      }

      // ============================================================================
      // NORMAL MODE - Add playlist songs to existing queue
      // ============================================================================
      final updatedQueue = List<SongInfo>.from(state.playbackQueue);
      int newCurrentIndex = state.currentIndex ?? 0;

      // Initialize queue with current song if empty
      if (updatedQueue.isEmpty && state.currentSong != null) {
        updatedQueue.add(state.currentSong!);
        newCurrentIndex = 0;
        logger.i(
          "Queue was empty, added current song first: ${state.currentSong!.title}",
        );
      }

      if (event.playNext) {
        //  PLAY NEXT → insert entire playlist after current position
        updatedQueue.insertAll(newCurrentIndex + 1, playlistSongs);
        logger.i(
          "Inserted ${playlistSongs.length} playlist songs as Play Next at index ${newCurrentIndex + 1}",
        );
      } else {
        // ➕ ADD TO QUEUE → append entire playlist to end
        updatedQueue.addAll(playlistSongs);
        logger.i(
          "Appended ${playlistSongs.length} playlist songs to end of queue",
        );
      }

      emit(
        state.copyWith(
          playbackQueue: updatedQueue,
          currentIndex: newCurrentIndex,
          error: null,
        ),
      );

      logger.i(
        "Playlist added to queue successfully. Total queue length: ${updatedQueue.length}, currentIndex: $newCurrentIndex",
      );
    } catch (e) {
      logger.e(
        "Error adding playlist to queue: ${event.playlistId}, error: $e",
      );
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
          logger.i("Repeat mode changed: none → one");
          break;
        case RepeatMode.one:
          nextMode = RepeatMode.all;
          logger.i("Repeat mode changed: one → all");
          break;
        case RepeatMode.all:
          nextMode = RepeatMode.none;
          logger.i("Repeat mode changed: all → none");
          break;
      }

      emit(state.copyWith(repeatMode: nextMode));
      logger.i("Current repeat mode: $nextMode");
    } catch (e) {
      logger.e("Error toggling repeat mode: $e");
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

      // REPEAT ONE - Replay the same song
      if (state.repeatMode == RepeatMode.one && state.currentSong != null) {
        logger.i("Repeat one enabled, replaying song");
        final media = Media(state.currentSong!.path);
        await player.open(media, play: true);
        emit(state.copyWith(isPlaying: true));
        return;
      }

      // Otherwise, automatically play next song (handles repeat all internally)
      add(PlayNext());
    } catch (e) {
      logger.e("Error handling song completion: $e");
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

  @override
  Future<void> close() {
    player.dispose();
    return super.close();
  }
}
