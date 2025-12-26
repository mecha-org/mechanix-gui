import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class PlaylistMenu extends StatelessWidget {
  final PlaylistInfo playlistInfo;

  const PlaylistMenu({super.key, required this.playlistInfo});

  @override
  Widget build(BuildContext context) {
    return MechanixMenu(
      animationDuration: const Duration(milliseconds: 100),
      theme: MechanixMenuThemeData(
        decoration: BoxDecoration(
          color: context.tertiary,
          borderRadius: BorderRadius.circular(8),
        ),
        itemBackgroundColor: context.tertiary,
      ),
      topTabWidth: 10,
      topTabRightSideShiftLength: 80,
      buttonIcon: const IconWidget(
        boxHeight: 24,
        boxWidth: 24,
        iconHeight: 24,
        iconWidth: 24,
        iconColor: Colors.white,
        iconPath: MusicIcons.threeDotIcon,
      ),
      dropdownPosition: DropdownPosition.centerRight,
      items: [
        MechanixMenuItemsType(
          onTap:
              () => context.read<SongsBloc>().add(
                DeletePlaylist(playlistInfo.id),
              ),
          title: "Delete",
          titleTextStyle: TextStyle(
            color: MusicColors.deleteColor,
            fontFamily: "Overused Grotesk",
          ),
          leading: const IconWidget(
            iconPath: MusicIcons.deleteIcon,
            iconColor: MusicColors.deleteColor,
          ),
        ),
      ],
    );
  }
}
