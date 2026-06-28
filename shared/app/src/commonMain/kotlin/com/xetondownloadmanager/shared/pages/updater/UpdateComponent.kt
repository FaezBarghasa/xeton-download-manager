package com.xetondownloadmanager.shared.pages.updater

import com.xetondownloadmanager.UpdateCheckStatus
import com.xetondownloadmanager.shared.util.AppVersion
import com.xetondownloadmanager.shared.util.BaseComponent
import com.xetondownloadmanager.UpdateManager
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.pagemanager.NotificationSender
import com.xetondownloadmanager.shared.ui.widget.MessageDialogType
import com.xetondownloadmanager.shared.util.mvi.ContainsEffects
import com.xetondownloadmanager.shared.util.mvi.supportEffects
import com.arkivanov.decompose.ComponentContext
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.update
import org.koin.core.component.KoinComponent
import com.xeton.util.compose.asStringSource
import kotlinx.coroutines.flow.drop
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

class UpdateComponent(
    ctx: ComponentContext,
    private val notificationSender: NotificationSender,
    private val updateManager: UpdateManager,
) : BaseComponent(ctx),
    ContainsEffects<UpdateComponent.Effects> by supportEffects(),
    KoinComponent {

    fun isUpdateSupported(): Boolean {
        return updateManager.isUpdateSupported()
    }

    val currentVersion = AppVersion.get()
    val showNewUpdate = MutableStateFlow(false)
    val newVersionData = updateManager.newVersionData
    private var updateApplierJob: Job? = null

    val updateCheckStatus = updateManager.updateCheckStatus

    fun performUpdate() {
        updateApplierJob?.cancel()
        updateApplierJob = scope.launch {
            try {
                updateManager.update()
            } catch (e: Exception) {
                showMessage(e)
            }
        }
    }

    private fun showMessage(e: Exception) {
        e.printStackTrace()
        notificationSender.sendDialogNotification(
            Res.string.update_error.asStringSource(),
            e.localizedMessage.orEmpty().asStringSource(),
            type = MessageDialogType.Error,
        )
    }

    fun showNewUpdate() {
        showNewUpdate.update { true }
    }

    fun requestCheckForUpdate() {
        scope.launch {
            updateManager.checkForUpdate()
        }
    }

    fun requestClose() {
        showNewUpdate.update { false }
    }

    init {
        updateCheckStatus
            .drop(1) // state flow
            .onEach {
                when (it) {
                    UpdateCheckStatus.Checking -> {
                        sendEffect(Effects.CheckingForUpdate)
                    }

                    is UpdateCheckStatus.Error -> {
                        sendEffect(Effects.Error(it.e))
                    }

                    UpdateCheckStatus.NewUpdate -> {
                        sendEffect(Effects.NewUpdate)
                        showNewUpdate()
                    }

                    UpdateCheckStatus.NoUpdate -> {
                        sendEffect(Effects.NoUpdate)
                    }

                    UpdateCheckStatus.IDLE -> {

                    }
                }
            }.launchIn(scope)
    }

    sealed interface Effects {
        data object CheckingForUpdate : Effects
        data object NoUpdate : Effects
        data object NewUpdate : Effects
        data class Error(val throwable: Throwable) : Effects
    }
}
