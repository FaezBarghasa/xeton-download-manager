package com.xetondownloadmanager.desktop.storage



public  val com.xetondownloadmanager.desktop.storage.CommonData.Companion.`lastSavedLocations`: arrow.optics.Lens<com.xetondownloadmanager.desktop.storage.CommonData, kotlin.collections.List<kotlin.String>>  get() = arrow.optics.Lens(
  get = { commonData: com.xetondownloadmanager.desktop.storage.CommonData -> commonData.`lastSavedLocations` },
  set = { commonData: com.xetondownloadmanager.desktop.storage.CommonData, value: kotlin.collections.List<kotlin.String> ->
  commonData.copy(`lastSavedLocations` = value)
}
)

public  val <__S> arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.storage.CommonData>.`lastSavedLocations`: arrow.optics.Lens<__S, kotlin.collections.List<kotlin.String>>  get() = this + com.xetondownloadmanager.desktop.storage.CommonData.`lastSavedLocations`
public  val <__S> arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.storage.CommonData>.`lastSavedLocations`: arrow.optics.Optional<__S, kotlin.collections.List<kotlin.String>>  get() = this + com.xetondownloadmanager.desktop.storage.CommonData.`lastSavedLocations`
public  val <__S> arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.storage.CommonData>.`lastSavedLocations`: arrow.optics.Traversal<__S, kotlin.collections.List<kotlin.String>>  get() = this + com.xetondownloadmanager.desktop.storage.CommonData.`lastSavedLocations`
