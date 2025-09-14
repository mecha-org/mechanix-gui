import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/about/data/about_repository.dart';

part 'about_event.dart';
part 'about_state.dart';

class AboutBloc extends Bloc<AboutEvent, AboutState> {
  final AboutRepository aboutRepository;

  AboutBloc({required this.aboutRepository}) : super(AboutState()) {
    on<InitializeAbout>(_onInitializeABout);
  }

  Future<void> _onInitializeABout(
      InitializeAbout event, Emitter<AboutState> emit) async {
    final about = await aboutRepository.getDeviceDetails();
    emit(state.copyWith(
      hostname: about.hostname,
      hardwareModel: about.hardwareModel,
      hardwareSerial: about.hardwareSerial,
      firmwareVersion: about.firmwareVersion,
      kernelRelease: about.kernelRelease,
      operatingSystemName: about.operatingSystemName,
    ));
  }
}
