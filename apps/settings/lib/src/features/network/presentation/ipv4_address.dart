import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
import 'package:widgets/widgets/select/select_type.dart';

class Ipv4AddressWidget extends StatefulWidget {
  const Ipv4AddressWidget({super.key});

  @override
  State<Ipv4AddressWidget> createState() => _Ipv4AddressWidgetState();
}

class _Ipv4AddressWidgetState extends State<Ipv4AddressWidget> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  ConfigureDNS? selectedDNS;

  void _handleDNSChange(SelectOption<ConfigureDNS> option) {
    setState(() {
      selectedDNS = option.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: ContainerWidget(
        child: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const CustomTitle(title: "IPV4 Address"),
              MechanixSelect<ConfigureDNS>(
                options: dnsOptions,
                value: selectedDNS,
                onChanged: _handleDNSChange,
              ),
              if (selectedDNS == 'STATIC')
                MechanixSectionList(title: 'Static', sectionListItems: [
                  SectionListItems(
                      title: 'IP Settings',
                      trailing: const Text('255.255.255.25')),
                  SectionListItems(
                      title: 'Gateway',
                      trailing: Row(
                        children: [
                          const Text('None').padRight(8),
                          const IconWidget(
                            iconWidth: 9,
                            iconHeight: 18,
                            iconPath: Images.rightIconArrow,
                          )
                        ],
                      ))
                ]).padOnly(top: 40, bottom: 8)
            ],
          ).padTop(8),
        ),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
      ),
    );
  }
}
