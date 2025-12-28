import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_to_playlist_sheet.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class SongMenu extends StatelessWidget {
  final SongInfo song;
  final VoidCallback? onToggleFavourite;
  const SongMenu({super.key, required this.song, this.onToggleFavourite});
  void _showAddToPlaylistSheet(BuildContext context) {
    context.read<SongsBloc>().add(LoadPlaylist());

    MechanixBottomSheet.show(
      topTabWidth: 370,
      topTabRightSideShiftLength: 40,
      context,
      child: AddToPlaylistSheet(song: song),
    );
  }

  @override
  Widget build(BuildContext context) {
    return MechanixMenu(
      animationDuration: const Duration(milliseconds: 100),
      topTabWidth: 10,
      topTabRightSideShiftLength: 80,
      dropdownPosition: DropdownPosition.centerRight,
      theme: MechanixMenuThemeData(
        decoration: BoxDecoration(color: context.tertiary),
        itemBackgroundColor: context.tertiary,
      ),

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
          onTap:
              () => context.read<SongsBloc>().add(
                AddToQueue(song, playNext: true),
              ),
          title: "Play Next",
          leading: IconWidget(
            iconPath: MusicIcons.playNextIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () {
            context.read<SongsBloc>().add(AddToQueue(song));
          },
          title: "Add to queue",
          leading: const IconWidget(
            iconPath: MusicIcons.queueIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () => _showAddToPlaylistSheet(context),
          title: "Add to playlist",
          leading: const IconWidget(
            iconPath: MusicIcons.addToPlaylistIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () => onToggleFavourite?.call(),
          title:
              song.isFavourite ? "Remove from favourite" : "Add to favourite",
          leading: const IconWidget(
            iconPath: MusicIcons.favouritesIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () => context.read<SongsBloc>().add(DeleteSong(song)),
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
