import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class BottomMenu extends StatelessWidget {
  const BottomMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<
      SongsBloc,
      SongsState,
      Tuple2<PlaylistViewEnum, MusicTabs>
    >(
      selector: (state) => Tuple2(state.playlistView, state.musicTab),
      builder: (context, state) {
        final playlistView = state.item1;

        return MechanixMenu(
          animationDuration: const Duration(milliseconds: 100),

          topTabWidth: 10,
          topTabRightSideShiftLength: 80,

          dropdownPosition: DropdownPosition.topRight,
          padding: const EdgeInsets.only(top: 0),

          theme: MechanixMenuThemeData(
            decoration: BoxDecoration(
              color: context.tertiary,
              borderRadius: BorderRadius.circular(8),
            ),
            itemBackgroundColor: context.tertiary,
          ),

          // theme: const MechanixMenuThemeData(
          //   buttonMargin: EdgeInsets.only(right: 12),
          // ),
          offset: const Offset(5, -15),
          buttonIcon: IconWidget(
            boxHeight: 24,
            boxWidth: 24,
            iconHeight: 24,
            iconWidth: 24,
            iconColor: Colors.white,
            iconPath: MusicIcons.threeDotIcon,
            activeIconColor: MusicColors.borderColor,
          ),
          items: [
            MechanixMenuItemsType(
              onTap: () {
                context.read<SongsBloc>().add(
                  BottomBarToggle(BottomBarView.add),
                );
              },
              title: "New Playlist",
              leading: IconWidget(
                iconPath: MusicIcons.addToPlaylistIcon,
                iconColor: Colors.white,
              ),
            ),
            MechanixMenuItemsType(
              onTap: () {
                context.read<SongsBloc>().add(
                  PlaylistViewMode(
                    playlistView == PlaylistViewEnum.list
                        ? PlaylistViewEnum.grid
                        : PlaylistViewEnum.list,
                  ),
                );
              },
              title:
                  "${playlistView == PlaylistViewEnum.list ? "Grid" : "List"} View",
              leading: const IconWidget(
                iconPath: MusicIcons.listViewIcon,
                iconColor: Colors.white,
              ),
            ),
          ],
        ).padRight(12);
      },
    );
  }
}
