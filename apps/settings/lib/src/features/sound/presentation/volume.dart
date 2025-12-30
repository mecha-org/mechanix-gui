import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

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
                .padTop(8),
            Container(
              margin: Spacing.bottom(40),
              child: MechanixSlider.bar(
                initialValue: state.outputSoundLevel,
                onChangeEnd: (value) => context.read<SoundBloc>().add(
                    SetOutputDeviceVolume(
                        state.defaultOutputDevice?.name ?? '', value)),
                leftIcon: IconWidget(
                  iconColor: Colors.white,
                  iconPath: Images.micOn,
                  iconWidth: 23,
                  iconHeight: 22,
                  boxHeight: 30,
                  boxWidth: 30,
                ),
              ),
            ),
            _HeaderWidget(
                leadingText: 'input Volume',
                trailingText: '${((state.inputSoundLevel * 100).round())} %'),
            Container(
              margin: Spacing.bottom(40),
              child: MechanixSlider.bar(
                initialValue: state.inputSoundLevel,
                onChangeEnd: (value) => context.read<SoundBloc>().add(
                    SetInputDeviceVolume(
                        state.defaultInputDevice?.name ?? '', value)),
                leftIcon: IconWidget(
                  iconColor: Colors.white,
                  iconPath: Images.volumeOn,
                  iconWidth: 20,
                  iconHeight: 28,
                  boxHeight: 30,
                  boxWidth: 30,
                ),
              ),
            ),
            MechanixSectionList(title: 'Device', sectionListItems: [
              SectionListItems(
                  title: 'Output Devices',
                  onTap: () => Navigator.pushNamed(
                      context, AppRoutes.soundOutputDevices),
                  trailing: SizedBox(
                    width: 200,
                    child: CustomTrailingText(
                      title: state.defaultOutputDevice?.description ??
                          'In-Built Speaker',
                      titleStyle: TextStyle(overflow: TextOverflow.ellipsis),
                    ).padRight(8),
                  )),
              SectionListItems(
                  title: 'Input Devices',
                  onTap: () =>
                      Navigator.pushNamed(context, AppRoutes.soundInputDevices),
                  trailing: SizedBox(
                    width: 200,
                    child: CustomTrailingText(
                      title: state.defaultInputDevice?.description ??
                          'In-Built Microphone',
                      titleStyle: TextStyle(
                        overflow: TextOverflow.ellipsis,
                      ),
                    ).padRight(8),
                  ))
            ])
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
