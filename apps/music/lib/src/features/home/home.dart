import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/favourites_tab/favourites_tab.dart';
import 'package:mechanix_music/src/features/home/bottom_bar.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:mechanix_music/src/features/home/home_tab/home_tab.dart';
import 'package:mechanix_music/src/features/music_tab/music_tab.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tab.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/playlist_view.dart';
import 'package:mechanix_music/src/features/search_tab/search_tab.dart';
import 'package:media_kit/media_kit.dart';
import 'package:tuple/tuple.dart';

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
  void initState() {
    super.initState();
    MediaKit.ensureInitialized();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      bottomNavigationBar: const BottomBar(),
      floatingActionButton:
          BlocSelector<SongsBloc, SongsState, Tuple2<MusicTabs, bool>>(
            selector: (state) => Tuple2(state.musicTab, state.isScrolling),
            builder: (context, state) {
              final musicTab = state.item1;
              final isVisible = !state.item2;

              final isAllowedTab = const {
                MusicTabs.music,
                MusicTabs.home,
                MusicTabs.favorites,
                MusicTabs.playlists,
              }.contains(musicTab);

              if (!isAllowedTab) return const SizedBox.shrink();

              return AnimatedSlide(
                offset: isVisible ? Offset.zero : const Offset(0, 0.25),
                duration: const Duration(milliseconds: 220),
                curve: Curves.easeOutCubic,
                child: AnimatedScale(
                  scale: isVisible ? 1.0 : 0.8,
                  duration: const Duration(milliseconds: 180),
                  curve: Curves.easeOutBack,
                  child: AnimatedOpacity(
                    opacity: isVisible ? 1.0 : 0.0,
                    duration: const Duration(milliseconds: 160),
                    curve: Curves.easeOut,
                    child: IgnorePointer(
                      ignoring: !isVisible,
                      child: MusicIconButton(
                        backgroundColor: MusicColors.disabledColor,
                        icon: MusicIcons.searchIcon,
                        onPressed: () {
                          context.read<SongsBloc>().add(
                            MusicTabSwitch(MusicTabs.search),
                          );
                        },
                      ),
                    ),
                  ),
                ),
              );
            },
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
