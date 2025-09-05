import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/route_list.dart';

class CustomAppBar extends StatelessWidget implements PreferredSizeWidget {
  final String title;
  final Widget? leftIcon;
  final Widget? customChild;
  final Widget? rightIcon1;
  final Widget? rightIcon2;
  final GestureTapCallback? leftIconOnTap;
  final GestureTapCallback? rightIcon1OnTap;
  final GestureTapCallback? rightIcon2OnTap;

  const CustomAppBar(
      {super.key,
      required this.title,
      this.leftIcon,
      this.rightIcon1,
      this.rightIcon2,
      this.leftIconOnTap,
      this.rightIcon1OnTap,
      this.rightIcon2OnTap,
      this.customChild});

  @override
  Widget build(BuildContext context) {
    final currentRoute = ModalRoute.of(context)?.settings.name ?? '/';
    final meta = routeMetaMap[currentRoute];
    final backTitle = meta?.backTitle ?? 'Settings';

    return AppBar(
      automaticallyImplyLeading: false,
      titleSpacing: 0,
      title: Row(
        children: [
          if (leftIcon != null)
            MouseRegion(
              cursor: SystemMouseCursors.click,
              child: GestureDetector(
                behavior: HitTestBehavior.translucent,
                onTap: leftIconOnTap,
                child: Row(
                  children: [
                    Container(
                      padding: const EdgeInsets.only(left: 20, right: 5),
                      child: SizedBox(width: 18, height: 18, child: leftIcon),
                    ),
                    if (currentRoute != '/')
                      Text(
                        backTitle,
                        style: TextStyle(
                          color: Color(0xFF2D8AFF),
                          fontWeight: FontWeight.w500,
                          fontSize: 16,
                        ),
                      ),
                  ],
                ),
              ),
            ),
          const SizedBox(width: 8),
          if (currentRoute == '/')
            Padding(
              padding: EdgeInsets.all(20),
              child: Text(
                backTitle,
                style: TextStyle(
                  fontWeight: FontWeight.w500,
                  color: Color(0xFFF0F0F0),
                  fontSize: 30,
                ),
              ),
            )
        ],
      ),
      actions: [
        if (customChild != null)
          Padding(
            padding: const EdgeInsets.only(right: 20.0),
            child: customChild,
          ),
        if (rightIcon1 != null)
          Padding(
            padding: const EdgeInsets.only(right: 8.0),
            child: IconButton(
              icon: SizedBox(width: 30, height: 30, child: rightIcon1),
              onPressed: rightIcon1OnTap,
            ),
          ),
        if (rightIcon2 != null)
          Padding(
            padding: const EdgeInsets.only(right: 28.0),
            child: IconButton(
              icon: SizedBox(width: 30, height: 30, child: rightIcon2),
              onPressed: rightIcon2OnTap,
            ),
          ),
      ],
    );
  }

  @override
  Size get preferredSize => Size.fromHeight(50);
}
