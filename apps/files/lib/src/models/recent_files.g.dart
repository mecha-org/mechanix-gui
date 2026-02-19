// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'recent_files.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class RecentFileAdapter extends TypeAdapter<RecentFile> {
  @override
  final int typeId = 2;

  @override
  RecentFile read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return RecentFile(
      path: fields[0] as String,
      openedAt: fields[1] as DateTime,
    );
  }

  @override
  void write(BinaryWriter writer, RecentFile obj) {
    writer
      ..writeByte(2)
      ..writeByte(0)
      ..write(obj.path)
      ..writeByte(1)
      ..write(obj.openedAt);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RecentFileAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
