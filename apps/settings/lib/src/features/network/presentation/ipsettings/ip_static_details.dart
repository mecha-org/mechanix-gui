import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_label_value.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';

class IpStaticDetails extends StatelessWidget {
  final IpModes? selectedMode;
  const IpStaticDetails({super.key, this.selectedMode});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          "Static",
          style: secondaryHeaderStyle,
        ),
        CustomLabelValue(
          title: "IP Settings",
          value: "192.160.12.1",
        ),
        CustomLabelValue(
          title: "Gateway",
          value: "asds",
        )
      ],
    );
  }
}
