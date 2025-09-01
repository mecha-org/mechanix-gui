import 'dart:io';
import 'package:audio_metadata_reader/audio_metadata_reader.dart'
    hide AudioMetadata;
import 'package:camera/src/features/player/models/types.dart';
import 'package:media_kit/media_kit.dart';
import 'package:hive/hive.dart';
import 'package:flutter/foundation.dart';
import 'package:audio_metadata_extractor/audio_metadata_extractor.dart';

class MusicPlayerManager extends ChangeNotifier {
  static final MusicPlayerManager instance = MusicPlayerManager._internal();
  factory MusicPlayerManager() => instance;

  late final Player player;
  late final Playlist playlist;

  // Hive box for playlists (stores List<String>)
  final Box<List> _playlistBox = Hive.box('playlistBox');
  final Box<List> _metaBox = Hive.box('metaBox');

  List<String> originalTracks = [];
  List<SongInfo> songInfos = [];

  List<SongInfo> playbackQueue = []; // active queue (all songs or album only)
  int currentIndex = 0;

  MusicPlayerManager._internal() {
    player = Player();

    final saved = _playlistBox.get('songs', defaultValue: [])!.cast<String>();
    final savedMeta = _metaBox.get('info', defaultValue: [])!.cast<Map>();

    if (saved.isNotEmpty) {
      playlist = Playlist(saved.map((p) => Media(p)).toList());
      player.open(playlist, play: false);

      if (savedMeta.isNotEmpty) {
        // restore full metadata
        songInfos =
            savedMeta
                .map((m) => SongInfo.fromJson(Map<String, dynamic>.from(m)))
                .toList();
      } else {
        // fallback if no metadata yet
        songInfos =
            saved.map((path) {
              final file = File(path);
              return SongInfo(
                path: path,
                title: file.uri.pathSegments.last,
                artist: "Unknown Artist",
              );
            }).toList();
      }

      // keep your playbackQueue in sync
      playbackQueue = songInfos;
    } else {
      playlist = Playlist([]);
      player.open(playlist, play: false);
    }
  }

  Future<void> playAt(int index) async {
    if (index < playlist.medias.length) {
      await player.jump(index);
    }
  }

  Future<void> clearPlaylist() async {
    playlist.medias.clear();
    await _playlistBox.delete('songs');
  }

  Future<void> playAlbum(List<SongInfo> albumSongs, int startIndex) async {
    final player = this.player;

    // Create a playback queue just for this album
    playbackQueue = albumSongs;

    // Open the album queue in the player
    await player.open(
      Playlist(
        albumSongs.map((s) => Media(s.path)).toList(),
        index: startIndex,
      ),
      play: true,
    );

    currentIndex = startIndex;

    // Notify UI (mini player, now playing screen)
    notifyListeners(); // or setState(() {})
  }

  Future<void> playAllSongs(int startIndex) async {
    final player = this.player;

    playbackQueue = allSongInfos;

    await player.open(
      Playlist(
        allSongInfos.map((s) => Media(s.path)).toList(),
        index: startIndex,
      ),
      play: true,
    );

    currentIndex = startIndex;
    notifyListeners();
  }

  Future<void> scanAndSaveSongs() async {
    final tempInfos = <SongInfo>[]; // collect first
    final home = Directory('/home');
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

    // only replace now (UI never goes empty mid-scan)
    songInfos = tempInfos;

    // after songInfos = tempInfos;
    if (currentIndex >= 0 && currentIndex < songInfos.length) {
      final currentPath =
          playbackQueue.isNotEmpty ? playbackQueue[currentIndex].path : null;

      if (currentPath != null) {
        final restoredIdx = songInfos.indexWhere((s) => s.path == currentPath);
        if (restoredIdx != -1) {
          currentIndex = restoredIdx;
          playbackQueue = songInfos;
        }
      }
    }

    await _playlistBox.put('songs', uniquePaths);

    // Also store metadata for restore
    await _metaBox.put('info', tempInfos.map((s) => s.toJson()).toList());

    notifyListeners(); // update UI
  }

  Future<void> playFilteredSongs(
    List<SongInfo> filteredSongs,
    int startIndex,
  ) async {
    final player = this.player;

    // Override playbackQueue with only the filtered results
    playbackQueue = filteredSongs;

    await player.open(
      Playlist(
        filteredSongs.map((s) => Media(s.path)).toList(),
        index: startIndex,
      ),
      play: true,
    );

    currentIndex = startIndex;
  }

  /// Load all saved songs from Hive
  List<String> get allSongs {
    return _playlistBox.get('songs', defaultValue: <String>[])!.cast<String>();
  }

  /// Return all metadata objects (safe for UI use)
  List<SongInfo> get allSongInfos => List.unmodifiable(songInfos);
}
