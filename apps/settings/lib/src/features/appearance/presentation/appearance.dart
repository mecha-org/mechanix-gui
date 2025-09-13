import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/switch_row.dart';

class Appearance extends StatefulWidget {
  const Appearance({super.key});

  @override
  State<Appearance> createState() => _AppearanceState();
}

class _AppearanceState extends State<Appearance> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  void onThemeSelected(String imagePath) {
    Navigator.pushNamed(
      context,
      AppRoutes.applyWallpaper,
      arguments: {'imagePath': imagePath},
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: 'Appearance',
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: () => backNavigation(context),
      ),
      body: ContainerWidget(
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 16.0),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceAround,
                children: [
                  buildRoundedImage(Images.theme1, true,
                      () => onThemeSelected(Images.theme1)),
                  buildRoundedImage(Images.theme2, false,
                      () => onThemeSelected(Images.theme2)),
                  buildRoundedImage(Images.theme3, false,
                      () => onThemeSelected(Images.theme3)),
                ],
              ),
            ),
            // mid content
            Expanded(
              child: SingleChildScrollView(
                child: Column(
                  children: [
                    SizedBox(height: 20),
                  ],
                ),
              ),
            ),
            FixedHeightRow(
              showTopBorder: true,
              showBottomBorder: false,
              child: CustomSwitchListTile(
                title: "Ambient mode",
                value: true,
                onChanged: (val) {
                  // set mode
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

Widget buildRoundedImage(
    String imagePath, bool currentWallpaper, VoidCallback onTap) {
  return GestureDetector(
    onTap: onTap,
    child: Container(
      width: 120,
      height: 120,
      decoration: BoxDecoration(
        border: Border.all(
          color: currentWallpaper ? Colors.lightBlue : Colors.grey,
          width: 2,
        ),
        borderRadius: BorderRadius.circular(16),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: Image.asset(
          imagePath,
          fit: BoxFit.cover,
        ),
      ),
    ),
  );
}
