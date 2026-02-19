import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/widgets/wallpaper_gallery.dart';
import 'package:widgets/mechanix.dart';

class SetUpWallpaper extends StatelessWidget {
  const SetUpWallpaper({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: const SingleChildScrollView(
        physics: BouncingScrollPhysics(),
        child: ContainerWidget(
            child: Column(
          children: [CustomTitle(title: "Set wallpaper"), WallpaperGallery()],
        )),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
      ),
    );
  }
}
