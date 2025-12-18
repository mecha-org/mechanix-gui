import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/favourites_tab/favourites_tab.dart';
import 'package:mechanix_music/src/features/home/bottom_bar.dart';
import 'package:mechanix_music/src/features/home/home_tab.dart';
import 'package:mechanix_music/src/features/music_tab/music_tab.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tab.dart';
import 'package:mechanix_music/src/features/search_tab/search_tab.dart';

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
      bottomNavigationBar: BottomBar(),
      body: BlocBuilder<SongsBloc, SongsState>(
        buildWhen: (p, c) => p.musicTab != c.musicTab,
        builder: (context, state) {
          if (state.musicTab == MusicTabs.search) {
            return SearchTab();
          } else if (state.musicTab == MusicTabs.music) {
            return MusicTab();
          } else if (state.musicTab == MusicTabs.playlists) {
            return PlaylistTab();
          } else if (state.musicTab == MusicTabs.favorites) {
            return FavouritesTab();
          }
          return HomeTab();
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
