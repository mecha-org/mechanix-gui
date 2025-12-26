import 'dart:io';

import 'package:audio_metadata_extractor/audio_metadata_extractor.dart';
import 'package:hive/hive.dart';
import 'package:logger/web.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/recently_played.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/features/home/data/songs_repository.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:uuid/uuid.dart';

class SongsRepositoryImpl extends SongsRepository {
  final logger = Logger();
  final uuid = Uuid();

  Future<void> ensureSongsConnected() async {
    if (!Hive.isBoxOpen(TableName.songsInfoTable)) {
      await Hive.openBox<SongInfo>(TableName.songsInfoTable);
    }
  }

  Future<void> ensureRecentlyPlayedConnected() async {
    if (!Hive.isBoxOpen(TableName.recentlyPlayedTable)) {
      await Hive.openBox<RecentlyPlayed>(TableName.recentlyPlayedTable);
    }
  }

  Future<void> ensurePlaylistConnected() async {
    if (!Hive.isBoxOpen(TableName.playlistTable)) {
      await Hive.openBox<PlaylistInfo>(TableName.playlistTable);
    }
  }

  @override
  Future<List<SongInfo>> scanAllSongs() async {
    logger.i("Scanning all songs");

    try {
      logger.i("Songs scanning started");

      // Get cache directory for artwork
      final cacheDir = await _getArtworkCacheDirectory();

      final tempInfos = <SongInfo>[];
      final home = Directory('/home/dhanish');

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

          final albumArtMetadata = await AudioMetadata.extract(file);

          // Save artwork to cache and get path
          String? artworkCachePath;
          if (albumArtMetadata?.coverData != null) {
            artworkCachePath = await _saveArtworkToCache(
              albumArtMetadata!.coverData!,
              uuid.v4(),
              cacheDir,
            );
          }

          tempInfos.add(
            SongInfo(
              index: index,
              id: uuid.v4(),
              path: file.path,
              title: albumArtMetadata?.trackName ?? file.uri.pathSegments.last,
              artist: albumArtMetadata?.firstArtists ?? 'Unknown Artist',
              album: albumArtMetadata?.album,
              duration: albumArtMetadata?.duration?.toString(),
              artworkPath: artworkCachePath, // Store file path instead of bytes
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
      await ensureSongsConnected();
      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);
      await songsBox.clear();
      for (final song in tempInfos) {
        await songsBox.put(song.id, song);
      }
      logger.i("Songs scanning completed");
      return tempInfos;
    } catch (e) {
      logger.e("Error scanning songs: $e");
      return [];
    }
  }

  /// Save artwork to cache directory and return the file path
  Future<String?> _saveArtworkToCache(
    List<int> coverData,
    String songId,
    Directory cacheDir,
  ) async {
    try {
      // Determine file extension based on image signature
      final extension = _getImageExtension(coverData);
      final filename = '$songId.$extension';
      final artworkFile = File(path.join(cacheDir.path, filename));

      await artworkFile.writeAsBytes(coverData);
      return artworkFile.path;
    } catch (e) {
      Logger().w("Error saving artwork: $e");
      return null;
    }
  }

  String _getImageExtension(List<int> data) {
    if (data.length < 4) return 'jpg';

    // PNG: 89 50 4E 47
    if (data[0] == 0x89 &&
        data[1] == 0x50 &&
        data[2] == 0x4E &&
        data[3] == 0x47) {
      return 'png';
    }
    // JPEG: FF D8 FF
    if (data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF) {
      return 'jpg';
    }
    // WebP: RIFF....WEBP
    if (data[0] == 0x52 &&
        data[1] == 0x49 &&
        data[2] == 0x46 &&
        data[3] == 0x46) {
      return 'webp';
    }

    return 'jpg'; // default
  }

  /// Optional: Clear old artwork cache (call this periodically or on app start)
  Future<void> clearArtworkCache() async {
    try {
      final cacheDir = await _getArtworkCacheDirectory();
      if (await cacheDir.exists()) {
        await cacheDir.delete(recursive: true);
        await cacheDir.create();
      }
    } catch (e) {
      Logger().e("Error clearing artwork cache: $e");
    }
  }

  /// Get or create the artwork cache directory
  Future<Directory> _getArtworkCacheDirectory() async {
    final cacheDir = await getApplicationCacheDirectory();
    final artworkDir = Directory(path.join(cacheDir.path, 'artwork'));

    if (!await artworkDir.exists()) {
      await artworkDir.create(recursive: true);
    }

    return artworkDir;
  }

  @override
  Future<List<SongInfo>> getAllSongs() async {
    logger.i("Loading songs from Hive");
    // emit(state.copyWith(isLoading: true, error: null));
    await ensureSongsConnected();
    try {
      final songsBox = await Hive.openBox<SongInfo>(TableName.songsInfoTable);
      final songs = songsBox.values.toList();
      return songs;
    } catch (e) {
      logger.e("Error loading songs from Hive: $e");
      return [];
    }
  }

  @override
  Future<SongInfo> deleteSong(SongInfo songInfo) async {
    try {
      logger.i("Deleting song from Hive ${songInfo.id}");
      await ensureSongsConnected();

      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);
      final songDetails = songsBox.get(songInfo.id);

      if (songDetails != null && songDetails.playlistIds.isNotEmpty) {
        await ensurePlaylistConnected();
        final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);

        for (final playlistId in songDetails.playlistIds) {
          final playlist = playlistBox.get(playlistId);
          if (playlist == null) continue;

          // 🔹 Remove song from playlist
          final updatedSongIds =
              playlist.songIds.where((id) => id != songInfo.id).toList();

          String? updatedCoverImagePath = playlist.coverImagePath;
          for (final songId in playlist.songIds) {
            final song = songsBox.get(songId);
            if (song?.artworkPath != null) {
              updatedCoverImagePath = song!.artworkPath;
              break;
            }
          }

          // // 🔹 If deleted song was cover → pick new one
          // if (songInfo.artworkPath != null &&
          //     songInfo.artworkPath == playlist.coverImagePath) {
          //   updatedCoverImagePath = null;

          //   for (final remainingSongId in updatedSongIds) {
          //     final remainingSong = songsBox.get(remainingSongId);
          //     if (remainingSong?.artworkPath != null) {
          //       updatedCoverImagePath = remainingSong!.artworkPath;
          //       break;
          //     }
          //   }
          // }

          final updatedPlaylist = playlist.copyWith(
            songIds: updatedSongIds,
            coverImagePath: updatedCoverImagePath,
          );

          await playlistBox.put(playlistId, updatedPlaylist);
        }
      }

      await songsBox.delete(songInfo.id);
      await ensureRecentlyPlayedConnected();

      final recentSongs = Hive.box<RecentlyPlayed>(
        TableName.recentlyPlayedTable,
      );
      await recentSongs.delete(songInfo.id);
      logger.i("Deleted song from Hive");

      return songInfo;
    } catch (e, stack) {
      logger.e("Error deleting song", error: e, stackTrace: stack);
      return songInfo;
    }
  }

  @override
  Future<bool> toggleFavouriteSong(SongInfo songInfo, bool isFavourite) async {
    try {
      await ensureSongsConnected();
      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);
      final updatedSong = songInfo.copyWith(isFavourite: isFavourite);

      await songsBox.put(updatedSong.id, updatedSong);
      return true;
    } catch (e) {
      logger.e("Error toggling favourite: $e");
      return false;
    }
  }

  @override
  Future<void> addToRecentlyPlayed(SongInfo songInfo) async {
    try {
      logger.i("Adding song to recently played ${songInfo.id}");

      await ensureRecentlyPlayedConnected();

      final box = Hive.box<RecentlyPlayed>(TableName.recentlyPlayedTable);
      final String key = songInfo.id;

      // 1️⃣ If already exists → update timestamp
      if (box.containsKey(key)) {
        final existing = box.get(key);
        if (existing != null) {
          await box.put(key, existing.copyWith(lastPlayedAt: DateTime.now()));
        }
        return;
      }

      // 2️⃣ If limit exceeded → remove oldest
      if (box.length >= Constants.recentlyPlayedLimit) {
        // Find the oldest entry
        final oldestEntry = box.toMap().entries.reduce((a, b) {
          return a.value.lastPlayedAt.isBefore(b.value.lastPlayedAt) ? a : b;
        });

        await box.delete(oldestEntry.key);
      }

      // 3️⃣ Insert new song
      await box.put(
        key,
        RecentlyPlayed(song: songInfo, lastPlayedAt: DateTime.now()),
      );
      logger.i("Song added to recently played");
    } catch (e, stack) {
      logger.e("Error adding to recently played", error: e, stackTrace: stack);
    }
  }

  @override
  Future<List<SongInfo>> getRecentlyPlayed() async {
    try {
      await ensureRecentlyPlayedConnected();
      final box = Hive.box<RecentlyPlayed>(TableName.recentlyPlayedTable);
      final recentSongs = box.values.toList();
      recentSongs.sort((a, b) => b.lastPlayedAt.compareTo(a.lastPlayedAt));

      return recentSongs.map((e) => e.song).toList();
    } catch (e, stack) {
      logger.e("Error getting recently played", error: e, stackTrace: stack);
      return [];
    }
  }

  @override
  Future<List<PlaylistInfo>> getPlaylist() async {
    try {
      await ensurePlaylistConnected();
      final box = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final playlists = box.values.toList();
      playlists.sort((a, b) => b.createdAt.compareTo(a.createdAt));
      return playlists;
    } catch (e, stack) {
      logger.e("Error getting playlist", error: e, stackTrace: stack);
      return [];
    }
  }

  @override
  Future<bool> createPlaylist(String playlistName) async {
    await ensurePlaylistConnected();
    try {
      if (playlistName.trim().isEmpty) {
        return false;
      }
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      if (playlistBox.values.length >= Constants.playlistLimit) {
        return false;
      }

      final createdPlaylist = PlaylistInfo(
        createdAt: DateTime.now(),
        id: uuid.v4(),
        name: playlistName.trim(),
        songIds: [],
        updatedAt: DateTime.now(),
        coverImagePath: null,
      );

      await playlistBox.put(createdPlaylist.id, createdPlaylist);

      logger.i("Playlist created: $playlistName");
      return true;
    } catch (_) {
      logger.e("Error creating playlist");
      return false;
    }
  }

  @override
  Future<bool> deletePlaylist(String playlistId) async {
    try {
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      final playlist = playlistBox.get(playlistId);
      if (playlist == null) {
        logger.w("Playlist not found: $playlistId");
        return false;
      }

      // 1️⃣ Delete playlist
      await playlistBox.delete(playlistId);

      // 2️⃣ Update only affected songs
      for (final songId in playlist.songIds) {
        final song = songsBox.get(songId);
        if (song == null) continue;

        final updatedSong = song.copyWith(
          playlistIds:
              song.playlistIds.where((id) => id != playlistId).toList(),
        );

        await songsBox.put(songId, updatedSong);
      }

      logger.i("Playlist deleted & songs updated: $playlistId");
      return true;
    } catch (e, stack) {
      logger.e("Error deleting playlist: $e", stackTrace: stack);
      return false;
    }
  }

  @override
  Future<List<SongInfo>> addToPlaylist(
    List<String> playlistIds,
    List<String> songIds,
  ) async {
    try {
      await ensurePlaylistConnected();
      await ensureSongsConnected();

      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final songInfoBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      final updatedSongs = <SongInfo>[];

      // Process each song
      for (final songId in songIds) {
        final song = songInfoBox.get(songId);

        if (song != null) {
          final updatedPlaylistIds = List<String>.from(song.playlistIds);

          // Process each playlist for this song
          for (final playlistId in playlistIds) {
            final playlist = playlistBox.get(playlistId);

            if (playlist != null) {
              if (playlist.songIds.length >= Constants.maxSongsPerPlaylist) {
                // Skip this playlist as it's full
                continue;
              }
              // Check if song is already in this playlist
              if (!playlist.songIds.contains(songId)) {
                // Add song to playlist
                final updatedPlaylist =
                    song.artworkPath != null && song.artworkPath!.isNotEmpty
                        ? playlist.copyWith(
                          songIds: [...playlist.songIds, songId],
                          coverImagePath: song.artworkPath,
                        )
                        : playlist.copyWith(
                          songIds: [...playlist.songIds, songId],
                          coverImagePath: playlist.coverImagePath,
                        );
                await playlistBox.put(playlistId, updatedPlaylist);
              }

              // Add playlist to song's list (if not already there)
              if (!updatedPlaylistIds.contains(playlistId)) {
                updatedPlaylistIds.add(playlistId);
              }
            }
          }

          // Update the song with all playlist changes
          final updatedSong = song.copyWith(playlistIds: updatedPlaylistIds);
          await songInfoBox.put(songId, updatedSong);
          updatedSongs.add(updatedSong);
        }
      }

      return updatedSongs;
    } catch (_) {
      return [];
    }
  }

  @override
  Future<List<SongInfo>> getPlaylistSongs(String playlistId) async {
    try {
      await ensurePlaylistConnected();
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);

      final playlist = playlistBox.get(playlistId);

      if (playlist == null) {
        logger.w("Playlist not found: $playlistId");
        return [];
      }

      final songInfoBox = Hive.box<SongInfo>(TableName.songsInfoTable);
      return playlist.songIds.map((id) => songInfoBox.get(id)!).toList();
    } catch (e, stack) {
      logger.e("Error getting playlist songs", error: e, stackTrace: stack);
      return [];
    }
  }

  @override
  Future<bool> updatePlaylistSongs(
    String playlistId,
    List<String> orderedSongIds,
    List<String> deletedSongIds,
  ) async {
    try {
      await ensurePlaylistConnected();

      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final songInfoBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      final playlist = playlistBox.get(playlistId);
      if (playlist == null) {
        logger.w("Playlist not found: $playlistId");
        return false;
      }

      String? updatedCoverImagePath;

      // 🔁 Handle deleted songs
      for (final songId in deletedSongIds) {
        final song = songInfoBox.get(songId);
        if (song == null) continue;

        // // 🖼️ If deleted song was cover, mark for replacement
        // if (song.artworkPath != null &&
        //     song.artworkPath == updatedCoverImagePath) {
        //   updatedCoverImagePath = null;
        // }

        final updatedSong = song.copyWith(
          playlistIds:
              song.playlistIds.where((id) => id != playlistId).toList(),
        );

        await songInfoBox.put(songId, updatedSong);
      }

      // 🔍 Pick new cover from remaining songs (if needed)
      for (final songId in orderedSongIds) {
        final song = songInfoBox.get(songId);
        if (song?.artworkPath != null) {
          updatedCoverImagePath = song!.artworkPath;
          break;
        }
      }

      // ✅ Final playlist update
      final updatedPlaylist = playlist.copyWith(
        songIds: orderedSongIds,
        coverImagePath: updatedCoverImagePath,
      );

      await playlistBox.put(playlistId, updatedPlaylist);

      return true;
    } catch (e, stack) {
      logger.e("Error updating playlist songs", error: e, stackTrace: stack);
      return false;
    }
  }

  @override
  Future<SearchResults> searchSongs(String query) async {
    try {
      await ensureSongsConnected();
      await ensurePlaylistConnected();

      final songBox = Hive.box<SongInfo>(TableName.songsInfoTable);
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);

      final lowercaseQuery = query.toLowerCase().trim();

      // Search in songs
      final matchingSongs =
          songBox.values.where((song) {
            return song.title.toLowerCase().contains(lowercaseQuery) ||
                song.artist.toLowerCase().contains(lowercaseQuery);
            // (song.album?.toLowerCase().contains(lowercaseQuery) ?? false);
          }).toList();

      // Search in playlists
      final matchingPlaylists =
          playlistBox.values.where((playlist) {
            return playlist.name.toLowerCase().contains(lowercaseQuery);
          }).toList();

      // Format the results
      final List<SongInfo> songResults = matchingSongs;
      final List<PlaylistInfo> playlistResults = matchingPlaylists;

      return SearchResults(
        query: query,
        songs: songResults,
        playlists: playlistResults,
      );
    } catch (e) {
      print('Error searching: $e');
      return SearchResults(query: query, songs: [], playlists: []);
    }
  }
}
