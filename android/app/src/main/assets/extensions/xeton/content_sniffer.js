// Xeton Mobile Content Sniffer — detects video/audio tags, HLS playlists, and direct links
(function() {
  function scanMedia() {
    // 1. Scan <video> and <audio> tags
    const mediaElements = document.querySelectorAll("video, audio, source");
    for (const el of mediaElements) {
      const src = el.src || el.currentSrc;
      if (src && src.startsWith("http")) {
        browser.runtime.sendMessage({
          type: "MEDIA_DETECTED",
          url: src,
          mimeType: el.type || (src.includes(".m3u8") ? "application/x-mpegURL" : "video/mp4"),
          title: document.title
        });
      }
    }
  }

  // Scan immediately and observe DOM changes
  scanMedia();
  const observer = new MutationObserver(() => scanMedia());
  observer.observe(document.body || document.documentElement, { childList: true, subtree: true });
})();
