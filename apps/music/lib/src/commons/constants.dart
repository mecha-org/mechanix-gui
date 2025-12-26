class TableName {
  static const String songsInfoTable = "musicSongsTablev4";
  static const String recentlyPlayedTable = "recentlyPlayedTablev2";
  static const String playlistTable = "playlistTablev1";
}

class Constants {
  static const double recentlyPlayedLimit = 9;
  static const double playlistLimit = 5;
  static const double maxSongsPerPlaylist = 5;
  static const Duration debounceDuration = Duration(microseconds: 300);
}

const audioExt = ['.mp3', '.wav', '.flac', '.m4a', '.aac', '.ogg', '.opus'];
