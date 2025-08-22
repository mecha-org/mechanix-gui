import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_row_item.dart';

class NetworkListRow extends StatelessWidget {
  final Widget mainIcon;
  final String title;
  final VoidCallback onInfoTap;
  final VoidCallback? onNetworkTap;
  final bool isActive;

  const NetworkListRow(
      {super.key,
      required this.mainIcon,
      required this.title,
      required this.onInfoTap,
      this.onNetworkTap,
      required this.isActive});

  @override
  Widget build(BuildContext context) {
    return CustomRowItem(
      onTap: !isActive ? onNetworkTap : null,
      title: title,
      child: Row(
        spacing: 0,
        mainAxisSize: MainAxisSize.min,
        children: [
          if (isActive)
            IconButton(
                onPressed: null,
                icon: CustomIcon(
                    icon: Image.asset(
                  Images.checkIcon,
                  height: 20,
                  width: 20,
                ))),
          IconButton(onPressed: null, icon: CustomIcon(icon: mainIcon)),
          IconButton(
              onPressed: onInfoTap,
              icon: CustomIcon(icon: Image.asset(Images.settings))),
        ],
      ),
    );
  }
}
