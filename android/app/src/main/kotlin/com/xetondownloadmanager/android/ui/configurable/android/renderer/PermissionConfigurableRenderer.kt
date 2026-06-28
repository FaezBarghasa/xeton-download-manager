package com.xetondownloadmanager.android.ui.configurable.android.renderer

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import com.xetondownloadmanager.android.pages.onboarding.permissions.AppPermissionState
import com.xetondownloadmanager.android.pages.onboarding.permissions.rememberAppPermissionState
import com.xetondownloadmanager.android.ui.configurable.ConfigTemplate
import com.xetondownloadmanager.android.ui.configurable.NextIcon
import com.xetondownloadmanager.android.ui.configurable.TitleAndDescription
import com.xetondownloadmanager.android.ui.configurable.android.item.PermissionConfigurable
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableUiProps
import com.xetondownloadmanager.shared.util.ui.LocalContentColor
import com.xetondownloadmanager.shared.util.ui.myColors
import com.xeton.util.compose.asStringSource

object PermissionConfigurableRenderer : ConfigurableRenderer<PermissionConfigurable> {
    @Composable
    override fun RenderConfigurable(
        configurable: PermissionConfigurable,
        configurableUiProps: ConfigurableUiProps
    ) {
        val permission by configurable.stateFlow.collectAsState()
        val permissionState = rememberAppPermissionState(permission) { result ->

        }

        RenderPermissionConfigurable(
            cfg = configurable,
            configurableUiProps = configurableUiProps,
            permissionState = permissionState
        )
    }

    @Composable
    fun RenderPermissionConfigurable(
        cfg: PermissionConfigurable,
        configurableUiProps: ConfigurableUiProps,
        permissionState: AppPermissionState,
    ) {
        ConfigTemplate(
            modifier = configurableUiProps.modifier
                .clickable {
                    permissionState.launchRequest()
                }
                .padding(configurableUiProps.itemPaddingValues),
            title = {
                TitleAndDescription(
                    cfg = cfg,
                    describe = true,
                    describeContent = if (permissionState.isGranted) {
                        Res.string.permission_granted
                    } else {
                        Res.string.permission_not_granted
                    }.asStringSource().rememberString(),
                    describeWrapper = { content ->
                        val contentColor =
                            if (permissionState.isGranted) myColors.success else myColors.warning
                        CompositionLocalProvider(
                            LocalContentColor provides contentColor
                        ) {
                            content()
                        }
                    }
                )
            },
            value = {
                NextIcon()
            }
        )
    }
}
