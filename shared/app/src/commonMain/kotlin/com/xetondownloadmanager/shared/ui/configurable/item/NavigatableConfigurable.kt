package com.xetondownloadmanager.shared.ui.configurable.item

import com.xetondownloadmanager.shared.ui.configurable.Configurable
import com.xeton.util.compose.StringSource
import com.xeton.util.compose.asStringSource
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

class NavigatableConfigurable(
    title: StringSource,
    description: StringSource,
    val onRequestNavigate: () -> Unit,
    enabled: StateFlow<Boolean> = DefaultEnabledValue,
    visible: StateFlow<Boolean> = DefaultVisibleValue,
) : Configurable<Unit>(
    title = title,
    description = description,
    backedBy = MutableStateFlow(Unit),
    describe = { "".asStringSource() },
    enabled = enabled,
    visible = visible,
) {
    object Key : Configurable.Key

    override fun getKey() = Key
}
