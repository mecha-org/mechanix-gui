import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';

class WirelessProtocols extends StatelessWidget {
  const WirelessProtocols({super.key});

  @override
  Widget build(BuildContext context) {
    return MechanixSimpleList(listItems: [
      SimpleListItems(
          onTap: () {
            Navigator.pushNamed(context, AppRoutes.security);
          },
          title: 'Security',
          trailing: Row(
            children: [
              Text(
                'WPA2/WPA3',
                style: context.textTheme.labelLarge,
              ).padRight(8),
              IconWidget(
                iconWidth: 9,
                iconHeight: 18,
                iconPath: Images.rightIconArrow,
              )
            ],
          ))
    ]).padTop(40);
  }
}
