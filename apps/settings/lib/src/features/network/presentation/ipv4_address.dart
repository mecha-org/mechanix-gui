import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
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

  String? selectedValue = '';

  void onChange(SelectOption value) {
    setState(() {
      selectedValue = value.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: "Configure DNS",
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: () => backNavigation(context),
      ),
      body: ContainerWidget(
        child: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              MechanixSelect(
                selectValue: selectedValue,
                onChanged: onChange,
                onTap: () {},
                options: [
                  SelectOption(
                    label: 'Automatic (DHCP)',
                    value: 'AUTOMATIC_DHCP',
                  ),
                  SelectOption(
                    label: 'Static',
                    value: 'STATIC',
                  ),
                ],
              ),
              if (selectedValue == 'STATIC')
                MechanixSectionList(title: 'Static', sectionListItems: [
                  SectionListItems(
                      title: 'IP Settings', trailing: Text('255.255.255.25')),
                  SectionListItems(
                      title: 'Gateway',
                      trailing: Row(
                        children: [
                          Text('None').padRight(8),
                          IconWidget(
                            iconWidth: 9,
                            iconHeight: 18,
                            iconPath: Images.rightIconArrow,
                          )
                        ],
                      ))
                ]).padOnly(top: 40, bottom: 8)
            ],
          ),
        ),
      ).padHorizontal(16),
    );
  }
}
