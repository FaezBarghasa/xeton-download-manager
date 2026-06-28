package com.xetondownloadmanager.integration

import kotlinx.serialization.Serializable

@Serializable
data class ApiQueueModel(
        val id: Long,
        val name: String,
)
