import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/appearance/bloc/appearance_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

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
    final wallpaper = context.read<AppearanceBloc>().state.wallpaperFileName;
    final appliedVariant = context.read<AppearanceBloc>().state.appliedVariant;
    final accentImage = accentPreviewImages
        .firstWhere((accent) => accent.accent == appliedVariant);

    return Scaffold(
      body: ContainerWidget(
        child: Column(
          children: [
            const CustomTitle(title: "Set wallpaper"),
            Center(
              child: Container(
                width: 388.39,
                height: 440,
                color: context.secondary,
                child: Center(
                  child: SizedBox(
                    width: 348.39,
                    height: 400,
                    child: Stack(
                      children: [
                        Image.asset(accentImage.wallpaperPreview),
                        Image.asset(wallpaper),
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
            widget: MechanixFilledButton(
              onPressed: _onSave,
              theme: const MechanixFilledButtonThemeData(
                buttonSize: Size(91, 40),
              ),
              label: "Save",
            ).padOnly(right: 18),
          )
        ],
      ),
    );
  }
}
