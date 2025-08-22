import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/label_value_row.dart';
import 'package:mechanix_settings/src/features/about/models/types.dart';
import 'package:mechanix_settings/src/commons/styles/text.dart';

class About extends StatefulWidget {
  const About({super.key});

  @override
  State<About> createState() => _AboutState();
}

class _AboutState extends State<About> {

  void _backNavigation() {
    Navigator.pop(context);
  }

   AboutDetailsType aboutDetails = AboutDetailsType(deviceName: '', iconPath: '', hostName: '', makeOrModel: '', wirelessIpAddress: '', 
    wirelessMacAddress: '', ethernetIpAddress: '', ethernetMacAddress: '', os: '', kernelVersion: '');

  @override
  void initState() {
    super.initState();
    // Simulate API load or default init
    aboutDetails = AboutDetailsType(
      deviceName: 'My Mecha Comet',
      iconPath: Images.device,
      hostName: 'comet-m.local',
      makeOrModel: 'Mecha Comet',
      wirelessIpAddress: '192.168.1.1',
      wirelessMacAddress: '00:1A:2B:3C:4D:5E',
      ethernetIpAddress: '192.168.1.2',
      ethernetMacAddress: '00:1A:2B:3C:4D:5F',
      os: 'Linux',
      kernelVersion: '6.6.36+mecha+',
    );
  }

  @override
  Widget build(BuildContext context) {

    return Scaffold(
      appBar: CustomAppBar(
        title: 'About Device',
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: _backNavigation,
      ),
      body: ContainerWidget(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Fixed non-scrolling header
            ListTile(
              title: Text(
                aboutDetails.deviceName,
                style: subHeaderTextStyle,
              ),
              trailing: Image.asset(
                aboutDetails.iconPath,
                height: 40,
                width: 40,
              ),
            ),

            // Scrollable section
            Expanded(
              child: SingleChildScrollView(
                child: Column(
                  children: [
                    LabelValueListRow(title: 'Host Name', value: aboutDetails.hostName),
                    LabelValueListRow(title: 'Make / Model', value: aboutDetails.makeOrModel),
                    LabelValueListRow(title: 'Wireless IP', value: aboutDetails.wirelessIpAddress),
                    LabelValueListRow(title: 'Ethernet IP', value: aboutDetails.ethernetIpAddress),
                    LabelValueListRow(title: 'Wireless MAC', value: aboutDetails.wirelessMacAddress),
                    LabelValueListRow(title: 'Ethernet MAC', value: aboutDetails.ethernetMacAddress),
                    LabelValueListRow(title: 'OS', value: aboutDetails.os),
                    LabelValueListRow(title: 'Kernel', value: aboutDetails.kernelVersion),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}