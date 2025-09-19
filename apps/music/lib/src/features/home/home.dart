import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/app_routes.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_event.dart';
import 'package:mechanix_music/src/features/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/home/mini_player.dart';
import 'package:mechanix_music/src/features/presentation/song_list_view.dart';
import 'package:widgets/extensions/edge_insets.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        surfaceTintColor: Colors.transparent,
        elevation: 0,
        title: const Text("Music"),
        actions: [
          IconButton(
            icon: const Icon(Icons.search),
            onPressed: () {
              Navigator.pushNamed(context, AppRoutes.searchPage);
            },
          ).padRight(5),
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () {
              context.read<SongsBloc>().add(ScanSongs());
            },
          ).padRight(5),
        ],
      ),
      body: BlocBuilder<SongsBloc, SongsState>(
        builder: (context, state) {
          return Stack(
            children: [
              // Song list with padding at bottom so it's not hidden by MiniPlayer
              Positioned.fill(
                child: SingleChildScrollView(
                  child: Column(
                    children: [
                      SongsListView(
                        songs: state.songs,
                        onSongTap: (song) {
                          Navigator.push(
                            context,
                            MaterialPageRoute(
                              builder: (_) => AudioPlayer(songDetails: song),
                            ),
                          );
                        },
                      ),
                      const SizedBox(height: 80), // space for MiniPlayer
                    ],
                  ),
                ),
              ),

              Positioned(
                left: 0,
                right: 0,
                bottom: 0,
                child:
                    state.currentSong != null
                        ? MiniPlayer(
                          currentPosition: state.position,
                          totalDuration: state.duration,
                          isPlaying: state.isPlaying,
                          currentIndex: state.currentIndex,
                          currentSong: state.currentSong!,
                        )
                        : SizedBox.shrink(),
              ),
            ],
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
