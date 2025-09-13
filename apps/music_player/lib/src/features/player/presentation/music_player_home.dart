import 'dart:async';
import 'dart:typed_data';
import 'package:music_player/app_routes.dart';
import 'package:music_player/src/commons/constants.dart';
import 'package:music_player/src/commons/widgets/row_container.dart';
import 'package:music_player/src/features/player/models/types.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/adapters.dart';
import 'package:media_kit/media_kit.dart';
import 'package:music_player/src/features/player/presentation/song_menu_optons.dart';
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

  final GlobalKey _selectedIconKey = GlobalKey();
  final GlobalKey _menuIconKey = GlobalKey();
  bool _isMenuOpen = false;
  String _sortOrder = "None";

  // --- Overlay Menu State Management ---
  OverlayEntry? _songMenuEntry;
  LayerLink? _activeLayerLink; // Track which item is currently active

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
    if (manager.playbackQueue.isNotEmpty) {
      _currentIndex = manager.currentIndex >= 0 ? manager.currentIndex : 0;
      _currentSong = manager.playbackQueue[_currentIndex];
    }
  }

  @override
  void dispose() {
    _playingSub.cancel();
    _posSub.cancel();
    _durSub.cancel();
    _playlistSub.cancel();
    _removeSongMenu();
    super.dispose();
  }

  // Overlay menu handler: always remove previous before showing new menu
  void _showSongMenu(BuildContext context, LayerLink link) {
    _removeSongMenu(); // Always clean up before displaying new menu
    _songMenuEntry = OverlayEntry(
      builder: (_) => MenuOptions(menuLink: link, entry: _songMenuEntry),
    );
    Overlay.of(context, rootOverlay: true).insert(_songMenuEntry!);
    _activeLayerLink = link;
  }

  void _removeSongMenu() {
    if (_songMenuEntry != null && _songMenuEntry!.mounted) {
      _songMenuEntry!.remove();
    }
    _songMenuEntry = null;
    _activeLayerLink = null;
  }

  @override
  Widget build(BuildContext context) {
    return
    // DefaultTabController(
    //   length: 2, // Songs + Albums
    //   child:
    Scaffold(
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
                  Image.asset(
                    Images.shuffle,
                    height: 28,
                    width: 28,
                    // color: _isMenuOpen ? Colors.grey : Colors.white, // Change color when open
                  ),
                  // Image.asset(
                  //   Images.sort,
                  //   height: 28,
                  //   width: 28,
                  //   // color: _isMenuOpen ? Colors.grey : Colors.white, // Change color when open
                  // ),
                  PopupMenuButton<String>(
                    tooltip: '',
                    icon: Image.asset(
                      Images.sort,
                      height: 28,
                      width: 28,
                      color: _isMenuOpen ? Colors.grey : Colors.white,
                    ),
                    onSelected: (value) {
                      // Handle sort order change
                      print("Selected sort order: $value");
                      setState(() {
                        _sortOrder = value;
                      });
                    },
                    position: PopupMenuPosition.under,
                    itemBuilder: (context) {
                      return [
                        PopupMenuItem(
                          value: "None",
                          child: RadioListTile<String>(
                            title: const Text("None"),
                            value: "None",
                            groupValue: "None",
                            onChanged: (val) {
                              setState(() => _sortOrder = val!);
                              Navigator.pop(context, val);
                            },
                          ),
                        ),
                        PopupMenuItem(
                          value: "Latest",
                          child: RadioListTile<String>(
                            title: const Text("Latest"),
                            value: "Latest",
                            groupValue: "Latest",
                            onChanged: (val) {
                              setState(() => _sortOrder = val!);
                              Navigator.pop(context, val);
                            },
                          ),
                        ),
                        PopupMenuItem(
                          value:
                              _sortOrder == "Ascending"
                                  ? "Ascending"
                                  : "Descending",
                          child: RadioListTile<String>(
                            title: Row(
                              children: [
                                const Text("Alphabetical"),
                                const SizedBox(width: 8),
                                Icon(
                                  _sortOrder == "Ascending"
                                      ? Icons.arrow_upward
                                      : Icons.arrow_downward,
                                  size: 16,
                                ),
                              ],
                            ),
                            value:
                                _sortOrder == "Ascending"
                                    ? "Ascending"
                                    : "Descending",
                            groupValue:
                                _sortOrder == "Ascending"
                                    ? "Ascending"
                                    : "Descending",
                            onChanged: (val) {
                              setState(() => _sortOrder = val!);
                              Navigator.pop(context, val);
                            },
                          ),
                        ),
                        // PopupMenuItem(
                        //   value: "Descending",
                        //   child: RadioListTile<String>(
                        //     title: Row(
                        //       children: const [
                        //         Text("Alphabetical"),
                        //         SizedBox(width: 8),
                        //         Icon(Icons.arrow_downward, size: 16),
                        //       ],
                        //     ),
                        //     value: "Descending",
                        //     groupValue:
                        //         "Descending", // _sortOrder --- IGNORE ---
                        //     onChanged: (val) {
                        //       // setState(() => _sortOrder = val);
                        //       Navigator.pop(context, val);
                        //     },
                        //   ),
                        // ),
                      ];
                    },
                  ),
                  IconButton(
                    icon: const Icon(Icons.search),
                    onPressed: () {
                      setState(() => _searchQuery = " "); // trigger search mode
                    },
                  ),
                  // _isScanning
                  //     ? const Padding(
                  //       padding: EdgeInsets.symmetric(horizontal: 16.0),
                  //       child: SizedBox(
                  //         width: 20,
                  //         height: 20,
                  //         child: CircularProgressIndicator(
                  //           strokeWidth: 2,
                  //           valueColor: AlwaysStoppedAnimation<Color>(
                  //             Colors.white,
                  //           ),
                  //         ),
                  //       ),
                  //     )
                  //     : IconButton(
                  //       icon: const Icon(Icons.refresh),
                  //       onPressed: _scanFiles,
                  //     ),
                ]
                : [],
        // bottom: TabBar(
        //   tabs: const [
        //     Tab(text: "Songs", icon: Icon(Icons.music_note)),
        //     Tab(text: "Albums", icon: Icon(Icons.album)),
        //   ],
        // ),
      ),

      body: Container(
        color: Colors.black, // change background color to black
        child: _buildSongsTab(),
      ),
    );
    // );
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
                  // Each item gets a unique LayerLink
                  final LayerLink itemLayerLink = LayerLink();
                  return FixedHeightRow(
                    child: ListTile(
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
                          color: selected ? Colors.white : null,
                        ),
                      ),
                      subtitle: Text(
                        "${song.artist}${song.album != null && song.album!.isNotEmpty ? ' | ${song.album}' : ''}",
                      ),
                      onTap: () async {
                        if (_searchQuery.isEmpty) {
                          final fullIndex = manager.songInfos.indexWhere(
                            (s) => s.path == song.path,
                          );
                          if (fullIndex != -1)
                            await manager.playAllSongs(fullIndex);
                        } else {
                          await manager.playFilteredSongs(infos, i);
                        }
                      },
                      trailing: CompositedTransformTarget(
                        link: itemLayerLink,
                        child: IconButton(
                          onPressed: () {
                            _showSongMenu(context, itemLayerLink);
                          },
                          icon: SizedBox(
                            height: 20,
                            width: 20,
                            child: Image.asset(Images.dots),
                          ),
                        ),
                      ),
                    ),
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
          onTap: () {
            Navigator.pushNamed(context, AppRoutes.player);
          },
        ),
      ],
    );
  }

  // Helper as in your code
  Widget buildArtworkOrIcon({
    required Uint8List? artwork,
    required IconData fallbackIcon,
    required bool isSelected,
    double size = 40,
    double radius = 20,
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
