import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/widgets/icon_widget.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class BottomMenu extends StatelessWidget {
  const BottomMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return MechanixMenu(
      animationDuration: const Duration(milliseconds: 100),
      topTabWidth: 10,
      topTabRightSideShiftLength: 80,
      dropdownPosition: DropdownPosition.topRight,
      padding: const EdgeInsets.only(top: 0),
      theme: const MechanixMenuThemeData(
        dropdownWidth: 180,
        buttonMargin: EdgeInsets.only(right: 12),
      ),
      offset: const Offset(-5, -15),
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
          title: "Add Playlist",
          leading: IconWidget(
            iconPath: MusicIcons.addToPlaylistIcon,
            iconColor: Colors.white,
          ),
        ),
        MechanixMenuItemsType(
          onTap: () {},
          title: "List View",
          leading: const IconWidget(
            iconPath: MusicIcons.listViewIcon,
            iconColor: Colors.white,
          ),
        ),
      ],
    );
  }
}
