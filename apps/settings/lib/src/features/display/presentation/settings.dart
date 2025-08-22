import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';

class ScreenOffTimeSettings extends StatefulWidget {
  const ScreenOffTimeSettings({super.key});

  @override
  State<ScreenOffTimeSettings> createState() => ScreenOffTimeSettingsState();
}

class ScreenOffTimeSettingsState extends State<ScreenOffTimeSettings> {
  String? selectedTime;

  @override
  void initState() {
    super.initState();

    WidgetsBinding.instance.addPostFrameCallback((_) {
      final args =
          ModalRoute.of(context)!.settings.arguments as Map<String, String>;
      final screenOffTime = args['screenOffTime'];

      setState(() {
        selectedTime = screenOffTime;
      });
    });
  }

  void _backNavigation() {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    List<String> screenOffTimes = displayScreenOffTimeToString.values.toList();

    return Scaffold(
      appBar: CustomAppBar(
        title: "Screen Off Timeout",
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: _backNavigation,
      ),
      body: ContainerWidget(
        child: ListView.builder(
          itemCount: screenOffTimes.length,
          itemBuilder: (context, index) {
            final time = screenOffTimes[index];
            return Container(
                margin: EdgeInsets.only(bottom: 10),
                decoration: rowBoxDecoration,
                child: RadioListTile<String>(
                  controlAffinity: ListTileControlAffinity.trailing,
                  title: Text(
                    time,
                    style: baseHeaderStyle,
                  ),
                  value: time,
                  activeColor: Color(0xFF2D8AFF),
                  materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                  contentPadding: EdgeInsets.symmetric(horizontal: 10),
                  groupValue: selectedTime,
                  onChanged: (value) {
                    if (value != null) {
                      Navigator.pop(context, value);
                    }
                  },
                ));
          },
        ),
      ),
    );
  }
}
