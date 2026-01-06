import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/select/select_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';

class WirelessProtocolsList extends StatefulWidget {
  const WirelessProtocolsList({
    this.accessPoint,
    super.key,
  });

  final NetworkManagerAccessPoint? accessPoint;

  @override
  State<WirelessProtocolsList> createState() => _WirelessProtocolsListState();
}

class _WirelessProtocolsListState extends State<WirelessProtocolsList> {
  WirelessProtocol? selectedValue;

  void onChange(SelectOption value) {
    setState(() {
      selectedValue = value.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              CustomTitle(
                title: "Join '${utf8.decode(widget.accessPoint!.ssid)}'",
              ),
              MechanixSimpleList(listItems: [
                SimpleListItems(
                    title: 'Private Wireless Address',
                    trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        value: true,
                        onChanged: (v) {}))
              ]),
              MechanixSelect(
                value: selectedValue,
                onChanged: onChange,
                options: wirelessProtocolOptions,
              ),
            ],
          ),
        ),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
      ),
    );
  }
}
