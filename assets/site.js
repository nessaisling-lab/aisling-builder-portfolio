// Progressive enhancement only. Every page works with this file blocked.
(function () {

  // Links people already have (an email, a résumé) point at the old one-page site's
  // anchors, or at the About page that folded into Home. Send them where it lives now.
  var moved = {
    '#projects': 'work/', '#founder': 'work/nisaba/', '#experience': 'experience/',
    '#documents': 'resume/', '#contact': 'contact/', '#about': ''
  };
  if (document.querySelector('.room') && moved[location.hash] !== undefined) {
    location.replace(moved[location.hash] || './');
    return;
  }
  var aboutMoved = { '#experience': 'experience/', '#education': 'school/', '#stack': 'experience/#stack', '#contact': 'contact/' };
  if (document.querySelector('.moved')) {
    location.replace('../' + (aboutMoved[location.hash] || ''));
    return;
  }

  // Less motion if the reader asked for it, in Reading settings or in their system settings.
  var calm = function () {
    var m = document.documentElement.dataset.motion;
    return m === 'reduce' || (m !== 'full' && matchMedia('(prefers-reduced-motion: reduce)').matches);
  };

  // Reading settings (text size, spacing, contrast, day or evening, motion, finish), from
  // the brand kit. Without the kit's script the slot stays empty and hidden.
  var slot = document.querySelector('.settings-slot');
  if (slot && window.NessBrand) NessBrand.settings.mount(slot);

  // Projects: the hall. Pressing a tile in the select strip puts that build on show: its
  // walkthrough on the far wall, its spec sheet, its model on the turntable, its door raised.
  // The arrow keys step through the builds and the class tabs narrow the strip. Without this
  // each tile is a plain link to its build and the capstone stays on show; a modified click
  // (new tab, new window) still opens the link.
  var hall = document.querySelector('.hall');
  // A figure ("1,250+", "15.6k", "180 / 223") counts its first number up from zero, the rest of
  // it unchanged, then is left exactly as written.
  var countUp = function (el) {
    var text = el.dataset.v || (el.dataset.v = el.textContent);
    var m = text.match(/[\d,]*\.?\d+/);
    if (!m) return;
    var to = parseFloat(m[0].replace(/,/g, '')), dp = (m[0].split('.')[1] || '').length, start = null;
    var step = function (now) {
      if (start === null) start = now;
      var t = Math.min((now - start) / 650, 1), v = to * (1 - Math.pow(1 - t, 3));
      var s = v.toFixed(dp);
      if (m[0].indexOf(',') >= 0) s = Number(s).toLocaleString('en-US', { minimumFractionDigits: dp });
      el.textContent = t < 1 ? text.replace(m[0], s) : text;
      if (t < 1) requestAnimationFrame(step);
    };
    requestAnimationFrame(step);
  };
  if (hall) {
    var tiles = [].slice.call(hall.querySelectorAll('.tile'));
    hall.querySelectorAll('.keys').forEach(function (k) { k.hidden = false; });
    var slugOf = function (a) { return (a.getAttribute('href').match(/work\/([^/]+)\/$/) || [])[1]; };
    var shown = function (a) { return !a.parentNode.hidden; };
    var show = function (slug, focus) {
      var panel = slug && document.getElementById('show-' + slug);
      if (!panel || !panel.classList.contains('show')) return false;
      hall.querySelectorAll('.show').forEach(function (s) {
        if (s === panel) return;
        s.hidden = true;
        var v = s.querySelector('video');
        if (v) v.pause();
      });
      panel.hidden = false;
      tiles.forEach(function (a) { a.setAttribute('aria-current', slugOf(a) === slug ? 'true' : 'false'); });
      // The bay painted on the floor is this build's, and its spec figures count up into place.
      var bay = document.getElementById('bay-n'), tn = panel.querySelector('.bay b');
      if (bay && tn) bay.textContent = 'BAY ' + tn.textContent;
      if (!calm()) panel.querySelectorAll('.sfig b').forEach(countUp);
      // turntable.js puts this build's model on the turntable.
      hall.dispatchEvent(new CustomEvent('show', { detail: slug }));
      // Keep the build on show in view in the strip (on phones it scrolls sideways), never the page.
      var strip = hall.querySelector('.strip'), cur = tiles.find(function (a) { return slugOf(a) === slug; });
      if (cur && strip.scrollWidth > strip.clientWidth) strip.scrollLeft = cur.parentNode.offsetLeft - strip.offsetLeft - 16;
      if (focus) panel.querySelector('h2').focus();
      return true;
    };
    // Each walkthrough's controls go on a strip along its top, where the model on the turntable never
    // covers them: play, the time and a scrubber, captions, sound, full screen. Without this the
    // video keeps the browser's own controls, along the bottom.
    var icon = function (d) { return '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="' + d + '"/></svg>'; };
    var P = {
      play: 'M8 5v14l11-7z', pause: 'M6 5h4v14H6zM14 5h4v14h-4z',
      loud: 'M4 9h4l5-4v14l-5-4H4zM15.5 8.5a5 5 0 0 1 0 7l-1.4-1.4a3 3 0 0 0 0-4.2z',
      mute: 'M4 9h4l5-4v14l-5-4H4zM16 9.4l1.4-1.4 2.1 2.1 2.1-2.1 1.4 1.4-2.1 2.1 2.1 2.1-1.4 1.4-2.1-2.1-2.1 2.1-1.4-1.4 2.1-2.1z',
      full: 'M4 4h6v2H6v4H4zM14 4h6v6h-2V6h-4zM4 14h2v4h4v2H4zM18 14h2v6h-6v-2h4z'
    };
    var clock = function (s) { s = Math.floor(s || 0); return Math.floor(s / 60) + ':' + ('0' + (s % 60)).slice(-2); };
    hall.querySelectorAll('.reel video').forEach(function (v) {
      var bar = document.createElement('div'), track = v.textTracks && v.textTracks[0];
      bar.className = 'vbar';
      var button = function (label, html) {
        var b = document.createElement('button');
        b.type = 'button'; b.setAttribute('aria-label', label); b.innerHTML = html;
        bar.appendChild(b);
        return b;
      };
      var play = button('Play', icon(P.play)), time = bar.appendChild(document.createElement('span'));
      var seek = bar.appendChild(document.createElement('input'));
      seek.type = 'range'; seek.min = 0; seek.max = 1000; seek.value = 0; seek.setAttribute('aria-label', 'Seek');
      var cc = track ? button('Captions', 'CC') : null, sound = button('Sound', ''), full = button('Full screen', icon(P.full));
      var sync = function () {
        play.innerHTML = icon(v.paused ? P.play : P.pause);
        play.setAttribute('aria-label', v.paused ? 'Play' : 'Pause');
        sound.innerHTML = icon(v.muted ? P.mute : P.loud);
        sound.setAttribute('aria-pressed', String(!v.muted));
        if (cc) cc.setAttribute('aria-pressed', String(track.mode === 'showing'));
        time.textContent = clock(v.currentTime) + (v.duration ? ' / ' + clock(v.duration) : '');
        if (v.duration && document.activeElement !== seek) seek.value = Math.round((v.currentTime / v.duration) * 1000);
      };
      play.addEventListener('click', function () { if (v.paused) v.play(); else v.pause(); });
      seek.addEventListener('input', function () { if (v.duration) v.currentTime = (seek.value / 1000) * v.duration; });
      if (cc) cc.addEventListener('click', function () { track.mode = track.mode === 'showing' ? 'hidden' : 'showing'; sync(); });
      sound.addEventListener('click', function () { v.muted = !v.muted; });
      full.addEventListener('click', function () { if (v.requestFullscreen) v.requestFullscreen(); else if (v.webkitEnterFullscreen) v.webkitEnterFullscreen(); });
      // Full screen has no bar of ours, so the browser's controls come back there.
      document.addEventListener('fullscreenchange', function () { v.controls = document.fullscreenElement === v; });
      ['play', 'pause', 'timeupdate', 'loadedmetadata', 'durationchange', 'volumechange'].forEach(function (e) { v.addEventListener(e, sync); });
      v.controls = false;
      v.parentNode.insertBefore(bar, v);
      sync();
    });
    // The big play button over each walkthrough.
    hall.querySelectorAll('.reel').forEach(function (r) {
      var v = r.querySelector('video'), b = r.querySelector('.play');
      if (!v || !b) return;
      b.hidden = false;
      b.addEventListener('click', function () { v.play(); });
      v.addEventListener('play', function () { b.hidden = true; });
      v.addEventListener('pause', function () { b.hidden = false; });
      v.addEventListener('ended', function () { b.hidden = false; });
    });
    var pick = function (a, focus) {
      if (!show(slugOf(a), focus)) return false;
      history.replaceState(null, '', '#' + slugOf(a));
      return true;
    };
    tiles.forEach(function (a) {
      a.addEventListener('click', function (e) {
        if (e.button || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
        // Enter on the build already on show opens it, as a game confirms a car already picked.
        if (e.detail === 0 && a.getAttribute('aria-current') === 'true') return;
        if (pick(a, true)) e.preventDefault();
      });
    });
    // ◀ ▶ step through the builds anywhere in the hall. ▲ ▼ do too while the list of builds runs
    // down the side (wide screens): in it, or with nothing else focused.
    var strip = hall.querySelector('.strip');
    document.addEventListener('keydown', function (e) {
      var across = e.key === 'ArrowLeft' || e.key === 'ArrowRight', down = e.key === 'ArrowUp' || e.key === 'ArrowDown';
      if ((!across && !down) || e.altKey || e.ctrlKey || e.metaKey) return;
      if (e.target.closest('input,textarea,select,video,[contenteditable],[popover]')) return;
      if (down && !(getComputedStyle(strip).position === 'absolute' && (strip.contains(e.target) || e.target === document.body))) return;
      var vis = tiles.filter(shown);
      var i = vis.findIndex(function (a) { return a.getAttribute('aria-current') === 'true'; });
      var next = vis[(i + (e.key === 'ArrowRight' || e.key === 'ArrowDown' ? 1 : -1) + vis.length) % vis.length];
      if (next && pick(next, false)) { e.preventDefault(); next.focus(); }
    });
    // On a phone the desk's first tap shows its note; the second opens Contact.
    var spot = hall.querySelector('.desk-spot');
    if (spot) {
      spot.querySelector('.hall-desk').addEventListener('click', function (e) {
        if (!matchMedia('(hover: none)').matches || spot.classList.contains('open')) return;
        e.preventDefault();
        spot.classList.add('open');
      });
      document.addEventListener('click', function (e) { if (!spot.contains(e.target)) spot.classList.remove('open'); });
    }
    // The sort menu: Featured (the hall's own order), Program order, or one class of build.
    // The arrow keys follow whatever order the list is in.
    var sort = hall.querySelector('.sort'), by = sort.querySelector('select');
    var featured = tiles.map(function (a) { return a.parentNode; });
    sort.hidden = false;
    by.addEventListener('change', function () {
      var v = by.value, items = featured.slice();
      if (v === 'program') items.sort(function (a, b) { return a.dataset.program - b.dataset.program; });
      items.forEach(function (li) {
        li.hidden = (v === 'integrity' || v === 'team') && li.dataset.group !== v;
        strip.appendChild(li);
      });
      tiles = items.map(function (li) { return li.querySelector('.tile'); });
      // If the build on show was sorted out of view, put the first one left on show.
      if (!tiles.some(function (a) { return shown(a) && a.getAttribute('aria-current') === 'true'; })) {
        var first = tiles.filter(shown)[0];
        if (first) pick(first, false);
      }
    });
    if (!show(location.hash.slice(1), false)) {
      var on = hall.querySelector('.show:not([hidden])');
      if (on) show(on.id.slice(5), false);
    }
  }
  // A time in a transcript plays the walkthrough from there, in the hall or on a build's page.
  document.addEventListener('click', function (e) {
    var t = e.target.closest && e.target.closest('.ts');
    if (!t) return;
    var v = (t.closest('.show, .case') || document).querySelector('video');
    if (!v) return;
    v.currentTime = +t.dataset.t;
    v.play();
  });
  // Experience: the stack ziggurat turns slowly clockwise, like the plate models. Drag it, or use
  // the arrow keys, to turn it by hand; tabbing to a tool turns that side to face you. Each side
  // is shaded by how squarely it faces the lamp. Without this it stands still at an angle and
  // the tool cards open all the same. Below 75rem it is unfolded flat, and none of this runs.
  var zig = document.querySelector('.zig');
  if (zig) {
    var model = zig.querySelector('.model'), wide = matchMedia('(min-width: 75rem)');
    var ry = 38, target = null, held = false, drag = null, moved = false, last = 0;
    var paint = function () {
      model.style.setProperty('--ry', ry.toFixed(2) + 'deg');
      for (var k = 0; k < 4; k++) {
        var c = Math.cos((ry + k * 90 + 30) * Math.PI / 180);
        model.style.setProperty('--sh' + k, (0.34 * (1 - Math.max(0, c))).toFixed(3));
      }
    };
    var tick = function (t) {
      if (wide.matches) {
        var dt = last ? Math.min(t - last, 100) : 0;
        if (target !== null) {
          var d = ((target - ry) % 360 + 540) % 360 - 180;
          if (Math.abs(d) < 0.3) { ry = target; target = null; } else ry += d * 0.12;
          paint();
        } else if (!held && drag === null && !calm()) { ry -= dt * 0.009; paint(); }  // one turn in 40s
      }
      last = t;
      requestAnimationFrame(tick);
    };
    zig.addEventListener('pointerenter', function () { held = true; });
    zig.addEventListener('pointerleave', function () { held = false; });
    zig.addEventListener('focusin', function (e) {
      held = true;
      var face = e.target.closest('.face');
      if (face && wide.matches) target = -90 * +face.className.match(/f(\d)/)[1] - 18;
    });
    zig.addEventListener('focusout', function () { held = false; });
    zig.addEventListener('pointerdown', function (e) {
      if (!wide.matches || e.button) return;
      drag = e.clientX; moved = false;
    });
    window.addEventListener('pointermove', function (e) {
      if (drag === null) return;
      var dx = e.clientX - drag;
      if (!moved && Math.abs(dx) > 3) { moved = true; zig.classList.add('dragging'); }
      if (moved) { ry += dx * 0.4; drag = e.clientX; target = null; paint(); }
    });
    window.addEventListener('pointerup', function () { drag = null; zig.classList.remove('dragging'); });
    // A drag that ends over a tool was a turn, not a press.
    zig.addEventListener('click', function (e) {
      if (moved) { e.preventDefault(); e.stopPropagation(); moved = false; }
    }, true);
    zig.addEventListener('keydown', function (e) {
      if (e.target !== zig || !wide.matches || (e.key !== 'ArrowLeft' && e.key !== 'ArrowRight')) return;
      e.preventDefault();
      target = ry + (e.key === 'ArrowRight' ? 30 : -30);
    });
    paint();
    requestAnimationFrame(tick);
  }

  // Build recordings play while on screen, unless the reader prefers less motion.
  // Without this script they are ordinary videos with controls.
  var still = calm();
  var vids = document.querySelectorAll('video[data-autoplay]');
  if (!still && 'IntersectionObserver' in window && vids.length) {
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (e.isIntersecting) { e.target.preload = 'auto'; e.target.play().catch(function () {}); }
        else { e.target.pause(); }
      });
    }, { threshold: 0.4 });
    vids.forEach(function (v) { io.observe(v); });
  }

  // Phones: fold role, team and stack so the video comes first. Without JS they stay open.
  var more = document.querySelectorAll('details.spec-more');
  if (more.length) {
    var narrow = matchMedia('(max-width: 52rem)');
    var fold = function () { more.forEach(function (d) { d.open = !narrow.matches; }); };
    fold();
    narrow.addEventListener('change', fold);
  }

  // Contact form: send in place and say what happened. Without JS it posts normally.
  var form = document.getElementById('contactForm');
  if (form) {
    var status = document.getElementById('formStatus');
    form.addEventListener('submit', function (e) {
      e.preventDefault();
      var send = form.querySelector('button[type=submit]');
      send.disabled = true;
      status.className = 'form-status';
      status.textContent = 'Sending…';
      fetch(form.action, { method: 'POST', body: new FormData(form), headers: { Accept: 'application/json' } })
        .then(function (r) { return r.json(); })
        .then(function (d) {
          if (d.success) { status.textContent = 'Sent. I will reply by email.'; form.reset(); }
          else { status.className = 'form-status err'; status.textContent = (d.message || 'That did not send.') + ' Email ness.aisling@gmail.com instead.'; }
        })
        .catch(function () {
          status.className = 'form-status err';
          status.textContent = 'That did not send. Email ness.aisling@gmail.com instead.';
        })
        .then(function () { send.disabled = false; });
    });
  }
})();
