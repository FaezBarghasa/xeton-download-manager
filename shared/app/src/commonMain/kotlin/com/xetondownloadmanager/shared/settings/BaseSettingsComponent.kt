package com.xetondownloadmanager.shared.settings

import com.xetondownloadmanager.shared.ui.configurable.ConfigurableGroup
import com.xetondownloadmanager.shared.util.BaseComponent
import com.xetondownloadmanager.shared.util.mvi.ContainsEffects
import com.xetondownloadmanager.shared.util.mvi.supportEffects
import com.arkivanov.decompose.ComponentContext
import kotlinx.coroutines.flow.StateFlow

abstract class BaseSettingsComponent(
    context: ComponentContext
) : BaseComponent(
    context
),
    ContainsEffects<BaseSettingsComponent.Effects> by supportEffects() {
    abstract val configurables: StateFlow<List<ConfigurableGroup>>

    sealed interface Effects {
        interface Platform : Effects
    }
}
