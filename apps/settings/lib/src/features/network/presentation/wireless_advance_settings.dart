import 'package:flutter/widgets.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_label_value.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class WirelessAdvanceSettings extends StatelessWidget {
  const WirelessAdvanceSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const SizedBox(
          height: 30,
        ),
        const Text(
          "Advanced Settings",
          style: secondaryHeaderStyle,
        ),
        CustomLabelValue(
          title: "Manage Network",
          onTap: () => onTap(context, AppRoutes.wirelessNetworkSettings),
          child: CustomIcon(
              height: 16, width: 16, icon: Image.asset(Images.rightIconArrow)),
        ),
        CustomLabelValue(
          title: "IP Settings",
          onTap: () => onTap(context, AppRoutes.ipSettings),
          child: CustomIcon(
              height: 16,
              width: 16,
              icon: Image.asset(
                Images.rightIconArrow,
              )),
        ),
        CustomLabelValue(
          title: "Ethernet",
          onTap: () => onTap(context, AppRoutes.ethernetDetails),
          child: CustomIcon(
              height: 16,
              width: 16,
              icon: Image.asset(
                Images.rightIconArrow,
              )),
        ),
        CustomLabelValue(
          title: "DNS",
          onTap: () => onTap(context, AppRoutes.dnsDetails),
          child: CustomIcon(
              height: 16,
              width: 16,
              icon: Image.asset(
                Images.rightIconArrow,
              )),
        ),
      ],
    );
  }
}

void onTap(BuildContext context, String route) {
  Navigator.pushNamed(context, route);
}
