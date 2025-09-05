import 'package:flutter/widgets.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class WirelessAdvanceSettings extends StatelessWidget {
  const WirelessAdvanceSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // const SizedBox(
        //   height: 30,
        // ),
        // const Text(
        //   "Advanced Settings",
        //   style: secondaryHeaderStyle,
        // ),
        // CustomLabelValue(
        //   title: "Manage Network",
        //   onTap: () => onTap(context, AppRoutes.wirelessNetworkSettings),
        //   child: CustomIcon(
        //       height: 16, width: 16, icon: Image.asset(Images.rightIconArrow)),
        // ),
        MechanixSectionList(title: 'Advanced Settings', sectionListItems: [
          SectionListItems(
              title: 'Manage Wireless',
              onTap: () => onTap(context, AppRoutes.wirelessNetworkSettings),
              trailing: IconWidget(
                iconWidth: 9,
                iconHeight: 18,
                iconPath: Images.rightIconArrow,
              ))
        ]),
        // CustomLabelValue(
        //   title: "IP Settings",
        //   onTap: () => onTap(context, AppRoutes.ipSettings),
        //   child: CustomIcon(
        //       height: 16,
        //       width: 16,
        //       icon: Image.asset(
        //         Images.rightIconArrow,
        //       )),
        // ),
        // CustomLabelValue(
        //   title: "Ethernet",
        //   onTap: () => onTap(context, AppRoutes.ethernetDetails),
        //   child: CustomIcon(
        //       height: 16,
        //       width: 16,
        //       icon: Image.asset(
        //         Images.rightIconArrow,
        //       )),
        // ),
        // CustomLabelValue(
        //   title: "DNS",
        //   onTap: () => onTap(context, AppRoutes.dnsDetails),
        //   child: CustomIcon(
        //       height: 16,
        //       width: 16,
        //       icon: Image.asset(
        //         Images.rightIconArrow,
        //       )),
        // ),
      ],
    );
  }
}

void onTap(BuildContext context, String route) {
  Navigator.pushNamed(context, route);
}
