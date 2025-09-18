import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_label_value.dart';
import 'package:mechanix_settings/src/commons/customWidgets/switch_row.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:widgets/mechanix.dart';

class EthernetSettings extends StatefulWidget {
  const EthernetSettings({super.key});

  @override
  State<EthernetSettings> createState() => _EthernetSettingsState();
}

class _EthernetSettingsState extends State<EthernetSettings> {
  bool isPowered = false;
  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: MechanixNavigationBar(title: "Network"),
        body: ContainerWidget(
            child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            CustomSwitchListTile(
                title: "Ethernet",
                value: isPowered,
                onChanged: (val) => setState(() {
                      isPowered = !isPowered;
                    })),
            if (isPowered)
              CustomLabelValue(
                title: "Ip Settings",
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text(
                      "None",
                      style: secondaryHeaderStyle,
                    ),
                    const SizedBox(width: 10),
                    CustomIcon(
                        height: 16,
                        width: 16,
                        icon: Image.asset(Images.rightIconArrow))
                  ],
                ),
              )
          ],
        ).padTop(8)));
  }
}
