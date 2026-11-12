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

  // Reading settings (text size, spacing, contrast, day or evening, motion, finish), from
  // the brand kit. Without the kit's script the slot stays empty and hidden.
  var slot = document.querySelector('.settings-slot');
  if (slot && window.NessBrand) NessBrand.settings.mount(slot);

  // Projects: pressing a bay raises that build onto the lift. Without this, each bay is a
  // plain link to its build; a modified click (new tab, new window) still opens the link.
  var lift = document.querySelector('.lift');
  if (lift) {
    var bays = document.querySelectorAll('.wall .nb-plate');
    var raise = function (slug, focus) {
      var panel = slug && document.getElementById('lift-' + slug);
      if (!panel || !panel.classList.contains('raised')) return false;
      lift.querySelectorAll('.raised').forEach(function (p) { p.hidden = p !== panel; });
      bays.forEach(function (a) { a.setAttribute('aria-current', a.dataset.slug === slug ? 'true' : 'false'); });
      if (focus) panel.querySelector('h2').focus();
      return true;
    };
    bays.forEach(function (a) {
      a.dataset.slug = (a.getAttribute('href').match(/work\/([^/]+)\/$/) || [])[1];
      a.setAttribute('aria-controls', 'lift-' + a.dataset.slug);
      a.addEventListener('click', function (e) {
        if (e.button || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
        if (raise(a.dataset.slug, true)) { e.preventDefault(); history.replaceState(null, '', '#' + a.dataset.slug); }
      });
    });
    var up = lift.querySelector('.raised:not([hidden])');
    if (!raise(location.hash.slice(1), false) && up) raise(up.id.slice(5), false);
  }

  // Experience: the stack ziggurat turns slowly clockwise, like the plate models. Drag it, or use
  // the arrow keys, to turn it by hand; tabbing to a tool turns that side to face you. Each side
  // is shaded by how squarely it faces the lamp. Without this it stands still at an angle and
  // the tool cards open all the same. Below 75rem it is unfolded flat, and none of this runs.
  var zig = document.querySelector('.zig');
  if (zig) {
    var model = zig.querySelector('.model'), wide = matchMedia('(min-width: 75rem)');
    var calm = function () {
      var m = document.documentElement.dataset.motion;
      return m === 'reduce' || (m !== 'full' && matchMedia('(prefers-reduced-motion: reduce)').matches);
    };
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
  var still = matchMedia('(prefers-reduced-motion: reduce)').matches;
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
