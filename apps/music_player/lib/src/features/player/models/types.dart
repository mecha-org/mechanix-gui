import 'dart:typed_data';
import 'dart:convert';

class SongInfo {
  final String path;
  final String title;
  final String artist;
  final String? album;
  final Duration? duration;
  final Uint8List? artwork; // null when unavailable

  SongInfo({
    required this.path,
    required this.title,
    required this.artist,
    this.album,
    this.duration,
    this.artwork,
  });

  Map<String, dynamic> toJson() => {
    'path': path,
    'title': title,
    'artist': artist,
    'album': album,
    // store duration in milliseconds
    'duration': duration?.inMilliseconds,
    // artwork as base64 string (or null)
    'artwork': artwork != null ? base64Encode(artwork!) : null,
  };

  factory SongInfo.fromJson(Map<String, dynamic> json) {
    final artStr = json['artwork'] as String?;
    return SongInfo(
      path: json['path'] as String,
      title: (json['title'] as String?) ?? '',
      artist: (json['artist'] as String?) ?? '',
      album: json['album'] as String?,
      duration:
          json['duration'] != null
              ? Duration(milliseconds: (json['duration'] as int))
              : null,
      artwork: artStr != null ? base64Decode(artStr) : null,
    );
  }
}

// Supported extensions
const audioExt = ['.mp3', '.wav', '.flac', '.m4a', '.aac', '.ogg', '.opus'];
