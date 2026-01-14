import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_protocols_list.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class WifiSecurityWidget extends StatefulWidget {
  const WifiSecurityWidget({super.key});

  @override
  State<WifiSecurityWidget> createState() => _WifiSecurityWidgetState();
}

class _WifiSecurityWidgetState extends State<WifiSecurityWidget> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  WirelessProtocol? selectedValue;

  void onChange(SelectOption value) {
    setState(() {
      selectedValue = value.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const MechanixNavigationBar(title: "Configure DNS"),
      body: const ContainerWidget(
        child: SingleChildScrollView(
          child: WirelessProtocolsList(),
        ),
      ).padOnly(left: 16, right: 16, top: 8),
    );
  }
}
