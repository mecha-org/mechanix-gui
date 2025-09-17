import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:mechanix_settings/src/features/date_time/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';

class DateTimeSettings extends StatefulWidget {
  const DateTimeSettings({super.key});

  @override
  State<DateTimeSettings> createState() => _DateTimeSettingsState();
}

class _DateTimeSettingsState extends State<DateTimeSettings> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        final currentTimeZoneAbbr = '';

        final month = state.systemDateTime?.month != null
            ? longMonths
                .firstWhere((e) => e.value == state.systemDateTime?.month)
                .label
            : '';

        final hour = state.systemDateTime?.hour != null
            ? hours12Options
                .firstWhere((e) =>
                    e.value == hour24to12(state.systemDateTime?.hour ?? 0))
                .label
            : '';

        final minute = state.systemDateTime?.minute != null
            ? minutesOptions
                .firstWhere((e) => e.value == state.systemDateTime?.minute)
                .label
            : '';

        final meridiem = state.systemDateTime != null
            ? DateFormat('a').format(state.systemDateTime!)
            : '';

        final hourMinute = '$hour : $minute';

        return Scaffold(
          appBar: MechanixNavigationBar(
            title: 'Date and Time',
          ),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  MechanixSimpleList(listItems: [
                    SimpleListItems(
                        title: 'Auto-time',
                        trailing: MechanixSwitch(
                            value: state.autoDateTime,
                            inactiveText: 'ON',
                            onChanged: (value) {
                              context
                                  .read<DateTimeBloc>()
                                  .add(ToggleAutoDateTime(value));
                            })),
                    SimpleListItems(
                      title: 'Set Time',
                      disabled: state.autoDateTime,
                      onTap: () =>
                          Navigator.pushNamed(context, AppRoutes.timeSettings),
                      trailing: Row(
                        children: [
                          CustomTrailingText(
                            title: '$hourMinute $meridiem $currentTimeZoneAbbr',
                          ),
                          IconWidget(
                            iconWidth: 10,
                            iconHeight: 17,
                            iconPath: Images.rightIconArrow,
                          )
                        ],
                      ),
                    ),
                    SimpleListItems(
                      title: 'Set Date',
                      disabled: state.autoDateTime,
                      onTap: () =>
                          Navigator.pushNamed(context, AppRoutes.dateSettings),
                      trailing: Row(
                        children: [
                          CustomTrailingText(
                              title:
                                  '${state.systemDateTime?.day ?? ""} $month ${state.systemDateTime?.year ?? ""}'),
                          IconWidget(
                            iconWidth: 10,
                            iconHeight: 17,
                            iconPath: Images.rightIconArrow,
                          )
                        ],
                      ),
                    ),
                  ])
                ],
              ).padVertical(8),
            ),
          ),
        );
      },
    );
  }
}
