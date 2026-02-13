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
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

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
  bool isPrivateAddress = false;

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
                title:
                    "Join ${widget.accessPoint != null ? '\'${utf8.decode(widget.accessPoint!.ssid)}\'' : ''}",
              ),
              MechanixSimpleList(listItems: [
                SimpleListItems(
                  title: 'Private Wireless Address',
                  titleTextStyle: const TextStyle(fontWeight: FontWeight.w700),
                  onTap: () {
                    setState(() {
                      isPrivateAddress = !isPrivateAddress;
                    });
                  },
                  trailing: MechanixSwitch(
                    activeText: 'OFF',
                    inactiveText: 'ON',
                    style: MechanixSwitchStyle(
                      activeTrackColor: context.secondaryContainer,
                      inactiveTrackColor: context.secondaryContainer,
                    ),
                    value: isPrivateAddress,
                    onChanged: (val) => {
                      setState(() {
                        isPrivateAddress = val;
                      })
                    },
                  ),
                ),
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
