import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';

class ApplyWallpaper extends StatefulWidget {
  const ApplyWallpaper({super.key});

  @override
  State<ApplyWallpaper> createState() => _ApplyWallpaperState();
}

class _ApplyWallpaperState extends State<ApplyWallpaper> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    final args =
        ModalRoute.of(context)!.settings.arguments as Map<String, dynamic>;

    final imagePath = args['imagePath'] as String;

    return Scaffold(
      appBar: CustomAppBar(
        title: 'Apply Wallpaper',
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: () => backNavigation(context),
      ),
      body: ContainerWidget(
        child: SizedBox.expand(
          child: Image.asset(
            Images.wallpaper,    // use imagePath
            fit: BoxFit.cover,  
          ),
        ),
      ),
    );
  }
}
