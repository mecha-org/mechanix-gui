import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';

class AccentPreviewType {
  const AccentPreviewType({
    required this.accent,
    required this.themePreview,
    required this.wallpaperPreview,
  });

  final MechanixVariant accent;
  final String themePreview;
  final String wallpaperPreview;
}

final List<String> wallpapersList = [
  Images.wallpaper1,
  Images.wallpaper2,
  Images.wallpaper3,
  Images.wallpaper4,
  Images.wallpaper5,
];

final List<AccentPreviewType> accentPreviewImages = [
  const AccentPreviewType(
    accent: MechanixVariant.amber,
    themePreview: Images.themeAmber,
    wallpaperPreview: Images.wallpaperAmber,
  ),
  const AccentPreviewType(
    accent: MechanixVariant.blue,
    themePreview: Images.themeBlue,
    wallpaperPreview: Images.wallpaperBlue,
  ),
  const AccentPreviewType(
    accent: MechanixVariant.coral,
    themePreview: Images.themeCoral,
    wallpaperPreview: Images.wallpaperCoral,
  ),
  const AccentPreviewType(
    accent: MechanixVariant.magenta,
    themePreview: Images.themeMagenta,
    wallpaperPreview: Images.wallpaperMagenta,
  ),
  const AccentPreviewType(
    accent: MechanixVariant.mint,
    themePreview: Images.themeMint,
    wallpaperPreview: Images.wallpaperMint,
  ),
  const AccentPreviewType(
    accent: MechanixVariant.orange,
    themePreview: Images.themeOrange,
    wallpaperPreview: Images.wallpaperOrange,
  ),
  const AccentPreviewType(
    accent: MechanixVariant.purple,
    themePreview: Images.themePurple,
    wallpaperPreview: Images.wallpaperPurple,
  ),
  const AccentPreviewType(
    accent: MechanixVariant.yellow,
    themePreview: Images.themeYellow,
    wallpaperPreview: Images.wallpaperYellow,
  ),
];
