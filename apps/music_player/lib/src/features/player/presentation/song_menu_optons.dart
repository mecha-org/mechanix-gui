import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class MenuOptions extends StatelessWidget {
  final LayerLink menuLink;
  final OverlayEntry? entry;

  const MenuOptions({super.key, required this.menuLink, this.entry});

void _closeMenu() {
  if (entry != null && entry!.mounted) {
    entry!.remove();
  }
}


  @override
  Widget build(BuildContext context) {
    return Positioned.fill(
      child: Stack(
        children: [
          GestureDetector(
            behavior: HitTestBehavior.translucent,
            onTap: _closeMenu,
          ),
          CompositedTransformFollower(
            link: menuLink,
            showWhenUnlinked: false,
            targetAnchor: Alignment.topRight,
            followerAnchor: Alignment.topRight,
            offset: const Offset(-20, 40),
            child: SizedBox(
              width: 200,
              child: MechanixMenu(
                backgroundColor: const Color.fromRGBO(68, 68, 68, 0.95),
                borderRadius: 16,
                items: [
                  MechanixMenuItem(
                    label: "Play next",
                    textStyle: const TextStyle(
                      color: Color(0xFFF0F0F0),
                      fontWeight: FontWeight.w500,
                    ),
                    leadingWidget: Icon(
                      Icons
                          .playlist_play, // Replace with your custom icon if needed
                      color: Color(0xFFF0F0F0),
                      size: 22,
                    ),
                    onTap: () {
                      // your Play next action
                      _closeMenu();
                    },
                  ),
                  MechanixMenuDivider(thickness: 1, color: Color(0xFF333333)),
                  MechanixMenuItem(
                    label: "Add to queue",
                    textStyle: const TextStyle(
                      color: Color(0xFFF0F0F0),
                      fontWeight: FontWeight.w500,
                    ),
                    leadingWidget: Icon(
                      Icons.queue_music, // Replace with your custom icon
                      color: Color(0xFFF0F0F0),
                      size: 22,
                    ),
                    onTap: () {
                      // your Add to queue action
                      _closeMenu();
                    },
                  ),
                  MechanixMenuDivider(thickness: 1, color: Color(0xFF333333)),
                  MechanixMenuItem(
                    label: "Add to playlist",
                    textStyle: const TextStyle(
                      color: Color(0xFFF0F0F0),
                      fontWeight: FontWeight.w500,
                    ),
                    leadingWidget: Icon(
                      Icons.playlist_add, // Replace with your custom icon
                      color: Color(0xFFF0F0F0),
                      size: 22,
                    ),
                    onTap: () {
                      // your Add to playlist action
                      _closeMenu();
                    },
                  ),
                  MechanixMenuDivider(thickness: 1, color: Color(0xFF333333)),
                  MechanixMenuItem(
                    label: "Add to favorite",
                    textStyle: const TextStyle(
                      color: Color(0xFFF0F0F0),
                      fontWeight: FontWeight.w500,
                    ),
                    leadingWidget: Icon(
                      Icons.star_border, // Replace with custom icon to match UI
                      color: Color(0xFFF0F0F0),
                      size: 22,
                    ),
                    onTap: () {
                      // your Add to favorite action
                      _closeMenu();
                    },
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
