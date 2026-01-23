import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/bloc/appearance_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/models/types.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/wallpaper_preview.dart';
import 'package:widgets/extension.dart';

class WallpaperGallery extends StatefulWidget {
  const WallpaperGallery({super.key});

  @override
  State<WallpaperGallery> createState() => _WallpaperGalleryState();
}

class _WallpaperGalleryState extends State<WallpaperGallery> {
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _precacheAllThemeImages();
  }

  Future<void> _precacheAllThemeImages() async {
    if (mounted) {
      for (final wallpaper in wallpapersList) {
        await precacheImage(
          AssetImage(wallpaper.wallpaperPreview),
          context,
          size: const Size(153, 176),
        );
      }
    }
  }

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

  final WallpaperPreviewType imagePath;

  void _onIconTap(BuildContext context) {
    context.read<AppearanceBloc>().add(SetWallpaperEvent(imagePath));
    final bloc = context.read<AppearanceBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: const WallpaperPreview(),
        ),
      ),
    );
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
              image: DecorationImage(
                image: AssetImage(imagePath.wallpaper),
                fit: BoxFit.cover,
              ),
              borderRadius: BorderRadius.circular(8),
              border: file == imagePath.wallpaper
                  ? Border.all(
                      color: context.primary,
                      style: BorderStyle.solid,
                      width: 2,
                    )
                  : null,
            ),
            // child: Image.asset(imagePath.wallpaper,),
          );
        },
      ),
    );
  }
}
