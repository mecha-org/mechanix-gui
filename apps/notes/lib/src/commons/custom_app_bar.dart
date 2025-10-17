import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/text.dart';

class CustomAppBar extends StatelessWidget implements PreferredSizeWidget {
  final String title;
  final Widget? leftIcon;
  final Widget? rightIcon1;
  final Widget? rightIcon2;
  final GestureTapCallback? leftIconOnTap;
  final GestureTapCallback? rightIcon1OnTap;
  final GestureTapCallback? rightIcon2OnTap;

  const CustomAppBar({
    super.key,
    required this.title,
    this.leftIcon,
    this.rightIcon1,
    this.rightIcon2,
    this.leftIconOnTap,
    this.rightIcon1OnTap,
    this.rightIcon2OnTap,
  });

  @override
  Widget build(BuildContext context) {
    return AppBar(
      automaticallyImplyLeading: false,
      title: Row(
        children: [
          if (leftIcon != null)
            Padding(
              padding: const EdgeInsets.only(right: 8.0),
              child: IconButton(
                onPressed: leftIconOnTap,
                icon: SizedBox(width: 30, height: 30, child: leftIcon),
              ),
            ),
          Text(title, style: headerTextStyle),
        ],
      ),
      actions: [
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
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}
