import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/appearance/bloc/appearance_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';

class SetUpTheme extends StatefulWidget {
  const SetUpTheme({super.key});

  @override
  State<SetUpTheme> createState() => _SetUpThemeState();
}

class _SetUpThemeState extends State<SetUpTheme> {
  void _onSave(BuildContext context) {
    context.read<AppearanceBloc>().add(ApplyThemeVariantEvent());
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    final variants = MechanixVariant.getAllVariants();

    return Scaffold(
      body: SingleChildScrollView(
        physics: const BouncingScrollPhysics(),
        child: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const CustomTitle(title: "Set Theme"),
              const CustomTrailingText(
                title: 'Set accent color',
                textAlign: TextAlign.start,
              ).padBottom(8),
              Center(
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceAround,
                  children: List.generate(
                    variants.length,
                    (index) => AccentColorTile(variant: variants[index]),
                  ),
                ),
              ),
              BlocSelector<AppearanceBloc, AppearanceState, MechanixVariant>(
                selector: (state) => state.variant,
                builder: (context, accent) {
                  final accentImage = accentPreviewImages
                      .firstWhere((variant) => variant.accent == accent);

                  return Align(
                    alignment: Alignment.topCenter,
                    child: Container(
                      key: ValueKey(accent),
                      width: 500,
                      height: 267,
                      margin: const EdgeInsets.only(top: 44),
                      child: Image.asset(
                        accentImage.themePreview,
                        key: ValueKey(accentImage.themePreview),
                        width: 500,
                        height: 267,
                      ),
                    ),
                  );
                },
              ),
            ],
          ),
        ),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
        anchorWidget: [
          BottomBarButton.widget(
            widget: IconButton(
              onPressed: () => _onSave(context),
              icon: const IconWidget(iconPath: Images.submit),
            ).padOnly(right: 8),
          )
        ],
      ),
    );
  }
}

class AccentColorTile extends StatelessWidget {
  const AccentColorTile({super.key, required this.variant});
  final MechanixVariant variant;

  @override
  Widget build(BuildContext context) {
    return BlocSelector<AppearanceBloc, AppearanceState, bool>(
      selector: (state) => state.variant == variant,
      builder: (context, isActive) {
        return GestureDetector(
          onTap: () {
            context.read<AppearanceBloc>().add(SetThemeVariantEvent(variant));
          },
          child: Container(
            height: 56,
            width: 56,
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(
              color: context.secondary,
              border: Border.all(
                color: isActive ? variant.color : context.secondaryContainer,
                style: BorderStyle.solid,
                width: 1,
              ),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Container(
              width: 40,
              height: 40,
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(4),
                color: variant.color,
              ),
            ),
          ),
        );
      },
    );
  }
}
