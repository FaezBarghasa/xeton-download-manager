// Xeton Download Manager - Floating Video Sniffer Overlay
// Injects an elegant floating action button over detected video/audio media elements.

(() => {
  if (window.__XETON_SNIFFER_OVERLAY_INJECTED__) return;
  window.__XETON_SNIFFER_OVERLAY_INJECTED__ = true;

  const activeBadges = new WeakMap();

  function attachBadgeToMedia(mediaEl) {
    if (activeBadges.has(mediaEl)) return;

    const badge = document.createElement('div');
    badge.className = 'xeton-sniffer-badge';
    badge.title = 'Download video with Xeton';
    badge.innerHTML = `
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
      </svg>
      <span>Download</span>
    `;

    badge.style.cssText = `
      position: absolute; z-index: 2147483646;
      display: none; align-items: center; gap: 5px;
      padding: 5px 10px; background: rgba(23, 24, 28, 0.92);
      color: #FFF; border: 1px solid rgba(99, 102, 241, 0.4);
      border-radius: 6px; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      font-size: 11px; font-weight: 700; cursor: pointer;
      box-shadow: 0 4px 16px rgba(0,0,0,0.4); backdrop-filter: blur(4px);
      transition: all 150ms ease; user-select: none;
    `;

    badge.onmouseenter = () => {
      badge.style.background = '#6366F1';
      badge.style.borderColor = '#818CF8';
      badge.style.transform = 'scale(1.05)';
    };
    badge.onmouseleave = () => {
      badge.style.background = 'rgba(23, 24, 28, 0.92)';
      badge.style.borderColor = 'rgba(99, 102, 241, 0.4)';
      badge.style.transform = 'scale(1.0)';
    };

    badge.onclick = (e) => {
      e.stopPropagation();
      e.preventDefault();
      const mediaUrl = mediaEl.currentSrc || mediaEl.src;
      if (!mediaUrl) return;

      chrome.runtime.sendMessage({
        action: "send_to_xeton",
        url: mediaUrl,
        title: document.title || "Video Download",
        type: mediaUrl.includes(".m3u8") ? "hls" : "http"
      });

      badge.innerHTML = `<span>Sent to Xeton!</span>`;
      setTimeout(() => {
        badge.innerHTML = `
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
          </svg>
          <span>Download</span>
        `;
      }, 2000);
    };

    document.body.appendChild(badge);
    activeBadges.set(mediaEl, badge);

    function updateBadgePosition() {
      const rect = mediaEl.getBoundingClientRect();
      if (rect.width < 120 || rect.height < 90 || rect.top > window.innerHeight || rect.bottom < 0) {
        badge.style.display = 'none';
        return;
      }
      badge.style.top = `${window.scrollY + rect.top + 10}px`;
      badge.style.left = `${window.scrollX + rect.right - 95}px`;
      badge.style.display = 'flex';
    }

    mediaEl.addEventListener('mouseenter', updateBadgePosition);
    mediaEl.addEventListener('play', updateBadgePosition);
    badge.addEventListener('mouseenter', () => { badge.style.display = 'flex'; });

    window.addEventListener('resize', () => {
      if (badge.style.display === 'flex') updateBadgePosition();
    });
    window.addEventListener('scroll', () => {
      if (badge.style.display === 'flex') updateBadgePosition();
    }, { passive: true });
  }

  function scanMediaElements() {
    document.querySelectorAll('video, audio').forEach(attachBadgeToMedia);
  }

  scanMediaElements();
  const observer = new MutationObserver(scanMediaElements);
  observer.observe(document.body, { childList: true, subtree: true });
})();
