import 'dart:io';
import 'package:audio_metadata_extractor/audio_metadata_extractor.dart';
import 'package:audio_metadata_reader/audio_metadata_reader.dart'
    hide AudioMetadata;
import 'package:hive/hive.dart';
import 'package:logger/web.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/recently_played.dart';
import 'package:mechanix_music/models/search_data.dart';
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

  Future<void> ensureSearchHistoryConnected() async {
    if (!Hive.isBoxOpen(TableName.searchTable)) {
      await Hive.openBox<SearchData>(TableName.searchTable);
    }
  }

  @override
  Future<List<SongInfo>> scanAllSongs() async {
    logger.i("Scanning all songs");

    try {
      logger.i("Songs scanning started");

      // Get cache directory for artwork
      final cacheDir = await _getArtworkCacheDirectory();

      await ensureSongsConnected();
      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      // Create a map of existing songs by path for quick lookup
      final existingSongsByPath = <String, SongInfo>{};
      for (final song in songsBox.values) {
        existingSongsByPath[song.path] = song;
      }

      final home = Directory(Constants.musicDir);

      final files =
          await home
              .list(recursive: false, followLinks: false)
              .where((entity) {
                final path = entity.path;
                if (path.split('/').any((p) => p.startsWith('.'))) return false;
                return entity is File &&
                    audioExt.any((ext) => path.toLowerCase().endsWith(ext));
              })
              .cast<File>()
              .toList();

      // Track which paths were found in the scan
      final scannedPaths = <String>{};
      final seen = <String>{};
      logger.i("Total audio files found: ${files.length}");

      final tempInfos = <SongInfo>[];

      for (int index = 0; index < files.length; index++) {
        final file = files[index];
        scannedPaths.add(file.path);

        try {
          final stat = await file.stat();
          final key = "${file.uri.pathSegments.last}_${stat.size}";
          if (!seen.add(key)) continue;

          final existingSong = existingSongsByPath[file.path];

          // Check if song exists and file hasn't been modified
          if (existingSong != null) {
            // File exists in database, just update index and keep existing data
            final updatedSong = existingSong.copyWith(index: index);
            await songsBox.put(updatedSong.id, updatedSong);
            tempInfos.add(updatedSong);
            logger.d("Updated existing song: ${file.path}");
            continue;
          }

          // New song - extract metadata
          // final albumArtMetadata = await AudioMetadata.extract(file);
          final metadata = await readMetadata(file);
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

          final newSong = SongInfo(
            index: index,
            id: uuid.v4(),
            path: file.path,
            title: albumArtMetadata?.trackName ?? file.uri.pathSegments.last,
            artist: albumArtMetadata?.firstArtists ?? 'Unknown Artist',
            album: albumArtMetadata?.album,
            duration: metadata.duration?.inSeconds.toString(),
            artworkPath: artworkCachePath,
          );

          await songsBox.put(newSong.id, newSong);
          tempInfos.add(newSong);
          logger.d("Added new song: ${file.path}");
        } catch (e) {
          logger.w("Error processing file ${file.path}: $e");

          final existingSong = existingSongsByPath[file.path];
          if (existingSong != null) {
            // Keep existing song data even if processing fails
            final updatedSong = existingSong.copyWith(index: index);
            await songsBox.put(updatedSong.id, updatedSong);
            tempInfos.add(updatedSong);
          } else {
            // Create minimal song info for new files that failed processing
            final newSong = SongInfo(
              index: index,
              id: uuid.v4(),
              path: file.path,
              title: file.uri.pathSegments.last,
              artist: 'Unknown Artist',
            );
            await songsBox.put(newSong.id, newSong);
            tempInfos.add(newSong);
          }
        }
      }

      // Remove songs that no longer exist on disk
      final songsToRemove = <String>[];
      for (final song in songsBox.values) {
        if (!scannedPaths.contains(song.path)) {
          songsToRemove.add(song.id);

          // Delete artwork cache if exists
          if (song.artworkPath != null) {
            try {
              final artworkFile = File(song.artworkPath!);
              if (await artworkFile.exists()) {
                await artworkFile.delete();
                logger.d("Deleted artwork cache: ${song.artworkPath}");
              }
            } catch (e) {
              logger.w("Error deleting artwork cache: $e");
            }
          }
        }
      }

      // Remove songs from box
      for (final songId in songsToRemove) {
        await songsBox.delete(songId);
        logger.d("Removed song with id: $songId");
      }

      // Remove songs from playlists
      if (songsToRemove.isNotEmpty) {
        await _removeSongsFromPlaylists(songsToRemove);
        await _removeFromRecents(songsToRemove);
      }

      logger.i(
        "Songs scanning completed. Added/Updated: ${tempInfos.length}, Removed: ${songsToRemove.length}",
      );
      return tempInfos;
    } catch (e) {
      logger.e("Error scanning songs: $e");
      return [];
    }
  }

  Future<void> _removeFromRecents(List<String> songIds) async {
    try {
      await ensureRecentlyPlayedConnected();

      final recentSongs = Hive.box<RecentlyPlayed>(
        TableName.recentlyPlayedTable,
      );

      final keysToRemove = <dynamic>[];

      // Find all recent songs that contain the deleted song IDs
      for (final entry in recentSongs.toMap().entries) {
        if (songIds.contains(entry.value.song.id)) {
          keysToRemove.add(entry.key);
        }
      }

      // Remove the found entries
      for (final key in keysToRemove) {
        await recentSongs.delete(key);
        logger.d("Removed song from recent songs with key: $key");
      }

      logger.i("Removed ${keysToRemove.length} songs from recently played");
    } catch (e) {
      logger.w("Error removing songs from recently played: $e");
    }
  }

  // Helper method to remove songs from all playlists
  Future<void> _removeSongsFromPlaylists(List<String> songIds) async {
    try {
      // Assuming you have a playlists box
      final playlistsBox = Hive.box<PlaylistInfo>(
        TableName.playlistTable,
      ); // Adjust according to your playlist model

      for (final playlist in playlistsBox.values) {
        bool modified = false;
        final updatedSongIds =
            playlist.songIds.where((id) {
              if (songIds.contains(id)) {
                modified = true;
                return false;
              }
              return true;
            }).toList();

        if (modified) {
          final updatedPlaylist = playlist.copyWith(
            songIds: updatedSongIds,
            coverImagePath: playlist.coverImagePath,
          );
          await playlistsBox.put(playlist.id, updatedPlaylist);
          logger.d("Removed deleted songs from playlist: ${playlist.name}");
        }
      }
    } catch (e) {
      logger.w("Error removing songs from playlists: $e");
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
  Future<bool> toggleFavouriteSong(
    List<String> songIds,
    bool isFavourite,
  ) async {
    try {
      await ensureSongsConnected();
      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      for (var i = 0; i < songIds.length; i++) {
        final song = songsBox.get(songIds[i]);
        if (song != null) {
          final updatedSong = song.copyWith(isFavourite: isFavourite);

          await songsBox.put(updatedSong.id, updatedSong);
        }
      }

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
  Future<bool> createUpdatePlaylist(
    String playlistName,
    String? playlistId,
  ) async {
    await ensurePlaylistConnected();
    try {
      if (playlistName.trim().isEmpty) {
        return false;
      }
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      if (playlistId != null) {
        final playlist = playlistBox.get(playlistId);
        if (playlist == null) {
          logger.w("Playlist not found: $playlistId");
          return false;
        }
        final updatedPlaylist = playlist.copyWith(
          name: playlistName.trim(),
          updatedAt: DateTime.now(),
          coverImagePath: playlist.coverImagePath,
        );
        await playlistBox.put(playlistId, updatedPlaylist);
        logger.i("Playlist updated: $playlistName");
        return true;
      }

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
        isShuffle: false,
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

      for (final songId in songIds) {
        final song = songInfoBox.get(songId);
        if (song == null) continue;

        final updatedPlaylistIds = List<String>.from(song.playlistIds);

        for (final playlistId in playlistIds) {
          final playlist = playlistBox.get(playlistId);
          if (playlist == null) continue;

          if (playlist.songIds.length >= Constants.maxSongsPerPlaylist) {
            continue;
          }

          // Skip if already exists
          if (playlist.songIds.contains(songId)) {
            if (!updatedPlaylistIds.contains(playlistId)) {
              updatedPlaylistIds.add(playlistId);
            }
            continue;
          }

          //  Updated songIds for playlist
          final updatedSongIds = [...playlist.songIds, songId];

          //  Recalculate isLiked for playlist
          final isPlaylistLiked = updatedSongIds.every((id) {
            final s = songInfoBox.get(id);
            return s?.isFavourite == true;
          });

          final updatedPlaylist = playlist.copyWith(
            songIds: updatedSongIds,
            coverImagePath: song.artworkPath ?? playlist.coverImagePath,
            isLiked: isPlaylistLiked,
          );

          await playlistBox.put(playlistId, updatedPlaylist);

          if (!updatedPlaylistIds.contains(playlistId)) {
            updatedPlaylistIds.add(playlistId);
          }
        }

        final updatedSong = song.copyWith(playlistIds: updatedPlaylistIds);

        await songInfoBox.put(songId, updatedSong);
        updatedSongs.add(updatedSong);
      }

      return updatedSongs;
    } catch (e) {
      return [];
    }
  }

  @override
  Future<List<SongInfo>> getPlaylistSongs(String playlistId) async {
    try {
      await ensurePlaylistConnected();
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final songInfoBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      final playlist = playlistBox.get(playlistId);
      if (playlist == null) {
        logger.w("Playlist not found: $playlistId");
        return [];
      }

      return playlist.songIds
          .map((id) => songInfoBox.get(id))
          .whereType<SongInfo>() // removes nulls safely
          .toList();
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

  @override
  Future<bool> shuffleToggle(String playlistId, bool isShuffle) async {
    try {
      logger.i("Shuffle toggle: $isShuffle");

      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final playlist = playlistBox.get(playlistId);

      if (playlist == null) {
        logger.w("Playlist not found: $playlistId");
        return false;
      }

      final copyData = playlist.copyWith(
        isShuffle: isShuffle,
        coverImagePath: playlist.coverImagePath,
      );
      await playlistBox.put(
        playlistId,
        // playlist.copyWith(isShuffle: isShuffle),
        copyData,
      );
      return true;
    } catch (_) {
      return false;
    }
  }

  @override
  Future<bool> storeSearchItem({
    PlaylistInfo? playlistInfo,
    SongInfo? songInfo,
  }) async {
    try {
      logger.i("Storing search item");
      await ensureSearchHistoryConnected();
      final box = Hive.box<SearchData>(TableName.searchTable);

      if (playlistInfo == null && songInfo == null) {
        return false;
      }

      // 1️⃣ Check for existing item (dedupe)
      SearchData? existingItem;

      for (final item in box.values) {
        if (playlistInfo != null &&
            item.isPlaylist &&
            item.playlistId == playlistInfo.id) {
          existingItem = item;
          break;
        }

        if (songInfo != null &&
            !item.isPlaylist &&
            item.songId == songInfo.id) {
          existingItem = item;
          break;
        }
      }

      // 2️⃣ If exists → update timestamp only
      if (existingItem != null) {
        existingItem = existingItem.copyWith(createdAt: DateTime.now());
        await box.put(existingItem.id, existingItem);
        return true;
      }

      // 3️⃣ Enforce size limit (remove oldest)
      if (box.length >= Constants.maxSearchItems) {
        final oldest = box.values.reduce(
          (a, b) => a.createdAt.isBefore(b.createdAt) ? a : b,
        );

        await oldest.delete();
      }

      // 4️⃣ Insert new item
      final data = SearchData(
        id: uuid.v4(),
        createdAt: DateTime.now(),
        isPlaylist: playlistInfo != null,
        playlistId: playlistInfo?.id,
        songId: songInfo?.id,
      );

      await box.put(data.id, data);

      return true;
    } catch (e, stack) {
      logger.e("Error storing search item", error: e, stackTrace: stack);
      return false;
    }
  }

  @override
  Future<List<SearchInfo>> getStoredSearchItems() async {
    try {
      logger.i("Getting stored search items");

      await ensureSearchHistoryConnected();
      await ensurePlaylistConnected();
      await ensureSongsConnected();

      final searchBox = Hive.box<SearchData>(TableName.searchTable);
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      final List<SearchInfo> results = [];

      for (final searchItem in searchBox.values) {
        if (searchItem.isPlaylist) {
          // Fetch latest playlist
          final playlist = playlistBox.get(searchItem.playlistId);

          if (playlist != null) {
            results.add(
              SearchInfo(
                id: searchItem.id,
                isPlaylist: true,
                playlistInfo: playlist,
                createdAt: searchItem.createdAt,
              ),
            );
          }
        } else {
          // 🔹 Fetch latest song
          final song = songsBox.get(searchItem.songId);

          if (song != null) {
            results.add(
              SearchInfo(
                id: searchItem.id,
                isPlaylist: false,
                songInfo: song,
                createdAt: searchItem.createdAt,
              ),
            );
          }
        }
      }

      // 🔹 Sort by recent searches
      results.sort((a, b) => b.createdAt.compareTo(a.createdAt));

      return results;
    } catch (e, stack) {
      logger.e(
        "Error getting stored search items",
        error: e,
        stackTrace: stack,
      );
      return [];
    }
  }

  @override
  Future<bool> clearSearchItems({
    String? searchId,
    required bool clearAll,
  }) async {
    try {
      logger.i("Clearing search history");
      await ensureSearchHistoryConnected();
      final box = Hive.box<SearchData>(TableName.searchTable);
      if (searchId != null) {
        await box.delete(searchId);
      } else {
        await box.clear();
      }
      return true;
    } catch (e, stack) {
      logger.e("Error clearing search history", error: e, stackTrace: stack);
      return false;
    }
  }

  @override
  Future<PlaylistInfo?> getSelectedPlaylist({
    required String playlistId,
  }) async {
    try {
      await ensurePlaylistConnected();
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final playlist = playlistBox.get(playlistId);
      if (playlist == null) {
        logger.w("Playlist not found: $playlistId");
        return null;
      }
      return playlist;
    } catch (e, stack) {
      logger.e("Error getting playlist", error: e, stackTrace: stack);
      return null;
    }
  }

  @override
  Future<List<PlaylistInfo>> searchedPlaylist({required String query}) async {
    try {
      logger.i("Searching playlist: $query");
      if (query.trim().isNotEmpty) {
        final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
        final searchedPlaylists =
            playlistBox.values.where((playlist) {
              return playlist.name.toLowerCase().contains(query.toLowerCase());
            }).toList();
        return searchedPlaylists.toList();
      }
      return [];
    } catch (e) {
      return [];
    }
  }

  @override
  Future<List<SongInfo>> searchedSong({required String query}) async {
    try {
      logger.i("Searching song: $query");
      if (query.trim().isNotEmpty) {
        final songBox = Hive.box<SongInfo>(TableName.songsInfoTable);
        final searchedSongs =
            songBox.values.where((song) {
              // return song.title.toLowerCase().contains(query.toLowerCase());
              return song.title.toLowerCase().contains(query.toLowerCase()) ||
                  song.artist.toLowerCase().contains(query.toLowerCase());
            }).toList();
        return searchedSongs.toList();
      }
      return [];
    } catch (e) {
      return [];
    }
  }

  @override
  Future<List<SongInfo>> getFavouriteSongs() async {
    try {
      logger.i("Getting favourite songs");
      await ensureSongsConnected();
      final songBox = Hive.box<SongInfo>(TableName.songsInfoTable);
      final favouriteSongs =
          songBox.values.where((song) {
            return song.isFavourite;
          }).toList();
      return favouriteSongs;
    } catch (e, stack) {
      logger.e("Error getting favourite songs", error: e, stackTrace: stack);
      return [];
    }
  }

  @override
  Future<bool> removeSongByPath(String path) async {
    try {
      await ensureSongsConnected();
      await ensurePlaylistConnected();

      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      // Find song by path
      if (songsBox.values.isEmpty) return false;

      final song = songsBox.values.firstWhere((s) => s.path == path);

      final songId = song.id;

      // Remove song from all playlists
      if (song.playlistIds.isNotEmpty) {
        for (final playlistId in song.playlistIds) {
          await _removeSongFromPlaylist(playlistId, songId);
        }
      }

      // Delete artwork file if exists
      if (song.artworkPath?.isNotEmpty == true) {
        try {
          final artworkFile = File(song.artworkPath!);
          if (await artworkFile.exists()) {
            await artworkFile.delete();
            logger.i('Deleted artwork: ${song.artworkPath}');
          }
        } catch (e) {
          logger.i('Warning: Failed to delete artwork file: $e');
        }
      }

      // Delete by ID (Hive key)
      await songsBox.delete(songId);
      await _removeFromRecents([songId]);

      logger.i('Song removed: ${song.title} by ${song.artist}');
      return true;
    } catch (e, stackTrace) {
      logger.i('Error removing song from path: $e');
      logger.i('Stack trace: $stackTrace');
      return false;
    }
  }

  /// Remove song from a specific playlist and update playlist artwork
  Future<void> _removeSongFromPlaylist(String playlistId, String songId) async {
    try {
      final playlistsBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final songsBox = Hive.box<SongInfo>(TableName.songsInfoTable);

      final playlist = playlistsBox.get(playlistId);
      if (playlist == null) return;

      // Remove song from playlist's song list
      final updatedSongIds = List<String>.from(playlist.songIds)
        ..remove(songId);

      // Update playlist artwork (use first song's artwork or null)
      String? newArtworkPath;
      if (updatedSongIds.isNotEmpty) {
        // Find first song with artwork
        for (final sid in updatedSongIds) {
          final song = songsBox.values.firstWhere((s) => s.id == sid);
          if (song.artworkPath != null && song.artworkPath!.isNotEmpty) {
            newArtworkPath = song.artworkPath;
            break;
          }
        }
      }

      // Delete old playlist artwork if it's different from the new one
      if (playlist.coverImagePath != null &&
          playlist.coverImagePath != newArtworkPath &&
          playlist.coverImagePath!.isNotEmpty) {
        try {
          final oldArtworkFile = File(playlist.coverImagePath!);
          if (await oldArtworkFile.exists()) {
            await oldArtworkFile.delete();
            logger.i(
              'Deleted old playlist artwork: ${playlist.coverImagePath}',
            );
          }
        } catch (e) {
          logger.i('Warning: Failed to delete old playlist artwork: $e');
        }
      }

      // Create updated playlist
      final updatedPlaylist = PlaylistInfo(
        id: playlist.id,
        name: playlist.name,
        songIds: updatedSongIds,
        coverImagePath: newArtworkPath,
        createdAt: playlist.createdAt,
        updatedAt: DateTime.now(),
        isShuffle: playlist.isShuffle,
      );

      // Update playlist in Hive
      await playlistsBox.put(playlistId, updatedPlaylist);

      logger.i('Removed song from playlist: ${playlist.name}');
    } catch (e) {
      logger.i('Error removing song from playlist $playlistId: $e');
    }
  }

  @override
  Future<bool> addOrUpdateSongFromPath(String path) async {
    try {
      final file = File(path);
      logger.i("path: $path");

      // Check if file exists
      if (!await file.exists()) {
        logger.i('File does not exist: $path');
        return false;
      }

      // Open Hive box
      await ensureSongsConnected();
      final box = Hive.box<SongInfo>(TableName.songsInfoTable);
      // Find existing song by path
      final existingIndex = box.values.toList().indexWhere(
        (s) => s.path == path,
      );

      final existingSong =
          existingIndex != -1 ? box.getAt(existingIndex) : null;
      final isUpdate = existingSong != null;

      // Read metadata
      // final metadata = readMetadata(file);
      final albumArtMetadata = await AudioMetadata.extract(file);
      final cacheDir = await _getArtworkCacheDirectory();

      // Handle artwork
      String? artworkCachePath;

      if (albumArtMetadata?.coverData != null) {
        // Delete old artwork if updating
        if (isUpdate && existingSong.artworkPath != null) {
          final oldArtFile = File(existingSong.artworkPath!);
          if (await oldArtFile.exists()) {
            await oldArtFile.delete();
          }
        }

        // Save new artwork
        final artworkId = isUpdate ? existingSong.id : uuid.v4();
        artworkCachePath = await _saveArtworkToCache(
          albumArtMetadata!.coverData!,
          artworkId,
          cacheDir,
        );
      } else if (isUpdate) {
        // Preserve existing artwork if no new one
        artworkCachePath = existingSong.artworkPath;
      }

      // Create song object
      final song = SongInfo(
        index: 0,
        id: isUpdate ? existingSong.id : uuid.v4(),
        path: file.path,
        title: albumArtMetadata?.trackName ?? file.uri.pathSegments.last,
        artist: albumArtMetadata?.firstArtists ?? 'Unknown Artist',
        album: albumArtMetadata?.album,
        duration: albumArtMetadata?.duration.toString(),
        artworkPath: artworkCachePath,
      );

      // Add or update in Hive
      if (isUpdate) {
        await box.putAt(existingIndex, song);
        logger.i('Song updated: ${song.title} by ${song.artist}');
      } else {
        await box.put(song.id, song);
        logger.i('Song added: ${song.title} by ${song.artist}');
      }

      return true;
    } catch (e, stackTrace) {
      logger.e('Error processing song from path: $e');
      logger.e('Stack trace: $stackTrace');
      return false;
    }
  }

  @override
  Future<bool> addPlaylistToLiked(String playlistId, bool isLiked) async {
    try {
      final playlistBox = Hive.box<PlaylistInfo>(TableName.playlistTable);
      final playlist = playlistBox.get(playlistId);
      if (playlist == null) {
        logger.w("Playlist not found: $playlistId");
        return false;
      }
      final updatedPlaylist = playlist.copyWith(
        isLiked: isLiked,
        coverImagePath: playlist.coverImagePath,
      );
      for (final songId in playlist.songIds) {
        final song = Hive.box<SongInfo>(TableName.songsInfoTable).get(songId);
        if (song == null) continue;
        final updatedSong = song.copyWith(isFavourite: isLiked);
        await Hive.box<SongInfo>(
          TableName.songsInfoTable,
        ).put(songId, updatedSong);
      }

      await playlistBox.put(playlistId, updatedPlaylist);
      return true;
    } catch (e) {
      logger.e("Error adding playlist to liked: $e");
      return false;
    }
  }
}
