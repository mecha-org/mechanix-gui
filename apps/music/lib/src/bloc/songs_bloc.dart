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
    on<SearchSong>(_onSearch);
    on<PlaySong>(_onPlaySong);
    on<TogglePlayPause>(_onTogglePlayPause);
    on<PlayNext>(_onPlayNext);
    on<PlayPrevious>(_onPlayPrevious);
    on<ShuffleToggle>(_shuffleToggle);
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
      emit(
        state.copyWith(
          currentSong: event.song,
          isPlaying: true,
          error: null,
          playbackQueue: [],
          currentPlaylist: const CurrentPlaylist(),
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

      if (state.musicMode == MusicMode.playlist) {
        logger.i("Next song in playlist");

        final playlist = player.state.playlist;

        if (state.currentPlaylist.isShuffle != null &&
            state.currentPlaylist.isShuffle!) {
          await player.next();
          final newIndex = player.state.playlist.index;
          final nextSong = state.playlistSongs[newIndex];

          emit(
            state.copyWith(
              currentSong: nextSong,
              isPlaying: player.state.playing,
              currentPlaylist: state.currentPlaylist.copyWith(
                currentIndex: newIndex,
                currentSongId: nextSong.id,
              ),
            ),
          );
        } else {
          final int currentIndex = playlist.index;

          // Safety checks
          if (currentIndex < 0 || currentIndex >= state.playlistSongs.length) {
            logger.w("Invalid playlist index: $currentIndex");
            return;
          }

          // If last song → stop or loop (your choice)
          if (currentIndex == state.playlistSongs.length - 1) {
            logger.i("Reached end of playlist");
            if (state.isPlaying) {
              emit(state.copyWith(isPlaying: player.state.playing));
            }

            return;
          }

          final nextIndex = currentIndex + 1;
          final nextSong = state.playlistSongs[nextIndex];

          // Move player to next track
          await player.next();
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
        }

        return;
      }

      SongInfo? nextSong;
      List<SongInfo> updatedQueue = state.playbackQueue;

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
          // Last in queue, EXIT queue, CLEAR IT, and continue from last queue song's position
          final lastQueueSongInList = state.songs.indexWhere(
            (song) => song.id == state.playbackQueue.last.id,
          );

          if (lastQueueSongInList != -1) {
            final nextIndex = (lastQueueSongInList + 1) % state.songs.length;
            nextSong = state.songs[nextIndex];
            updatedQueue = []; // CLEAR THE QUEUE after exiting
            logger.i(
              "Exited queue forward, CLEARING queue, continuing in main list: ${nextSong.title}",
            );
          }
        }
      } else {
        // Currently NOT in queue - stay in main list
        final currentInList = state.songs.indexWhere(
          (song) => song.id == state.currentSong!.id,
        );

        if (currentInList != -1) {
          final nextIndex = (currentInList + 1) % state.songs.length;
          nextSong = state.songs[nextIndex];
          logger.i("Next from main list: ${nextSong.title}");
        }
      }

      if (nextSong == null) {
        logger.w("Could not determine next song");
        return;
      }

      final media = Media(nextSong.path);
      await player.open(media);

      await songsRepository.addToRecentlyPlayed(nextSong);
      add(RecentSongs());
      emit(
        state.copyWith(
          currentSong: nextSong,
          playbackQueue: updatedQueue, // Update queue (might be cleared)
          isPlaying: player.state.playing,
          error: null,
        ),
      );

      logger.i(
        "Playing: ${nextSong.title}, Queue length: ${updatedQueue.length}",
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

      if (state.musicMode == MusicMode.playlist) {
        final playlist = player.state.playlist;

        final int currentIndex = playlist.index;

        // Safety checks
        if (currentIndex < 0 || currentIndex >= state.playlistSongs.length) {
          logger.w("Invalid playlist index: $currentIndex");
          return;
        }

        // If first song → stop or loop (your choice)
        if (currentIndex == 0) {
          logger.i("Reached start of playlist");
          return;
        }

        final prevIndex = currentIndex - 1;
        final prevSong = state.playlistSongs[prevIndex];

        // Move player to previous track
        await player.previous(); // OR: await player.jump(prevIndex);

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

      SongInfo? prevSong;

      // Check if current song is in queue
      final queueIndex = state.playbackQueue.indexWhere(
        (song) => song.id == state.currentSong!.id,
      );

      if (queueIndex != -1) {
        // 🔹 Currently IN queue
        if (queueIndex == 0) {
          // 🚫 First song in queue → do nothing
          logger.i("At first song in queue, cannot go previous");
          return;
        }

        // Play previous from queue
        prevSong = state.playbackQueue[queueIndex - 1];
        logger.i("Previous from queue: ${prevSong.title}");
      } else {
        // 🔹 Currently NOT in queue → main list
        final currentIndex = state.songs.indexWhere(
          (song) => song.id == state.currentSong!.id,
        );

        if (currentIndex <= 0) {
          // 🚫 First song in main list → do nothing
          logger.i("At first song in main list, cannot go previous");
          return;
        }

        prevSong = state.songs[currentIndex - 1];
        logger.i("Previous from main list: ${prevSong.title}");
      }

      final media = Media(prevSong.path);
      await player.open(media);
      await songsRepository.addToRecentlyPlayed(prevSong);
      add(RecentSongs());
      emit(
        state.copyWith(
          currentSong: prevSong,
          isPlaying: player.state.playing,
          error: null,
        ),
      );

      logger.i("Playing: ${prevSong.title}");
    } catch (e) {
      logger.e("Error playing previous song: $e");
      emit(state.copyWith(error: "Failed to play previous song: $e"));
    }
  }

  Future<void> _shuffleToggle(
    ShuffleToggle event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Toggling shuffle ${event.isShuffle} ");
    final isUpdated = await songsRepository.shuffleToggle(
      event.playlistId,
      event.isShuffle,
    );

    if (isUpdated) {
      await player.setShuffle(event.isShuffle);
    }
    add(LoadPlaylist());
    // emit(state.copyWith());
    // logger.i("Shuffle: ${state.isShuffled}");
  }

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
                      .map(
                        (song) =>
                            event.songIds.contains(song.id)
                                ? song = song.copyWith(
                                  isFavourite: event.isFavourite,
                                )
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
            isPlaying: true,
            error: null,
          ),
        );

        logger.i("No current song, started playing: ${event.songInfo.title}");
        return;
      }

      final updatedQueue = List<SongInfo>.from(state.playbackQueue);

      // // 🚫 Avoid duplicates
      // if (updatedQueue.any((s) => s.id == event.songInfo.id)) {
      //   logger.i("Song already exists in queue, skipping");
      //   return;
      // }

      // Initialize queue with current song if empty
      if (updatedQueue.isEmpty && state.currentSong != null) {
        updatedQueue.add(state.currentSong!);
        logger.i(
          "Queue was empty, added current song first: ${state.currentSong!.title}",
        );
      }

      if (event.playNext) {
        // ▶️ PLAY NEXT → insert after current song
        final currentIndex = updatedQueue.indexWhere(
          (song) => song.id == state.currentSong?.id,
        );

        if (currentIndex != -1) {
          updatedQueue.insert(currentIndex + 1, event.songInfo);
          logger.i("Inserted song as Play Next at index ${currentIndex + 1}");
        } else {
          // Safety fallback
          updatedQueue.add(event.songInfo);
          logger.w("Current song not found in queue, appended to end");
        }
      } else {
        // ➕ ADD TO QUEUE → append to end
        updatedQueue.add(event.songInfo);
        logger.i("Appended song to end of queue");
      }

      emit(state.copyWith(playbackQueue: updatedQueue, error: null));

      logger.i(
        "Queue updated successfully. Queue length: ${updatedQueue.length}",
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
    emit(state.copyWith(bottomBarView: BottomBarView.normal));
    logger.i("Playlist created successfully");
    add(LoadPlaylist());
  }

  Future<void> loadPlaylist(
    LoadPlaylist event,
    Emitter<SongsState> emit,
  ) async {
    logger.i("Loading playlist");
    final playlist = await songsRepository.getPlaylist();

    emit(
      state.copyWith(bottomBarView: BottomBarView.normal, playlists: playlist),
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
          playlistSongs:
              event.isMusicList
                  ? [...state.playlistSongs, ...updatedSongs]
                  : state.playlistSongs,
          songs:
              state.songs.map((song) {
                // Replace song if it was updated, otherwise keep original
                return updatedSongsMap[song.id] ?? song;
              }).toList(),
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
      if (state.currentPlaylist.playlistId == event.playlistId &&
          event.songIndex == null) {
        return add(TogglePlayPause());
      }
      final playlist = await songsRepository.getPlaylistSongs(event.playlistId);
      await player.open(
        Playlist(
          playlist.map((song) => Media(song.path)).toList(),
          index: index,
        ),
        play: true,
      );

      // await player.setPlaylistMode(PlaylistMode.single);
      // await player.setShuffle(event.isShuffle);

      emit(
        state.copyWith(
          currentSong: state.playlistSongs[index],
          currentPlaylist: CurrentPlaylist(
            playlistId: event.playlistId,
            currentIndex: index,
            currentSongId: state.playlistSongs[index].id,
          ),
          isPlaying: true,
          error: null,
          playbackQueue: [],
          musicMode: MusicMode.playlist,
        ),
      );
      logger.i("Playlist played successfully");
    } catch (e) {
      logger.e("Error playing playlist: $e");
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

  @override
  Future<void> close() {
    player.dispose();
    return super.close();
  }
}
