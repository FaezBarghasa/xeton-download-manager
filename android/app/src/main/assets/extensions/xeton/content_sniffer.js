// Xeton Mobile Content Sniffer — detects video/audio tags, HLS playlists, and direct links
(function() {
  const seenUrls = new Set();
  let debounceTimer = null;

  function scanMedia() {
    const mediaElements = document.querySelectorAll("video, audio, source");
    for (const el of mediaElements) {
      const src = el.src || el.currentSrc;
      if (src && src.startsWith("http") && !seenUrls.has(src)) {
        seenUrls.add(src);
        browser.runtime.sendMessage({
          type: "MEDIA_DETECTED",
          url: src,
          mimeType: el.type || (src.includes(".m3u8") ? "application/x-mpegURL" : "video/mp4"),
          title: document.title
        }).catch(() => {
          // Extension bridge might not be ready yet
        });
      }
    }
  }

  function debouncedScan() {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
    debounceTimer = setTimeout(scanMedia, 300);
  }

  // Scan immediately
  scanMedia();

  // Observe DOM changes with 300ms debounce to prevent battery/CPU thrashing on infinite scroll
  const targetNode = document.body || document.documentElement;
  if (targetNode) {
    const observer = new MutationObserver(() => debouncedScan());
    observer.observe(targetNode, { childList: true, subtree: true });
  }
})();
