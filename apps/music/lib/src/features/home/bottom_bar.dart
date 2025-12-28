import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/bottom_menu.dart';
import 'package:mechanix_music/src/features/home/mini_player.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_playlist_bar.dart';
import 'package:mechanix_music/src/features/search_tab/music_search_bar.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';

class BottomBar extends StatelessWidget {
  const BottomBar({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, BottomBarView>(
      selector: (state) => state.bottomBarView,
      builder: (context, bottomBarView) {
        return AnimatedSwitcher(
          duration: const Duration(milliseconds: 300),
          switchInCurve: Curves.easeInOut,
          switchOutCurve: Curves.easeInOut,
          transitionBuilder: (Widget child, Animation<double> animation) {
            // Slide transition from bottom
            return SlideTransition(
              position: Tween<Offset>(
                begin: const Offset(0.0, 0.3),
                end: Offset.zero,
              ).animate(animation),
              child: FadeTransition(opacity: animation, child: child),
            );
          },
          child: _buildBottomBarContent(context, bottomBarView),
        );
      },
    );
  }

  Widget _buildBottomBarContent(
    BuildContext context,
    BottomBarView bottomBarView,
  ) {
    // Use unique keys for AnimatedSwitcher to detect changes
    if (bottomBarView == BottomBarView.add) {
      return AddPlaylistBar(key: const ValueKey('add_playlist'));
    } else if (bottomBarView == BottomBarView.search) {
      return const MusicSearchBar(key: ValueKey('search_bar'));
    }

    return Column(
      key: const ValueKey('default_bar'),
      mainAxisAlignment: MainAxisAlignment.end,
      mainAxisSize: MainAxisSize.min,
      children: [
        const MiniPlayer(),
        BlocSelector<SongsBloc, SongsState, MusicTabs>(
          selector: (state) => state.musicTab,
          builder:
              (context, state) => MechanixBottomBar(
                theme: MechanixBottomBarThemeData(
                  decoration: BoxDecoration(
                    color: Color(0xFF2E2E2E),
                    borderRadius: BorderRadius.only(
                      topLeft: Radius.circular(0),
                      topRight: Radius.circular(0),
                    ),
                  ),
                ),
                leadingWidget:
                    state == MusicTabs.playlistInfo
                        ? [
                          BottomBarButton(
                            isDisabled: state == MusicTabs.home,
                            iconTheme: MechanixBottomBarIconThemeData(
                              iconSize: Size(28, 28),
                              iconBoxSize: Size(44, 44),
                              iconColor:
                                  state == MusicTabs.home
                                      ? Theme.of(context).disabledColor
                                      : null,
                            ),
                            onPressed:
                                state == MusicTabs.home
                                    ? null
                                    : () {
                                      context.read<SongsBloc>().add(
                                        BackTabEvent(),
                                      );
                                    },
                            iconPath: MusicIcons.backIcon,
                          ),
                        ]
                        : [],
                centerWidget: [
                  BottomBarButton(
                    onPressed: () {
                      context.read<SongsBloc>().add(
                        MusicTabSwitch(MusicTabs.home),
                      );
                    },
                    isSelected: state == MusicTabs.home,
                    iconTheme: const MechanixBottomBarIconThemeData(
                      iconSize: Size(28, 28),
                      iconBoxSize: Size(44, 44),
                    ),
                    iconPath: MusicIcons.homeIcon,
                  ),
                  BottomBarButton(
                    isSelected: state == MusicTabs.search,
                    onPressed: () {
                      context.read<SongsBloc>().add(
                        MusicTabSwitch(MusicTabs.search),
                      );
                    },
                    iconTheme: const MechanixBottomBarIconThemeData(
                      iconSize: Size(28, 28),
                      iconBoxSize: Size(44, 44),
                    ),
                    iconPath: MusicIcons.searchIcon,
                  ),
                  BottomBarButton(
                    isSelected: state == MusicTabs.music,
                    onPressed: () {
                      context.read<SongsBloc>().add(
                        MusicTabSwitch(MusicTabs.music),
                      );
                    },
                    iconTheme: const MechanixBottomBarIconThemeData(
                      iconSize: Size(28, 28),
                      iconBoxSize: Size(44, 44),
                    ),
                    iconPath: MusicIcons.musicIcon,
                  ),
                  BottomBarButton(
                    isSelected:
                        state == MusicTabs.playlists ||
                        state == MusicTabs.playlistInfo,
                    onPressed: () {
                      context.read<SongsBloc>().add(
                        MusicTabSwitch(MusicTabs.playlists),
                      );
                    },
                    iconTheme: const MechanixBottomBarIconThemeData(
                      iconSize: Size(28, 28),
                      iconBoxSize: Size(44, 44),
                    ),
                    iconPath: MusicIcons.playlistIcon,
                  ),
                  BottomBarButton(
                    isSelected: state == MusicTabs.favorites,
                    onPressed: () {
                      context.read<SongsBloc>().add(
                        MusicTabSwitch(MusicTabs.favorites),
                      );
                    },
                    iconTheme: const MechanixBottomBarIconThemeData(
                      iconSize: Size(28, 28),
                      iconBoxSize: Size(44, 44),
                    ),
                    iconPath: MusicIcons.favouritesIcon,
                  ),
                ],
                anchorWidget:
                    state == MusicTabs.playlists
                        ? [BottomBarButton.widget(widget: BottomMenu())]
                        : [],
              ),
        ),
      ],
    );
  }
}
