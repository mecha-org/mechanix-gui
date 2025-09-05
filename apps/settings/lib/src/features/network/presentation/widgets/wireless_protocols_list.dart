import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/select/select_type.dart';

class WirelessProtocolsList extends StatelessWidget {
  const WirelessProtocolsList(
      {super.key, this.selectedValue = '', required this.onChanged});

  final String? selectedValue;

  final void Function(SelectOption) onChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        MechanixSimpleList(listItems: [
          SimpleListItems(
              title: 'Private Wireless Address',
              trailing: CustomToggle(value: false, onChanged: (v) {}))
        ]),
        MechanixSelect(
          selectValue: selectedValue,
          onChanged: onChanged,
          onTap: () {},
          options: [
            SelectOption(
              label: 'None',
              value: 'NONE',
            ),
            SelectOption(
              label: 'WEP',
              value: 'WEP',
            ),
            SelectOption(
              label: 'WPA',
              value: 'WPA',
            ),
            SelectOption(
              label: 'WPA2/WPA3',
              value: 'WPA2/WPA3',
            ),
            SelectOption(
              label: 'WPA3',
              value: 'WPA3',
            ),
            SelectOption(
              label: 'WPA Enterprise',
              value: 'WPA_ENTERPRISE',
            ),
            SelectOption(
              label: 'WPA2 Enterprise',
              value: 'WPA2_ENTERPRISE',
            ),
            SelectOption(
              label: 'WPA3 Enterprise',
              value: 'WPA3_ENTERPRISE',
            ),
          ],
        ),
      ],
    );
  }
}
