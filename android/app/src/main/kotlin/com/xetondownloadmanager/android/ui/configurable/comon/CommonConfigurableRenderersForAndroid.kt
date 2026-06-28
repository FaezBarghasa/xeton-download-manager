package com.xetondownloadmanager.android.ui.configurable.comon

import com.xetondownloadmanager.android.ui.configurable.comon.renderer.BooleanConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.DayOfWeekConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.EnumConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.FileChecksumConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.FloatConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.FolderConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.IntConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.LongConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.NavigatableConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.ProxyConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.SoundConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.SpeedLimitConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.StringConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.ThemeConfigurableRenderer
import com.xetondownloadmanager.android.ui.configurable.comon.renderer.TimeConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.CommonConfigurableRenderers

val CommonConfigurableRenderersForAndroid = CommonConfigurableRenderers(
    booleanConfigurableRenderer = BooleanConfigurableRenderer,
    dayOfWeekConfigurableRenderer = DayOfWeekConfigurableRenderer,
    fileChecksumConfigurableRenderer = FileChecksumConfigurableRenderer,
    floatConfigurableRenderer = FloatConfigurableRenderer,
    folderConfigurableRenderer = FolderConfigurableRenderer,
    intConfigurableRenderer = IntConfigurableRenderer,
    longConfigurableRenderer = LongConfigurableRenderer,
    perHostSettingsConfigurableRenderer = NavigatableConfigurableRenderer,
    enumConfigurableRenderer = EnumConfigurableRenderer,
    speedConfigurableRenderer = SpeedLimitConfigurableRenderer,
    stringConfigurableRenderer = StringConfigurableRenderer,
    themeConfigurableRenderer = ThemeConfigurableRenderer,
    timeConfigurableRenderer = TimeConfigurableRenderer,
    proxyConfigurableRenderer = ProxyConfigurableRenderer,
    soundConfigurableRenderer = SoundConfigurableRenderer,
)
