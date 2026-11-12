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
  if (hall) {
    var tiles = [].slice.call(hall.querySelectorAll('.tile'));
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
      // turntable.js puts this build's model on the turntable.
      hall.dispatchEvent(new CustomEvent('show', { detail: slug }));
      // Keep the build on show in view in the strip (on phones it scrolls sideways), never the page.
      var strip = hall.querySelector('.strip'), cur = tiles.find(function (a) { return slugOf(a) === slug; });
      if (cur && strip.scrollWidth > strip.clientWidth) strip.scrollLeft = cur.parentNode.offsetLeft - strip.offsetLeft - 16;
      if (focus) panel.querySelector('h2').focus();
      return true;
    };
    // The big play button over each walkthrough; the video's own controls remain.
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
        if (pick(a, true)) e.preventDefault();
      });
    });
    document.addEventListener('keydown', function (e) {
      if ((e.key !== 'ArrowLeft' && e.key !== 'ArrowRight') || e.altKey || e.ctrlKey || e.metaKey) return;
      if (e.target.closest('input,textarea,select,video,[contenteditable],[popover]')) return;
      var vis = tiles.filter(shown);
      var i = vis.findIndex(function (a) { return a.getAttribute('aria-current') === 'true'; });
      var next = vis[(i + (e.key === 'ArrowRight' ? 1 : -1) + vis.length) % vis.length];
      if (next && pick(next, false)) { e.preventDefault(); next.focus(); }
    });
    var classes = hall.querySelector('.classes');
    classes.hidden = false;
    classes.addEventListener('click', function (e) {
      var b = e.target.closest('button');
      if (!b) return;
      classes.querySelectorAll('button').forEach(function (x) { x.setAttribute('aria-pressed', x === b ? 'true' : 'false'); });
      var g = b.dataset.group;
      tiles.forEach(function (a) { a.parentNode.hidden = g !== 'all' && a.parentNode.dataset.group !== g; });
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
