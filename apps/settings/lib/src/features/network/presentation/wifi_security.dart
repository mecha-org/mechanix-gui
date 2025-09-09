import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
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
            child: WirelessProtocolsList(
          onChanged: onChange,
          selectedValue: selectedValue,
        )),
      ).padHorizontal(16),
    );
  }
}
