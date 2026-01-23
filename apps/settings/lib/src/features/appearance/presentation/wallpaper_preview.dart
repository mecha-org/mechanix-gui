import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/appearance/bloc/appearance_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';

class WallpaperPreview extends StatefulWidget {
  const WallpaperPreview({super.key});

  @override
  State<WallpaperPreview> createState() => _WallpaperPreviewState();
}

class _WallpaperPreviewState extends State<WallpaperPreview> {
  void _onSave() {
    context.read<AppearanceBloc>().add(ApplyWallpaperEvent());

    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    final currentWallpaper =
        context.read<AppearanceBloc>().state.wallpaperFileName;
    final appliedVariant = context.read<AppearanceBloc>().state.appliedVariant;
    final accentImage = accentPreviewImages
        .firstWhere((accent) => accent.accent == appliedVariant);
    final wallpaperPreview = wallpapersList
        .firstWhere((wallpaper) => wallpaper.wallpaper == currentWallpaper)
        .wallpaperPreview;

    return Scaffold(
      body: ContainerWidget(
        child: Column(
          children: [
            const CustomTitle(title: "Set wallpaper"),
            Center(
              child: Container(
                width: 388.39,
                height: 445,
                color: context.secondary,
                child: Center(
                  child: SizedBox(
                    width: 348.39,
                    height: 400,
                    child: Stack(
                      children: [
                        Positioned(
                          bottom: 0,
                          child: Image.asset(
                            accentImage.wallpaperPreview,
                            width: 348,
                          ),
                        ),
                        Positioned(
                          top: 0,
                          child: Container(
                            width: 348,
                            height: 350,
                            decoration: BoxDecoration(
                              image: DecorationImage(
                                image: AssetImage(wallpaperPreview),
                                fit: BoxFit.cover,
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
        anchorWidget: [
          BottomBarButton.widget(
            widget: IconButton(
              onPressed: _onSave,
              icon: const IconWidget(iconPath: Images.submit),
            ).padOnly(right: 8),
          )
        ],
      ),
    );
  }
}
