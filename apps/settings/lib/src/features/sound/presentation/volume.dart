import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/mechanix_simple_list_theme.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';

class VolumeWidget extends StatefulWidget {
  const VolumeWidget({super.key});

  @override
  State<VolumeWidget> createState() => _VolumeWidgetState();
}

class _VolumeWidgetState extends State<VolumeWidget> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(
      builder: (context, state) {
        return Column(
          children: [
            _HeaderWidget(
                    leadingText: 'Output Volume',
                    trailingText:
                        '${((state.outputSoundLevel) * 100).round()} %')
                .padTop(11),
            MechanixSlider.bar(
              initialValue: state.outputSoundLevel,
              onChangeEnd: (value) => context.read<SoundBloc>().add(
                  SetOutputDeviceVolume(
                      state.defaultOutputDevice?.name ?? '', value)),
              // iconPath: Images.volumeOn,
              leftIcon: IconWidget(
                iconColor: context.primaryContainer,
                iconPath: Images.volumeOn,
                iconWidth: 23,
                iconHeight: 22,
                boxHeight: 30,
                boxWidth: 30,
              ),
            ),
            MechanixSimpleList(
              theme: const MechanixSimpleListThemeData(
                widgetMargin: EdgeInsets.only(bottom: 36),
              ),
              listItems: [
                SimpleListItems(
                  title: 'Output Devices',
                  onTap: () => Navigator.pushNamed(
                      context, AppRoutes.soundOutputDevices),
                  trailing: _buildAnchorWidget(
                      context,
                      state.defaultOutputDevice?.description ??
                          'In-Built Speaker'),
                ),
              ],
            ).padTop(4),
            _HeaderWidget(
                leadingText: 'input Volume',
                trailingText: '${((state.inputSoundLevel * 100).round())} %'),
            MechanixSlider.bar(
              initialValue: state.inputSoundLevel,
              onChangeEnd: (value) => context.read<SoundBloc>().add(
                  SetInputDeviceVolume(
                      state.defaultInputDevice?.name ?? '', value)),
              leftIcon: IconWidget(
                iconColor: context.primaryContainer,
                iconPath: Images.micOn,
                iconWidth: 20,
                iconHeight: 28,
                boxHeight: 30,
                boxWidth: 30,
              ),
            ),
            MechanixSimpleList(
              theme: const MechanixSimpleListThemeData(
                widgetMargin: EdgeInsets.only(bottom: 36),
              ),
              listItems: [
                SimpleListItems(
                  title: 'Input Devices',
                  onTap: () =>
                      Navigator.pushNamed(context, AppRoutes.soundInputDevices),
                  trailing: _buildAnchorWidget(
                      context,
                      state.defaultInputDevice?.description ??
                          'In-Built Microphone'),
                ),
              ],
            ).padTop(4),
          ],
        );
      },
    );
  }
}

class _HeaderWidget extends StatelessWidget {
  const _HeaderWidget({
    required this.leadingText,
    required this.trailingText,
  });

  final String leadingText;

  final String trailingText;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        CustomTrailingText(title: leadingText),
        CustomTrailingText(title: trailingText),
      ],
    ).padBottom(8);
  }
}

Widget _buildAnchorWidget(BuildContext context, String title) {
  return Row(
    mainAxisAlignment: MainAxisAlignment.center,
    children: [
      SizedBox(
        width: 250,
        child: CustomTrailingText(
          title: title,
          titleStyle: const TextStyle(
            overflow: TextOverflow.ellipsis,
          ),
        ),
      ).padRight(8),
      IconWidget(
        iconWidth: 8,
        iconHeight: 14,
        boxWidth: 20,
        boxHeight: 20,
        iconColor: context.outlineVariant,
        iconPath: Images.rightIconArrow,
      ),
    ],
  );
}
