package com.xetondownloadmanager.desktop.storage



public  val com.xetondownloadmanager.desktop.storage.PageStatesModel.Companion.`home`: arrow.optics.Lens<com.xetondownloadmanager.desktop.storage.PageStatesModel, com.xetondownloadmanager.desktop.pages.home.HomePageStateToPersist>  get() = arrow.optics.Lens(
  get = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel -> pageStatesModel.`home` },
  set = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel, value: com.xetondownloadmanager.desktop.pages.home.HomePageStateToPersist ->
  pageStatesModel.copy(`home` = value)
}
)

public  val com.xetondownloadmanager.desktop.storage.PageStatesModel.Companion.`settings`: arrow.optics.Lens<com.xetondownloadmanager.desktop.storage.PageStatesModel, com.xetondownloadmanager.desktop.pages.settings.SettingPageStateToPersist>  get() = arrow.optics.Lens(
  get = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel -> pageStatesModel.`settings` },
  set = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel, value: com.xetondownloadmanager.desktop.pages.settings.SettingPageStateToPersist ->
  pageStatesModel.copy(`settings` = value)
}
)

public  val com.xetondownloadmanager.desktop.storage.PageStatesModel.Companion.`downloadPage`: arrow.optics.Lens<com.xetondownloadmanager.desktop.storage.PageStatesModel, com.xetondownloadmanager.desktop.pages.singleDownloadPage.SingleDownloadPageStateToPersist>  get() = arrow.optics.Lens(
  get = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel -> pageStatesModel.`downloadPage` },
  set = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel, value: com.xetondownloadmanager.desktop.pages.singleDownloadPage.SingleDownloadPageStateToPersist ->
  pageStatesModel.copy(`downloadPage` = value)
}
)

public  val com.xetondownloadmanager.desktop.storage.PageStatesModel.Companion.`global`: arrow.optics.Lens<com.xetondownloadmanager.desktop.storage.PageStatesModel, com.xetondownloadmanager.desktop.storage.CommonData>  get() = arrow.optics.Lens(
  get = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel -> pageStatesModel.`global` },
  set = { pageStatesModel: com.xetondownloadmanager.desktop.storage.PageStatesModel, value: com.xetondownloadmanager.desktop.storage.CommonData ->
  pageStatesModel.copy(`global` = value)
}
)

public  val <__S> arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`home`: arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.pages.home.HomePageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`home`
public  val <__S> arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`home`: arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.pages.home.HomePageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`home`
public  val <__S> arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`home`: arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.pages.home.HomePageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`home`

public  val <__S> arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`settings`: arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.pages.settings.SettingPageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`settings`
public  val <__S> arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`settings`: arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.pages.settings.SettingPageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`settings`
public  val <__S> arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`settings`: arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.pages.settings.SettingPageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`settings`

public  val <__S> arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`downloadPage`: arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.pages.singleDownloadPage.SingleDownloadPageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`downloadPage`
public  val <__S> arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`downloadPage`: arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.pages.singleDownloadPage.SingleDownloadPageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`downloadPage`
public  val <__S> arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`downloadPage`: arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.pages.singleDownloadPage.SingleDownloadPageStateToPersist>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`downloadPage`

public  val <__S> arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`global`: arrow.optics.Lens<__S, com.xetondownloadmanager.desktop.storage.CommonData>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`global`
public  val <__S> arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`global`: arrow.optics.Optional<__S, com.xetondownloadmanager.desktop.storage.CommonData>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`global`
public  val <__S> arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.storage.PageStatesModel>.`global`: arrow.optics.Traversal<__S, com.xetondownloadmanager.desktop.storage.CommonData>  get() = this + com.xetondownloadmanager.desktop.storage.PageStatesModel.`global`
