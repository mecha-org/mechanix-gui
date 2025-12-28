import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/favourites_tab/favourites_tab.dart';
import 'package:mechanix_music/src/features/home/bottom_bar.dart';
import 'package:mechanix_music/src/features/home/home_tab/home_tab.dart';
import 'package:mechanix_music/src/features/music_tab/music_tab.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tab.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/playlist_view.dart';
import 'package:mechanix_music/src/features/search_tab/search_tab.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  Widget _getTabWidget(MusicTabs tab) {
    switch (tab) {
      case MusicTabs.search:
        return SearchTab();
      case MusicTabs.music:
        return MusicTab();
      case MusicTabs.playlistInfo:
        return PlaylistView();
      case MusicTabs.playlists:
        return PlaylistTab();
      case MusicTabs.favorites:
        return FavouritesTab();
      default:
        return HomeTab();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      bottomNavigationBar: BlocSelector<SongsBloc, SongsState, BottomBarView>(
        selector: (state) => state.bottomBarView,
        builder:
            (context, bottomBarView) =>
                bottomBarView == BottomBarView.normal
                    ? const BottomBar()
                    : const SizedBox.shrink(),
      ),
      body: BlocBuilder<SongsBloc, SongsState>(
        buildWhen: (p, c) => p.musicTab != c.musicTab,
        builder: (context, state) {
          return AnimatedSwitcher(
            duration: const Duration(milliseconds: 300),
            switchInCurve: Curves.easeInOut,
            switchOutCurve: Curves.easeInOut,
            transitionBuilder: (Widget child, Animation<double> animation) {
              // Fade transition
              return FadeTransition(opacity: animation, child: child);
            },
            child: KeyedSubtree(
              key: ValueKey<MusicTabs>(state.musicTab),
              child: _getTabWidget(state.musicTab),
            ),
          );
        },
      ),
    );
  }
}
