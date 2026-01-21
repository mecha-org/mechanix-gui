import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/features/appearance/bloc/appearance_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/models/types.dart';
import 'package:widgets/extension.dart';

class WallpaperGallery extends StatelessWidget {
  const WallpaperGallery({super.key});

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 20,
      runSpacing: 20,
      children: wallpapersList
          .map((imagePath) => _WallpaperImages(
                imagePath: imagePath,
              ))
          .toList(),
    );
  }
}

class _WallpaperImages extends StatelessWidget {
  const _WallpaperImages({
    required this.imagePath,
  });

  final String imagePath;

  void _onIconTap(BuildContext context) {
    context.read<AppearanceBloc>().add(SetWallpaperEvent(imagePath));
    Navigator.pushNamed(context, AppRoutes.wallpaperPreview);
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () => _onIconTap(context),
      child: BlocSelector<AppearanceBloc, AppearanceState, String>(
        selector: (state) {
          return state.appliedWallpaper;
        },
        builder: (context, file) {
          return Container(
            width: 153,
            height: 176,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(8),
              border: file == imagePath
                  ? Border.all(
                      color: context.primary,
                      style: BorderStyle.solid,
                      width: 2,
                    )
                  : null,
            ),
            child: Image.asset(imagePath),
          );
        },
      ),
    );
  }
}
