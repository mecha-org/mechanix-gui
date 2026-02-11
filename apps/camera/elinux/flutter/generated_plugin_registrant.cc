//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <camera_elinux/camera_elinux_plugin.h>
#include <video_player_elinux/video_player_elinux_plugin.h>

void RegisterPlugins(flutter::PluginRegistry* registry) {
  CameraElinuxPluginRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("CameraElinuxPlugin"));
  VideoPlayerElinuxPluginRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("VideoPlayerElinuxPlugin"));
}
