import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_label_value.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:widgets/mechanix.dart';

class DnsSettings extends StatelessWidget {
  const DnsSettings({super.key});
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
            Text(
              "DNS",
              style: baseHeaderStyle,
            ),
            CustomLabelValue(
              title: "DNS Server 1",
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    "AE:16:AF:80:CF:2F",
                    style: secondaryHeaderStyle,
                  ),
                  const SizedBox(width: 10),
                  CustomIcon(
                      height: 16,
                      width: 16,
                      icon: Image.asset(Images.rightIconArrow))
                ],
              ),
            ),
            CustomLabelValue(
              title: "DNS Server 1",
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    "AE:16:AF:80:CF:2F",
                    style: secondaryHeaderStyle,
                  ),
                  const SizedBox(width: 10),
                  CustomIcon(
                      height: 16,
                      width: 16,
                      icon: Image.asset(Images.rightIconArrow))
                ],
              ),
            ),
          ],
        ).padTop(8)));
  }
}
