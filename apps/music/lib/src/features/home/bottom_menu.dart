import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class BottomMenu extends StatelessWidget {
  const BottomMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, PlaylistViewEnum>(
      selector: (state) => state.playlistView,
      builder:
          (context, playlistView) => MechanixMenu(
            animationDuration: const Duration(milliseconds: 100),

            topTabWidth: 10,
            topTabRightSideShiftLength: 80,

            dropdownPosition: DropdownPosition.topRight,
            padding: const EdgeInsets.only(top: 0),

            // theme: const MechanixMenuThemeData(
            //   buttonMargin: EdgeInsets.only(right: 12),
            // ),
            offset: const Offset(5, -15),
            buttonIcon: const IconWidget(
              boxHeight: 24,
              boxWidth: 24,
              iconHeight: 24,
              iconWidth: 24,
              iconColor: Colors.white,
              iconPath: MusicIcons.threeDotIcon,
            ),
            items: [
              MechanixMenuItemsType(
                onTap: () {
                  context.read<SongsBloc>().add(BottomBarToggle());
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
          ).padRight(12),
    );
  }
}
