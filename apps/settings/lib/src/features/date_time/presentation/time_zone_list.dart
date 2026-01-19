import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:mechanix_settings/src/features/date_time/models/types.dart';
import 'package:widgets/mechanix.dart';

class TimeZoneList extends StatelessWidget {
  const TimeZoneList({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<DateTimeBloc, DateTimeState, String>(
      selector: (state) => state.selectedTimezone,
      builder: (context, state) {
        return Scaffold(
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  const CustomTitle(title: "Time Zone"),
                  MechanixSelect(
                    options: selectTimeZones,
                    value: state,
                    onChanged: (value) {
                      context
                          .read<DateTimeBloc>()
                          .add(SetSelectedTimezoneEvent(value.value));

                      Navigator.pop(context);
                    },
                  )
                ],
              ),
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
