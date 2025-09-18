import 'dart:io';
import 'dart:typed_data';
import 'package:audio_metadata_extractor/audio_metadata_extractor.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:media_kit/media_kit.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:uuid/uuid.dart';
import 'songs_event.dart';
import 'songs_state.dart';
import 'package:audio_metadata_reader/audio_metadata_reader.dart'
    hide AudioMetadata;
import 'package:hive/hive.dart';

class SongsBloc extends Bloc<SongsEvent, SongsState> {
  final Player player = Player();
  final logger = Logger();
  SongsBloc() : super(const SongsState()) {
    on<ScanSongs>(_onScanSongs);
    on<LoadSongsFromHive>(_onLoadSongsFromHive);
    on<SearchSong>(_onSearch);
    on<PlaySong>(_onPlaySong);
    on<TogglePlayPause>(_onTogglePlayPause);
    on<PlayNext>(_onPlayNext);
    on<PlayPrevious>(_onPlayPrevious);
    on<SeekSong>(_onSeekSong);
    on<UpdateDuration>(_onUpdateDuration);
    on<UpdatePosition>(_onUpdatePosition);

    // _initializePlayerListeners();

    add(LoadSongsFromHive());
  }

  void _initializePlayerListeners() {
    // Listen to position changes
    player.stream.position.listen((position) {
      if (!isClosed) {
        add(UpdatePosition(position));
      }
    });

    // Listen to duration changes
    player.stream.duration.listen((duration) {
      if (!isClosed) {
        add(UpdateDuration(duration));
      }
    });

    // Listen to playback completion
    player.stream.completed.listen((completed) {
      if (completed && !isClosed) {
        add(PlayNext());
      }
    });
  }

  Future<void> _onScanSongs(ScanSongs event, Emitter<SongsState> emit) async {
    emit(state.copyWith(isLoading: true, error: null));
    final logger = Logger();
    final uuid = Uuid();
    try {
      logger.i("Songs scanning started");
      final tempInfos = <SongInfo>[];
      final home = Directory('/home/mecha');

      final files =
          await home
              .list(recursive: true, followLinks: false)
              .where((entity) {
                final path = entity.path;
                if (path.split('/').any((p) => p.startsWith('.'))) return false;
                return entity is File &&
                    audioExt.any((ext) => path.toLowerCase().endsWith(ext));
              })
              .cast<File>()
              .toList();

      final seen = <String>{};
      logger.i("Total audio files found: ${files.length}");

      for (int index = 0; index < files.length; index++) {
        final file = files[index];
        try {
          final stat = await file.stat();
          final key = "${file.uri.pathSegments.last}_${stat.size}";
          if (!seen.add(key)) continue;

          final metadata = await readMetadata(file);
          final albumArtMetadata = await AudioMetadata.extract(file);

          tempInfos.add(
            SongInfo(
              index: index,
              id: uuid.v4(),
              path: file.path,
              title: metadata.title ?? file.uri.pathSegments.last,
              artist: metadata.artist ?? 'Unknown Artist',
              album: metadata.album,
              duration: metadata.duration?.toString(),
              artwork:
                  albumArtMetadata?.coverData != null
                      ? Uint8List.fromList(albumArtMetadata!.coverData!)
                      : null,
            ),
          );
        } catch (e) {
          logger.w("Error processing file ${file.path}: $e");
          tempInfos.add(
            SongInfo(
              index: index,
              id: uuid.v4(),
              path: file.path,
              title: file.uri.pathSegments.last,
              artist: 'Unknown Artist',
            ),
          );
        }
      }

      logger.i("Storing songs in Hive");
      final songsBox = await Hive.openBox<SongInfo>(TableName.songsInfoTable);
      await songsBox.clear();
      await songsBox.addAll(tempInfos);

      // Create initial playlist
      final playlist = Playlist(tempInfos.map((s) => Media(s.path)).toList());
      await player.open(playlist, play: false);

      emit(
        state.copyWith(
          songs: tempInfos,
          currentIndex: -1,
          playbackQueue: tempInfos,
          playlist: playlist,
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
    final logger = Logger();
    logger.i("Loading songs from Hive");
    emit(state.copyWith(isLoading: true, error: null));

    try {
      final songsBox = await Hive.openBox<SongInfo>(TableName.songsInfoTable);
      final songs = songsBox.values.toList();

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

  Future<void> _onSearch(SearchSong event, Emitter<SongsState> emit) async {
    final logger = Logger();
    logger.i("Search started: ${event.searchQuery}");

    if (event.searchQuery.isEmpty) {
      emit(state.copyWith(searchedSongs: []));
      return;
    }

    final searchSongs =
        state.songs.where((element) {
          final title = element.title.toLowerCase();
          final artist = element.artist.toLowerCase();
          final album = element.album?.toLowerCase() ?? "";
          final query = event.searchQuery.toLowerCase();

          return title.contains(query) ||
              artist.contains(query) ||
              album.contains(query);
        }).toList();

    logger.i("Search completed: ${searchSongs.length} results");
    emit(state.copyWith(searchedSongs: searchSongs));
  }

  Future<void> _onPlaySong(PlaySong event, Emitter<SongsState> emit) async {
    if (event.index < 0 || event.index >= state.playbackQueue.length) {
      logger.w("Invalid song index: ${event.index}");
      return;
    }

    try {
      final selectedSong = state.playbackQueue[event.index];

      await player.jump(event.index);
      await player.play();
      _initializePlayerListeners();
      emit(
        state.copyWith(
          currentIndex: event.index,
          isPlaying: true,
          currentSong: selectedSong,
          error: null,
        ),
      );
    } catch (e) {
      logger.e("Error playing song at index ${event.index}: $e");
      emit(state.copyWith(error: "Failed to play song: $e"));
    }
  }

  Future<void> _onTogglePlayPause(
    TogglePlayPause event,
    Emitter<SongsState> emit,
  ) async {
    try {
      if (state.currentIndex == -1 && state.playbackQueue.isNotEmpty) {
        // No song selected, play the first one
        add(PlaySong(0));
        return;
      }

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
    if (state.playbackQueue.isEmpty) return;

    try {
      final nextIndex = (state.currentIndex + 1) % state.playbackQueue.length;

      // If we're at the last song, we can either stop or loop back to first
      // Here we're implementing loop behavior
      await player.next();

      final nextSong = state.playbackQueue[nextIndex];

      emit(
        state.copyWith(
          currentIndex: nextIndex,
          currentSong: nextSong,
          isPlaying: true,
          error: null,
        ),
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
    if (state.playbackQueue.isEmpty) return;

    try {
      final prevIndex =
          state.currentIndex <= 0
              ? state.playbackQueue.length - 1
              : state.currentIndex - 1;

      await player.previous();

      final prevSong = state.playbackQueue[prevIndex];

      emit(
        state.copyWith(
          currentIndex: prevIndex,
          currentSong: prevSong,
          isPlaying: true,
          error: null,
        ),
      );
    } catch (e) {
      logger.e("Error playing previous song: $e");
      emit(state.copyWith(error: "Failed to play previous song: $e"));
    }
  }

  Future<void> _onSeekSong(SeekSong event, Emitter<SongsState> emit) async {
    try {
      await player.seek(event.position);
      emit(state.copyWith(position: event.position));
    } catch (e) {
      logger.e("Error seeking to position ${event.position}: $e");
      emit(state.copyWith(error: "Failed to seek: $e"));
    }
  }

  Future<void> _onUpdatePosition(
    UpdatePosition event,
    Emitter<SongsState> emit,
  ) async {
    emit(state.copyWith(position: event.position));
  }

  Future<void> _onUpdateDuration(
    UpdateDuration event,
    Emitter<SongsState> emit,
  ) async {
    emit(state.copyWith(duration: event.duration));
  }

  @override
  Future<void> close() {
    player.dispose();
    return super.close();
  }
}
