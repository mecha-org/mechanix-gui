part of 'about_bloc.dart';

class AboutState extends Equatable {
  const AboutState({
    this.hostname = '',
    this.hardwareModel = '',
    this.hardwareSerial = '',
    this.firmwareVersion = '',
    this.kernelRelease = '',
    this.operatingSystemName = '',
  });

  final String hostname;
  final String hardwareModel;
  final String hardwareSerial;
  final String firmwareVersion;
  final String kernelRelease;
  final String operatingSystemName;

  AboutState copyWith({
    String? hostname,
    String? hardwareModel,
    String? hardwareSerial,
    String? firmwareVersion,
    String? kernelRelease,
    String? operatingSystemName,
  }) {
    return AboutState(
      hostname: hostname ?? this.hostname,
      hardwareModel: hardwareModel ?? this.hardwareModel,
      hardwareSerial: hardwareSerial ?? this.hardwareSerial,
      firmwareVersion: firmwareVersion ?? this.firmwareVersion,
      kernelRelease: kernelRelease ?? this.kernelRelease,
      operatingSystemName: operatingSystemName ?? this.operatingSystemName,
    );
  }

  @override
  List<Object> get props => [];
}
