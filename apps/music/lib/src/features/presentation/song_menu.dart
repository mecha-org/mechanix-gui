import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/widgets/icon_widget.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class SongMenu extends StatelessWidget {
  const SongMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return MechanixMenu(
      animationDuration: const Duration(milliseconds: 100),
      topTabWidth: 10,
      topTabRightSideShiftLength: 80,
      dropdownPosition: DropdownPosition.centerRight,
      theme: const MechanixMenuThemeData(
        dropdownWidth: 180,
        titleTextStyle: TextStyle(
          fontSize: 18,
          height: 1.2,
          color: MusicColors.primaryTextColor,
          fontFamily: "Overused Grotesk",
        ),
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
        const MechanixMenuItemsType(
          title: "Play Next",
          leading: IconWidget(
            iconPath: MusicIcons.playNextIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () {},

          title: "Add to queue",
          leading: const IconWidget(
            iconPath: MusicIcons.queueIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () {},
          title: "Add to playlist",
          leading: const IconWidget(
            iconPath: MusicIcons.addToPlaylistIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () {},
          title: "Add to favourite",
          leading: const IconWidget(
            iconPath: MusicIcons.favouritesIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () {},
          title: "Delete",
          titleTextStyle: TextStyle(color: MusicColors.deleteColor, fontFamily: "Overused Grotesk"),
          leading: const IconWidget(
            iconPath: MusicIcons.deleteIcon,
            iconColor: MusicColors.deleteColor,
          ),
        ),
      ],
    );
  }
}
