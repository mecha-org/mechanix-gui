import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/right_icon_arrow_widget.dart';
import 'package:mechanix_settings/src/features/appearance/bloc/appearance_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/models/types.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/set_up_theme.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/set_up_wallpaper.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

class Appearance extends StatefulWidget {
  const Appearance({super.key});

  @override
  State<Appearance> createState() => _AppearanceState();
}

class _AppearanceState extends State<Appearance> {
  void onThemeTap(BuildContext context) {
    final bloc = context.read<AppearanceBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: const SetUpTheme(),
        ),
      ),
    );
  }

  void onWallpaperTap(BuildContext context) {
    final bloc = context.read<AppearanceBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: const SetUpWallpaper(),
        ),
      ),
    );
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _precacheAllThemeImages();
  }

  Future<void> _precacheAllThemeImages() async {
    final allPreviewPaths =
        accentPreviewImages.map((e) => e.themePreview).toList();

    if (mounted) {
      for (final wallpaper in wallpapersList) {
        await precacheImage(
          AssetImage(wallpaper.wallpaper),
          context,
          size: const Size(153, 176),
        );
      }

      for (final path in allPreviewPaths) {
        await precacheImage(
          AssetImage(path),
          context,
          size: const Size(500, 267),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: ContainerWidget(
        child: Column(
          children: [
            const CustomTitle(title: 'Appearance'),
            MechanixSimpleList(
              listItems: [
                SimpleListItems(
                  title: 'Dark Mode',
                  disabled: true,
                  trailing:
                      BlocSelector<AppearanceBloc, AppearanceState, ThemeMode>(
                    selector: (state) {
                      return state.themeMode;
                    },
                    builder: (context, state) {
                      return MechanixSwitch(
                        value: state == ThemeMode.dark,
                        activeText: "OFF",
                        inactiveText: "ON",
                        style: MechanixSwitchStyle(
                            activeThumbColor:
                                context.primary.withAlpha((0.4 * 255).toInt())),
                        onChanged: (_) {
                          // TODO: Implement dart/ light mode switch changes
                        },
                      );
                    },
                  ),
                ),
                SimpleListItems(
                  title: 'Set Wallpaper',
                  onTap: () => onWallpaperTap(context),
                  trailing: const RightIconArrowWidget(),
                ),
                SimpleListItems(
                  title: 'Theme',
                  onTap: () => onThemeTap(context),
                  trailing: BlocSelector<AppearanceBloc, AppearanceState,
                      MechanixVariant>(
                    selector: (state) => state.appliedVariant,
                    builder: (context, state) {
                      return Row(
                        children: [
                          Container(
                            width: 16.25,
                            height: 16.25,
                            margin: const EdgeInsets.only(right: 10),
                            decoration: BoxDecoration(
                              shape: BoxShape.circle,
                              color: state.color,
                            ),
                          ).padRight(8),
                          Text(
                            state.name,
                          ).padRight(8),
                          const RightIconArrowWidget(),
                        ],
                      );
                    },
                  ),
                ),
              ],
            )
          ],
        ).padTop(8),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
      ),
    );
  }
}
