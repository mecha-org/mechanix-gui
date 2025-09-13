import 'dart:async';
import 'dart:typed_data';
import 'package:music_player/src/features/player/models/types.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/adapters.dart';
import 'package:media_kit/media_kit.dart';
import '../data/music_player_manager.dart';
import 'mini_player.dart';

class MusicPlayerPage extends StatefulWidget {
  const MusicPlayerPage({super.key});
  @override
  State<MusicPlayerPage> createState() => _MusicPlayerPageState();
}

class _MusicPlayerPageState extends State<MusicPlayerPage> {
  final manager = MusicPlayerManager.instance;

  Duration _position = Duration.zero;
  Duration _duration = Duration.zero;
  bool _isPlaying = false;
  int _currentIndex = 0;
  bool _isScanning = false;
  SongInfo? _currentSong;

  String _searchQuery = "";
  final TextEditingController _searchController = TextEditingController();

  late final StreamSubscription<bool> _playingSub;
  late final StreamSubscription<Duration> _posSub;
  late final StreamSubscription<Duration> _durSub;
  late final StreamSubscription<Playlist> _playlistSub;

  @override
  void initState() {
    super.initState();
    final player = manager.player;

    _playingSub = player.stream.playing.listen(
      (p) => setState(() => _isPlaying = p),
    );
    _posSub = player.stream.position.listen(
      (pos) => setState(() => _position = pos),
    );
    _durSub = player.stream.duration.listen(
      (dur) => setState(() => _duration = dur),
    );
    _playlistSub = player.stream.playlist.listen((pl) {
      final idx = pl.index;
      if (idx >= 0 && idx < manager.playbackQueue.length) {
        setState(() {
          _currentIndex = idx;
          _currentSong = manager.playbackQueue[idx];
        });
      }
    });

    // Initialize current song/index
    if (manager.playbackQueue.isNotEmpty) {
      _currentIndex = manager.currentIndex >= 0 ? manager.currentIndex : 0;
      _currentSong = manager.playbackQueue[_currentIndex];
      _isPlaying = false; // default until first playing event
      _position = Duration.zero; // default until first position event
      _duration = Duration.zero; // default until first duration event
    }
  }

  @override
  void dispose() {
    _playingSub.cancel();
    _posSub.cancel();
    _durSub.cancel();
    _playlistSub.cancel();
    super.dispose();
  }

  Future<void> _scanFiles() async {
    setState(() => _isScanning = true);

    await manager.scanAndSaveSongs();

    final songs = manager.allSongs;
    if (songs.isNotEmpty) {
      await manager.player.open(
        Playlist(songs.map((s) => Media(s)).toList(), index: 0),
        play: false,
      );

      // Set current song to first one
      setState(() {
        _currentIndex = 0;
        _currentSong =
            manager.playbackQueue.isNotEmpty ? manager.playbackQueue[0] : null;
      });
    }

    setState(() => _isScanning = false);
  }

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 2, // Songs + Albums
      child: Scaffold(
        appBar: AppBar(
          backgroundColor: Theme.of(context).primaryColor,
          title:
              _searchQuery.isEmpty
                  ? const Text("Music")
                  : TextField(
                    controller: _searchController,
                    autofocus: true,
                    style: const TextStyle(color: Colors.white),
                    cursorColor: Colors.white,
                    decoration: InputDecoration(
                      contentPadding: const EdgeInsets.symmetric(
                        vertical: 12.0,
                        horizontal: 16.0,
                      ),
                      filled: true,
                      fillColor: Colors.white.withOpacity(
                        0.1,
                      ), // light background
                      hintText: "Search...",
                      hintStyle: const TextStyle(color: Colors.white54),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(20.0),
                        borderSide: BorderSide.none, // no border
                      ),
                      prefixIcon: const Icon(Icons.search, color: Colors.white),
                      suffixIcon: IconButton(
                        icon: const Icon(Icons.close, color: Colors.white),
                        onPressed: () {
                          setState(() {
                            _searchQuery = "";
                            _searchController.clear();
                          });
                        },
                      ),
                    ),
                    onChanged: (val) {
                      setState(() => _searchQuery = val.toLowerCase());
                    },
                  ),

          actions:
              _searchQuery.isEmpty
                  ? [
                    IconButton(
                      icon: const Icon(Icons.search),
                      onPressed: () {
                        setState(
                          () => _searchQuery = " ",
                        ); // trigger search mode
                      },
                    ),
                    _isScanning
                        ? const Padding(
                          padding: EdgeInsets.symmetric(horizontal: 16.0),
                          child: SizedBox(
                            width: 20,
                            height: 20,
                            child: CircularProgressIndicator(
                              strokeWidth: 2,
                              valueColor: AlwaysStoppedAnimation<Color>(
                                Colors.white,
                              ),
                            ),
                          ),
                        )
                        : IconButton(
                          icon: const Icon(Icons.refresh),
                          onPressed: _scanFiles,
                        ),
                  ]
                  : [],
          bottom: TabBar(
            tabs: const [
              Tab(text: "Songs", icon: Icon(Icons.music_note)),
              Tab(text: "Albums", icon: Icon(Icons.album)),
            ],
          ),
        ),

        body: TabBarView(children: [_buildSongsTab(), _buildAlbumsTab()]),
      ),
    );
  }

  Widget _buildSongsTab() {
    return Column(
      children: [
        Expanded(
          child: ValueListenableBuilder(
            valueListenable: Hive.box<List>('playlistBox').listenable(),
            builder: (context, box, _) {
              final infos =
                  manager.songInfos
                      .where(
                        (s) =>
                            s.title.toLowerCase().contains(_searchQuery) ||
                            s.artist.toLowerCase().contains(_searchQuery) ||
                            (s.album ?? "").toLowerCase().contains(
                              _searchQuery,
                            ),
                      )
                      .toList();

              return ListView.builder(
                itemCount: infos.length,
                itemBuilder: (context, i) {
                  final song = infos[i];
                  final selected =
                      _currentSong != null && song.path == _currentSong!.path;

                  return ListTile(
                    leading: buildArtworkOrIcon(
                      artwork: song.artwork,
                      fallbackIcon: Icons.music_note,
                      isSelected: selected,
                    ),
                    title: Text(
                      song.title,
                      style: TextStyle(
                        fontWeight:
                            selected ? FontWeight.bold : FontWeight.normal,
                        color:
                            selected
                                ? Theme.of(context).secondaryHeaderColor
                                : null,
                      ),
                    ),
                    subtitle: Text(
                      "${song.artist}${song.album != null && song.album!.isNotEmpty ? ' | ${song.album}' : ''}",
                    ),
                    onTap: () async {
                      if (_searchQuery.isEmpty) {
                        // normal mode → play from full list
                        final fullIndex = manager.songInfos.indexWhere(
                          (s) => s.path == song.path,
                        );
                        if (fullIndex != -1) {
                          await manager.playAllSongs(fullIndex);
                        }
                      } else {
                        // search active → play from filtered list
                        await manager.playFilteredSongs(infos, i);
                      }
                    },
                    trailing:
                        selected
                            ? Icon(
                              Icons.bar_chart_rounded,
                              color: Theme.of(context).secondaryHeaderColor,
                            )
                            : null,
                  );
                },
              );
            },
          ),
        ),
        MiniPlayer(
          isPlaying: _isPlaying,
          position: _position,
          duration: _duration,
          currentIndex: _currentIndex,
        ),
      ],
    );
  }

  Widget _buildAlbumsTab() {
    final infos =
        manager.songInfos
            .where(
              (s) =>
                  s.title.toLowerCase().contains(_searchQuery) ||
                  s.artist.toLowerCase().contains(_searchQuery) ||
                  (s.album ?? "").toLowerCase().contains(_searchQuery),
            )
            .toList();

    // Group by album
    final albums = <String, List<SongInfo>>{};
    for (final song in infos) {
      final albumName =
          song.album?.isNotEmpty == true ? song.album! : "Unknown Album";
      albums.putIfAbsent(albumName, () => []).add(song);
    }

    if (albums.isEmpty) {
      return const Center(child: Text("No albums found"));
    }

    return Column(
      children: [
        Expanded(
          child: ListView(
            children:
                albums.entries.map((entry) {
                  final albumName = entry.key;
                  final albumSongs = entry.value;
                  final isCurrentAlbum = albumSongs.contains(_currentSong);

                  return ExpansionTile(
                    leading: buildArtworkOrIcon(
                      artwork: albumSongs.first.artwork,
                      fallbackIcon: Icons.album,
                      isSelected: isCurrentAlbum,
                    ),
                    title: Text(
                      albumName,
                      style: TextStyle(
                        fontWeight: FontWeight.bold,
                        color:
                            isCurrentAlbum
                                ? Theme.of(context).secondaryHeaderColor
                                : null,
                      ),
                    ),
                    subtitle: Text("${albumSongs.length} song(s)"),
                    children:
                        albumSongs.map((song) {
                          final selected =
                              _currentSong != null &&
                              song.path == _currentSong!.path;

                          return ListTile(
                            leading: buildArtworkOrIcon(
                              artwork: null,
                              fallbackIcon: Icons.music_note,
                              isSelected: selected,
                              size: 30,
                              radius: 6,
                            ),
                            title: Text(
                              song.title,
                              style: TextStyle(
                                fontWeight:
                                    selected
                                        ? FontWeight.bold
                                        : FontWeight.normal,
                                color:
                                    selected
                                        ? Theme.of(context).secondaryHeaderColor
                                        : null,
                              ),
                            ),
                            subtitle: Text(song.artist),
                            onTap: () {
                              final startIndex = albumSongs.indexOf(song);
                              if (startIndex != -1) {
                                manager.playAlbum(albumSongs, startIndex);
                              }
                            },
                            trailing:
                                selected
                                    ? Icon(
                                      Icons.bar_chart_rounded,
                                      color:
                                          Theme.of(
                                            context,
                                          ).secondaryHeaderColor,
                                    )
                                    : null,
                          );
                        }).toList(),
                  );
                }).toList(),
          ),
        ),

        MiniPlayer(
          isPlaying: _isPlaying,
          position: _position,
          duration: _duration,
          currentIndex: _currentIndex,
        ),
      ],
    );
  }

  Widget buildArtworkOrIcon({
    required Uint8List? artwork,
    required IconData fallbackIcon,
    required bool isSelected,
    double size = 40,
    double radius = 8,
  }) {
    if (artwork != null) {
      return ClipRRect(
        borderRadius: BorderRadius.circular(radius),
        child: Image.memory(
          artwork,
          width: size,
          height: size,
          fit: BoxFit.cover,
        ),
      );
    } else {
      return Container(
        width: size,
        height: size,
        decoration: BoxDecoration(
          color: Colors.grey.shade200,
          borderRadius: BorderRadius.circular(radius),
          border: Border.all(
            color:
                isSelected
                    ? Theme.of(context).secondaryHeaderColor
                    : Colors.grey,
            width: 1.5,
          ),
        ),
        child: Icon(
          fallbackIcon,
          color:
              isSelected
                  ? Theme.of(context).secondaryHeaderColor
                  : Colors.grey.shade600,
        ),
      );
    }
  }
}
