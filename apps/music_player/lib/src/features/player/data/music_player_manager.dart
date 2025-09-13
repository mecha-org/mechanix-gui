import 'dart:io';
import 'package:audio_metadata_reader/audio_metadata_reader.dart' hide AudioMetadata;
import 'package:music_player/src/features/player/models/types.dart';
import 'package:media_kit/media_kit.dart';
import 'package:hive/hive.dart';
import 'package:flutter/foundation.dart';
import 'package:audio_metadata_extractor/audio_metadata_extractor.dart';

class MusicPlayerManager extends ChangeNotifier {
  static final MusicPlayerManager instance = MusicPlayerManager._internal();
  factory MusicPlayerManager() => instance;

  late final Player player;
  late final Playlist playlist;

  // Hive boxes
  final Box<List> _playlistBox = Hive.box('playlistBox'); 
  final Box<List> _metaBox = Hive.box('metaBox');
  final Box<List> _recentBox = Hive.box('recentBox'); // new for recently played

  List<String> originalTracks = [];
  List<SongInfo> songInfos = [];
  List<SongInfo> playbackQueue = []; 
  int currentIndex = 0;

  // Recently played
  List<SongInfo> recentSongs = [];

  MusicPlayerManager._internal() {
    player = Player();

    final saved = _playlistBox.get('songs', defaultValue: [])!.cast<String>();
    final savedMeta = _metaBox.get('info', defaultValue: [])!.cast<Map>();

    if (saved.isNotEmpty) {
      playlist = Playlist(saved.map((p) => Media(p)).toList());
      player.open(playlist, play: false);

      if (savedMeta.isNotEmpty) {
        songInfos = savedMeta
            .map((m) => SongInfo.fromJson(Map<String, dynamic>.from(m)))
            .toList();
      } else {
        songInfos = saved.map((path) {
          final file = File(path);
          return SongInfo(
            path: path,
            title: file.uri.pathSegments.last,
            artist: "Unknown Artist",
          );
        }).toList();
      }

      playbackQueue = songInfos;
    } else {
      playlist = Playlist([]);
      player.open(playlist, play: false);
    }

    // Restore recent songs from Hive
    final savedRecent = _recentBox.get('songs', defaultValue: [])!.cast<Map>();
    if (savedRecent.isNotEmpty) {
      recentSongs = savedRecent
          .map((m) => SongInfo.fromJson(Map<String, dynamic>.from(m)))
          .toList();
    } else if (savedRecent == null || (savedRecent as List).isEmpty) {
      recentSongs = songInfos.take(5).toList();
      _recentBox.put('songs', recentSongs.map((s) => s.toJson()).toList());

    }
  }

  Future<void> playAt(int index) async {
    if (index < playlist.medias.length) {
      await player.jump(index);
      if (index >= 0 && index < playbackQueue.length) {
        _addToRecent(playbackQueue[index]);
      }
    }
  }

  Future<void> clearPlaylist() async {
    playlist.medias.clear();
    await _playlistBox.delete('songs');
  }

  Future<void> playAlbum(List<SongInfo> albumSongs, int startIndex) async {
    playbackQueue = albumSongs;

    await player.open(
      Playlist(albumSongs.map((s) => Media(s.path)).toList(), index: startIndex),
      play: true,
    );

    currentIndex = startIndex;
    _addToRecent(albumSongs[startIndex]);
    notifyListeners();
  }

  Future<void> playAllSongs(int startIndex) async {
    playbackQueue = allSongInfos;

    await player.open(
      Playlist(allSongInfos.map((s) => Media(s.path)).toList(), index: startIndex),
      play: true,
    );

    currentIndex = startIndex;
    _addToRecent(allSongInfos[startIndex]);
    notifyListeners();
  }

  Future<void> playFilteredSongs(List<SongInfo> filteredSongs, int startIndex) async {
    playbackQueue = filteredSongs;

    await player.open(
      Playlist(filteredSongs.map((s) => Media(s.path)).toList(), index: startIndex),
      play: true,
    );

    currentIndex = startIndex;
    _addToRecent(filteredSongs[startIndex]);
  }

  Future<void> playSong(SongInfo song) async {
    await player.open(
      Playlist([Media(song.path)], index: 0),
      play: true,
    );
    playbackQueue = [song];
    currentIndex = 0;
    _addToRecent(song);
  }

  /// PRIVATE: Handle adding to recent list
  Future<void> _addToRecent(SongInfo song) async {
    recentSongs.removeWhere((s) => s.path == song.path);
    recentSongs.insert(0, song);

    if (recentSongs.length > 20) {
      recentSongs = recentSongs.sublist(0, 20);
    }

    await _recentBox.put(
      'songs',
      recentSongs.map((s) => s.toJson()).toList(),
    );

    notifyListeners();
  }

  Future<void> scanAndSaveSongs() async {
    final tempInfos = <SongInfo>[];
    final home = Directory('/home');
    final files = await home
        .list(recursive: true, followLinks: false)
        .where((entity) {
          final path = entity.path;
          if (path.split('/').any((p) => p.startsWith('.'))) return false;
          return entity is File && audioExt.any((ext) => path.toLowerCase().endsWith(ext));
        })
        .cast<File>()
        .toList();

    final seen = <String>{};
    final uniquePaths = <String>[];

    for (final file in files) {
      try {
        final stat = await file.stat();
        final key = "${file.uri.pathSegments.last}_${stat.size}";
        if (!seen.add(key)) continue;
        uniquePaths.add(file.path);

        // Await metadata reading
        final metadata = await readMetadata(file);
        debugPrint("Metadata for ${file.path}: $metadata");
        final albumArtMetadata = await AudioMetadata.extract(file);

        tempInfos.add(
          SongInfo(
            path: file.path,
            title: metadata.title ?? file.uri.pathSegments.last,
            artist: metadata.artist ?? 'Unknown Artist',
            album: metadata.album,
            duration: metadata.duration,
            artwork:
                albumArtMetadata?.coverData != null
                    ? Uint8List.fromList(albumArtMetadata!.coverData!)
                    : null,
          ),
        );
      } catch (e) {
        debugPrint("Metadata error for ${file.path}: $e");
        tempInfos.add(
          SongInfo(
            path: file.path,
            title: file.uri.pathSegments.last,
            artist: 'Unknown Artist',
          ),
        );
      }
    }

    songInfos = tempInfos;

    if (currentIndex >= 0 && currentIndex < songInfos.length) {
      final currentPath = playbackQueue.isNotEmpty ? playbackQueue[currentIndex].path : null;

      if (currentPath != null) {
        final restoredIdx = songInfos.indexWhere((s) => s.path == currentPath);
        if (restoredIdx != -1) {
          currentIndex = restoredIdx;
          playbackQueue = songInfos;
        }
      }
    }

    await _playlistBox.put('songs', uniquePaths);
    await _metaBox.put('info', tempInfos.map((s) => s.toJson()).toList());

    notifyListeners();
  }

  

  /// Load all saved songs from Hive
  List<String> get allSongs {
    return _playlistBox.get('songs', defaultValue: <String>[])!.cast<String>();
  }

  /// Return all metadata objects (safe for UI use)
  List<SongInfo> get allSongInfos => List.unmodifiable(songInfos);
}