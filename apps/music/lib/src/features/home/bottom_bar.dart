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
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';

class BottomBar extends StatelessWidget {
  const BottomBar({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, BottomBarView>(
      selector: (state) => state.bottomBarView,
      builder:
          (context, bottomBarView) =>
              bottomBarView == BottomBarView.add
                  ? AddPlaylistBar()
                  : Column(
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
                              leadingWidget: [
                                BottomBarButton(
                                  iconTheme:
                                      const MechanixBottomBarIconThemeData(
                                        iconSize: Size(28, 28),
                                        iconBoxSize: Size(44, 44),
                                      ),
                                  onPressed: () {
                                    if (Navigator.canPop(context)) {
                                      Navigator.pop(context);
                                    }
                                  },

                                  iconPath: MusicIcons.backIcon,
                                ),
                              ],
                              centerWidget: [
                                BottomBarButton(
                                  onPressed: () {
                                    context.read<SongsBloc>().add(
                                      MusicTabSwitch(MusicTabs.home),
                                    );
                                  },
                                  isSelected: state == MusicTabs.home,
                                  iconTheme:
                                      const MechanixBottomBarIconThemeData(
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
                                  iconTheme:
                                      const MechanixBottomBarIconThemeData(
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
                                  iconTheme:
                                      const MechanixBottomBarIconThemeData(
                                        iconSize: Size(28, 28),
                                        iconBoxSize: Size(44, 44),
                                      ),
                                  iconPath: MusicIcons.musicIcon,
                                ),
                                BottomBarButton(
                                  isSelected: state == MusicTabs.playlists,
                                  onPressed: () {
                                    context.read<SongsBloc>().add(
                                      MusicTabSwitch(MusicTabs.playlists),
                                    );
                                  },
                                  iconTheme:
                                      const MechanixBottomBarIconThemeData(
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
                                  iconTheme:
                                      const MechanixBottomBarIconThemeData(
                                        iconSize: Size(28, 28),
                                        iconBoxSize: Size(44, 44),
                                      ),
                                  iconPath: MusicIcons.favouritesIcon,
                                ),
                              ],
                              anchorWidget: [
                                BottomBarButton.widget(widget: BottomMenu()),
                              ],
                            ),
                      ),
                    ],
                  ),
    );
  }
}
