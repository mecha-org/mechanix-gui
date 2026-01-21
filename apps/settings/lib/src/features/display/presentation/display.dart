import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/display/bloc/display_bloc.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';

class DisplayPage extends StatefulWidget {
  const DisplayPage({super.key});

  @override
  State<DisplayPage> createState() => _DisplayPageState();
}

class _DisplayPageState extends State<DisplayPage> {
  String screenOffTime = "";
  String lockScreenTime = "";

  @override
  void initState() {
    super.initState();

    context.read<DisplayBloc>().add(GetDefaultSettingsEvent());
  }

  void changeBrightnessAuto(bool value) {
    context.read<DisplayBloc>().add(SetAutoBrightnessEvent(value));
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DisplayBloc, DisplayState>(
      builder: (context, state) {
        return Scaffold(
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  const CustomTitle(title: "Display"),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        'Brightness',
                        style: context.textTheme.labelLarge,
                      ),
                      Text(
                        '${(state.brightness * 100).round()} %',
                        style: context.textTheme.labelLarge,
                      )
                    ],
                  ).padVertical(8),
                  BlocListener<DisplayBloc, DisplayState>(
                    listener: (context, state) {
                      // TODO: implement listener
                    },
                    listenWhen: (previous, current) =>
                        previous.brightness != current.brightness,
                    child: MechanixSlider.dot(
                      isDotSlider: true,
                      initialValue: state.brightness,
                      onChangeEnd: (value) => context
                          .read<DisplayBloc>()
                          .add(SetBrightnessEvent(value)),
                      onChanged: (value) => context
                          .read<DisplayBloc>()
                          .add(SetBrightnessChangeEvent(value)),
                      leftIcon: IconWidget(
                        iconColor: context.primaryContainer,
                        iconPath: Images.sunIcon,
                        iconWidth: 20,
                        iconHeight: 28,
                        boxHeight: 30,
                        boxWidth: 30,
                      ),
                    ),
                  ),
                  MechanixSectionList(
                      title: 'Display Options',
                      physics: const BouncingScrollPhysics(),
                      sectionListItems: [
                        SectionListItems(
                            title: 'Auto Brightness',
                            defaultTrailingIcon: false,
                            trailing: MechanixSwitch(
                              activeText: 'OFF',
                              inactiveText: 'ON',
                              value: state.isAutoBrightness,
                              onChanged: changeBrightnessAuto,
                            )),
                        SectionListItems(
                          title: 'Screen Off Time',
                          onTap: () async {
                            // final result = await
                            Navigator.pushNamed(
                              context,
                              AppRoutes.displayScreenOffTime,
                              arguments: {'screenOffTime': state.screenTimeout},
                            );
                          },
                          trailing: Text(displayLabel(state.screenTimeout))
                              .padRight(8),
                        ),
                      ]).padTop(40),
                ],
              ).padTop(8),
            ),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
          ),
        );
      },
    );
  }
}
