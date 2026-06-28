package com.xetondownloadmanager.shared.util.downloaderror.definederrors

import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorMapper
import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorMapper.Companion.createErrorReason
import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorReason
import com.xeton.downloader.exception.ServerPartIsNotTheSameAsWeExpectException
import com.xeton.downloader.exception.ServerResumeSupportChangeException
import com.xeton.util.compose.asStringSource

object ResumeSupportChangedDownloadErrorMapper : DownloadErrorMapper {
    override fun accept(throwable: Throwable): Boolean {
        return throwable is ServerResumeSupportChangeException
    }

    override fun getReason(throwable: Throwable): DownloadErrorReason {
        return createErrorReason(
            title = Res.string.download_error_reason_server_resume_change_title.asStringSource().getString(),
            description = Res.string.download_error_reason_server_resume_change_description.asStringSource()
                .getString(),
            suggestion = Res.string.download_error_reason_server_resume_change_suggestion.asStringSource().getString(),
            throwable = throwable,
        )
    }
}
