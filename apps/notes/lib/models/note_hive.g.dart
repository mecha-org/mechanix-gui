// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'note_hive.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class NoteHiveAdapter extends TypeAdapter<NoteHive> {
  @override
  final int typeId = 0;

  @override
  NoteHive read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return NoteHive(
      id: fields[0] as String,
      title: fields[1] as String,
      content: fields[2] as String,
      createdAt: fields[3] as DateTime,
      updatedAt: fields[4] as DateTime,
      plainText: fields[5] as String,
      isPinned: fields[6] as bool,
      tag: fields[7] as String,
    );
  }

  @override
  void write(BinaryWriter writer, NoteHive obj) {
    writer
      ..writeByte(8)
      ..writeByte(0)
      ..write(obj.id)
      ..writeByte(1)
      ..write(obj.title)
      ..writeByte(2)
      ..write(obj.content)
      ..writeByte(3)
      ..write(obj.createdAt)
      ..writeByte(4)
      ..write(obj.updatedAt)
      ..writeByte(5)
      ..write(obj.plainText)
      ..writeByte(6)
      ..write(obj.isPinned)
      ..writeByte(7)
      ..write(obj.tag);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NoteHiveAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
