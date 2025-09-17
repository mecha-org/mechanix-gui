import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:mechanix_settings/src/features/network/presentation/ipsettings/ip_static_details.dart';
import 'package:widgets/mechanix.dart';

class IpSettings extends StatefulWidget {
  const IpSettings({super.key});

  @override
  State<IpSettings> createState() => _IpSettingsState();
}

class _IpSettingsState extends State<IpSettings> {
  IpModes? selectedMode;

  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: MechanixNavigationBar(title: "Network"),
      body: ContainerWidget(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              "IP Settings",
              style: baseHeaderStyle,
            ),
            const SizedBox(height: 20),
            ListView.builder(
              shrinkWrap: true,
              itemCount: ipModesList.length,
              itemBuilder: (context, index) {
                final modeKey = ipModesList.keys.elementAt(index);
                final modeLabel = ipModesList.values.elementAt(index);

                return Container(
                    margin: EdgeInsets.only(bottom: 10),
                    height: 48,
                    padding: EdgeInsets.symmetric(horizontal: 10),
                    alignment: Alignment.center,
                    decoration: rowBoxDecoration,
                    child: RadioListTile<IpModes>(
                      title: Text(
                        modeLabel,
                        style: baseHeaderStyle,
                      ),
                      controlAffinity: ListTileControlAffinity.trailing,
                      value: modeKey,
                      groupValue: selectedMode,
                      onChanged: (value) {
                        setState(() {
                          if (value != null) {
                            selectedMode = value;
                          }
                        });
                      },
                      activeColor: const Color(0xFF2D8AFF),
                      materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      contentPadding:
                          const EdgeInsets.symmetric(horizontal: 10),
                      dense: true,
                    ));
              },
            ),
            if (selectedMode == IpModes.static)
              IpStaticDetails(selectedMode: selectedMode),
          ],
        ),
      ),
    );
  }
}
