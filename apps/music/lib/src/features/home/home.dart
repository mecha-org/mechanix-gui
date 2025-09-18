import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/app_routes.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_event.dart';
import 'package:mechanix_music/src/features/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/presentation/song_list_view.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  String _searchQuery = "";
  final TextEditingController _searchController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Colors.black,
        surfaceTintColor: Colors.black,
        elevation: 0,
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
                    fillColor: Colors.white.withOpacity(0.1),
                    hintText: "Search...",
                    hintStyle: const TextStyle(color: Colors.white54),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(20.0),
                      borderSide: BorderSide.none,
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
        actions: [
          IconButton(
            icon: const Icon(Icons.search),
            onPressed: () {
              Navigator.pushNamed(context, AppRoutes.searchPage);
            },
          ),
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () {
              context.read<SongsBloc>().add(ScanSongs());
            },
          ),
        ],
      ),
      body: BlocBuilder<SongsBloc, SongsState>(
        builder: (context, state) {
          return Scaffold(
            backgroundColor: Colors.black,
            body: SongsListView(
              songs: state.songs,
              onSongTap: (song) {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (_) => AudioPlayer(songDetails: song),
                  ),
                );
                // Handle play / navigation
              },
            ),
          );
        },
      ),
    );
  }

  Widget buildArtworkOrIcon({
    required Uint8List? artwork,
    required IconData fallbackIcon,
    double size = 40,
    double radius = 8,
  }) {
    if (artwork != null) {
      return ClipRRect(
        borderRadius: BorderRadius.circular(50),
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
          borderRadius: BorderRadius.circular(50),
        ),
        child: Icon(fallbackIcon, color: Colors.grey.shade600),
      );
    }
  }
}
